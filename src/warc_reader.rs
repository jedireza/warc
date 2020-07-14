use crate::parser;
use crate::{Error, Record};

use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "gzip")]
use libflate::gzip::Decoder as GzipReader;

const KB: usize = 1_024;
const MB: usize = 1_048_576;

pub struct WarcReader<R> {
    reader: R,
}

impl<R: BufRead> WarcReader<R> {
    pub fn new(r: R) -> Self {
        WarcReader { reader: r }
    }
}

impl WarcReader<BufReader<fs::File>> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let reader = BufReader::with_capacity(1 * MB, file);

        Ok(WarcReader::new(reader))
    }
}

#[cfg(feature = "gzip")]
impl WarcReader<BufReader<GzipReader<std::fs::File>>> {
    pub fn from_path_gzip<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let gzip_stream = GzipReader::new(file)?;
        let reader = BufReader::with_capacity(1 * MB, gzip_stream);

        Ok(WarcReader::new(reader))
    }
}

impl<R: BufRead> Iterator for WarcReader<R> {
    type Item = Result<Record, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut header_buffer: Vec<u8> = Vec::with_capacity(64 * KB);
        let mut found_headers = false;
        while !found_headers {
            let bytes_read = match self.reader.read_until(b'\n', &mut header_buffer) {
                Err(_) => return Some(Err(Error::ReadData)),
                Ok(len) => len,
            };

            if bytes_read == 0 {
                return None;
            }

            if bytes_read == 2 {
                let last_two_chars = header_buffer.len() - 2;
                if &header_buffer[last_two_chars..] == b"\r\n" {
                    found_headers = true;
                }
            }
        }

        let headers_parsed = match parser::headers(&header_buffer) {
            Err(_) => return Some(Err(Error::ParseHeaders)),
            Ok(parsed) => parsed.1,
        };
        let version_ref = headers_parsed.0;
        let headers_ref = headers_parsed.1;
        let expected_body_len = headers_parsed.2;

        let mut body_buffer: Vec<u8> = Vec::with_capacity(1 * MB);
        let mut found_body = expected_body_len == 0;
        let mut body_bytes_read = 0;
        let maximum_read_range = expected_body_len + 4;
        while !found_body {
            let bytes_read = match self.reader.read_until(b'\n', &mut body_buffer) {
                Err(_) => return Some(Err(Error::ReadData)),
                Ok(len) => len,
            };

            body_bytes_read += bytes_read;

            // we expect 4 characters (\r\n\r\n) after the body
            if bytes_read == 2 && body_bytes_read == maximum_read_range {
                found_body = true;
            }

            if bytes_read == 0 {
                return Some(Err(Error::UnexpectedEOB));
            }

            if body_bytes_read > maximum_read_range {
                return Some(Err(Error::ReadOverflow));
            }
        }

        let body_ref = &body_buffer[..expected_body_len];

        let record = Record {
            version: version_ref.to_owned(),
            headers: headers_ref
                .into_iter()
                .map(|(token, value)| (token.into(), value.to_owned()))
                .collect(),
            body: body_ref.to_owned(),
        };
        return Some(Ok(record));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::{BufReader, Cursor};
    use std::iter::FromIterator;

    use crate::{WarcReader, header::WarcHeader};
    macro_rules! create_reader {
        ($raw:expr) => { {
            BufReader::new(Cursor::new($raw.get(..).unwrap()))
        } }
    }

    #[test]
    fn basic_record() {
        let raw = b"\
            WARC/1.0\r\n\
            Warc-Type: dunno\r\n\
            Content-Length: 5\r\n\
            WARC-Record-Id: <urn:test:basic-record:record-0>\r\n\
            WARC-Date: 2020-07-08T02:52:55Z\r\n\
            \r\n\
            12345\r\n\
            \r\n\
        ";

        let expected_version = "1.0";
        let expected_headers: HashMap<WarcHeader, Vec<u8>> =
            HashMap::from_iter(vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (WarcHeader::ContentLength, b"5".to_vec()),
                (WarcHeader::RecordID, b"<urn:test:basic-record:record-0>".to_vec()),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ].into_iter());
        let expected_body: &[u8] = b"12345";

        let mut reader = WarcReader::new(create_reader!(raw));
        let record = reader.next().unwrap().unwrap();
        assert_eq!(record.version, expected_version);
        assert_eq!(record.headers, expected_headers);
        assert_eq!(record.body, expected_body);
    }

    #[test]
    fn two_records() {
        let raw = b"\
            WARC/1.0\r\n\
            Warc-Type: dunno\r\n\
            Content-Length: 5\r\n\
            WARC-Record-Id: <urn:test:two-records:record-0>\r\n\
            WARC-Date: 2020-07-08T02:52:55Z\r\n\
            \r\n\
            12345\r\n\
            \r\n\
            WARC/1.0\r\n\
            Warc-Type: another\r\n\
            WARC-Record-Id: <urn:test:two-records:record-1>\r\n\
            WARC-Date: 2020-07-08T02:52:56Z\r\n\
            Content-Length: 6\r\n\
            \r\n\
            123456\r\n\
            \r\n\
        ";

        let mut reader = WarcReader::new(create_reader!(raw));
        {
            let expected_version = "1.0";
            let expected_headers: HashMap<WarcHeader, Vec<u8>> =
                HashMap::from_iter(vec![
                    (WarcHeader::WarcType, b"dunno".to_vec()),
                    (WarcHeader::ContentLength, b"5".to_vec()),
                    (WarcHeader::RecordID, b"<urn:test:two-records:record-0>".to_vec()),
                    (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
                ].into_iter());
            let expected_body: &[u8] = b"12345";

            let record = reader.next().unwrap().unwrap();
            assert_eq!(record.version, expected_version);
            assert_eq!(record.headers, expected_headers);
            assert_eq!(record.body, expected_body);
        }

        {
            let expected_version = "1.0";
            let expected_headers: HashMap<WarcHeader, Vec<u8>> =
                HashMap::from_iter(vec![
                    (WarcHeader::WarcType, b"another".to_vec()),
                    (WarcHeader::ContentLength, b"6".to_vec()),
                    (WarcHeader::RecordID, b"<urn:test:two-records:record-1>".to_vec()),
                    (WarcHeader::Date, b"2020-07-08T02:52:56Z".to_vec()),
                ].into_iter());
            let expected_body: &[u8] = b"123456";

            let record = reader.next().unwrap().unwrap();
            assert_eq!(record.version, expected_version);
            assert_eq!(record.headers, expected_headers);
            assert_eq!(record.body, expected_body);
        }
    }
}
