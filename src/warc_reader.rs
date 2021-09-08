use crate::parser;
use crate::{BufferedBody, Error, RawRecordHeader, Record, StreamingBody};

use std::convert::TryInto;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg(feature = "gzip")]
use libflate::gzip::MultiDecoder as GzipReader;

const KB: usize = 1_024;
const MB: usize = 1_048_576;

/// A reader which iteratively parses WARC records from a stream.
pub struct WarcReader<R> {
    reader: R,
}

impl<R: BufRead> WarcReader<R> {
    /// Create a new reader.
    pub fn new(r: R) -> Self {
        WarcReader { reader: r }
    }

    /// Create an iterator over all of the raw records read.
    ///
    /// This only does well-formedness checks on the headers. See `RawRecordHeader` for more
    /// information.
    pub fn iter_raw_records(self) -> RawRecordIter<R> {
        RawRecordIter::new(self.reader)
    }

    /// Create an iterator over all of the records read.
    ///
    /// This will fully build each record and check it for semantic correctness. See the `Record`
    /// type for more information.
    pub fn iter_records(self) -> RecordIter<R> {
        RecordIter::new(self.reader)
    }

    /// Create a streaming iterator over all of the records read.
    ///
    /// This will build each record header, and allow the caller to decide whether to read
    /// the body or not.
    pub fn stream_records(&mut self) -> StreamingIter<'_, R> {
        StreamingIter::new(&mut self.reader)
    }
}

impl WarcReader<BufReader<fs::File>> {
    /// Create a new reader which reads from file.
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
impl WarcReader<BufReader<GzipReader<BufReader<std::fs::File>>>> {
    /// Create a new reader which reads from a compressed file.
    ///
    /// Only GZIP compression is currently supported.
    pub fn from_path_gzip<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::File::open(&path)?;

        let gzip_stream = GzipReader::new(BufReader::with_capacity(1 * MB, file))?;
        Ok(WarcReader::new(BufReader::new(gzip_stream)))
    }
}

/// An iterator of raw records streamed from a reader. See `RawRecord` for more information.
pub struct RawRecordIter<R> {
    reader: R,
}

impl<R: BufRead> RawRecordIter<R> {
    pub(crate) fn new(reader: R) -> RawRecordIter<R> {
        RawRecordIter { reader }
    }
}

impl<R: BufRead> Iterator for RawRecordIter<R> {
    type Item = Result<(RawRecordHeader, Vec<u8>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut header_buffer: Vec<u8> = Vec::with_capacity(64 * KB);
        let mut found_headers = false;
        while !found_headers {
            let bytes_read = match self.reader.read_until(b'\n', &mut header_buffer) {
                Err(io) => return Some(Err(Error::ReadData(io))),
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
            Err(e) => return Some(Err(Error::ParseHeaders(e.to_owned()))),
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
                Err(io) => return Some(Err(Error::ReadData(io))),
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

        let headers = RawRecordHeader {
            version: version_ref.to_owned(),
            headers: headers_ref
                .into_iter()
                .map(|(token, value)| (token.into(), value.to_owned()))
                .collect(),
        };
        let body = body_ref.to_owned();
        Some(Ok((headers, body)))
    }
}

/// An iterator which returns the records read by a reader.
pub struct RecordIter<R> {
    reader: R,
}

impl<R: BufRead> RecordIter<R> {
    pub(crate) fn new(reader: R) -> RecordIter<R> {
        RecordIter { reader }
    }
}

impl<R: BufRead> Iterator for RecordIter<R> {
    type Item = Result<Record<BufferedBody>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut header_buffer: Vec<u8> = Vec::with_capacity(64 * KB);
        let mut found_headers = false;
        while !found_headers {
            let bytes_read = match self.reader.read_until(b'\n', &mut header_buffer) {
                Err(io) => return Some(Err(Error::ReadData(io))),
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
            Err(e) => return Some(Err(Error::ParseHeaders(e.to_owned()))),
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
                Err(io) => return Some(Err(Error::ReadData(io))),
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

        let headers = RawRecordHeader {
            version: version_ref.to_owned(),
            headers: headers_ref
                .into_iter()
                .map(|(token, value)| (token.into(), value.to_owned()))
                .collect(),
        };
        let body = body_ref.to_owned();
        match headers.try_into() {
            Ok(b) => {
                let buffered: Record<_> = b;
                Some(Ok(buffered.add_body(body)))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

/// An iterator-like type to "stream" records from a reader.
///
/// This API returns records which use the `StreamingBody` type. This allows reading record headers
/// and metadata without reading the bodies. Bodies can be read or skipped as desired.
///
/// This is streaming iterator is particularly useful for streams of records which are indefinite
/// or contain and records of unknown size.
pub struct StreamingIter<'r, R> {
    reader: &'r mut R,
    current_item_size: u64,
    first_record: bool,
}

impl<R: BufRead> StreamingIter<'_, R> {
    pub(crate) fn new(reader: &mut R) -> StreamingIter<'_, R> {
        StreamingIter {
            reader,
            current_item_size: 0,
            first_record: true,
        }
    }

    fn skip_body(&mut self) -> Result<(), Error> {
        let mut read_buffer = [0u8; 1 * MB];
        let maximum_read_range = self.current_item_size;
        let mut body_bytes_left = maximum_read_range;
        while body_bytes_left > 0 {
            let read_size = std::cmp::min(body_bytes_left, read_buffer.len() as u64) as usize;
            let bytes_read = match self.reader.read(&mut read_buffer[..read_size]) {
                Err(io) => return Err(Error::ReadData(io)),
                Ok(len) => len as u64,
            };
            if bytes_read == 0 {
                return Err(Error::UnexpectedEOB);
            }
            body_bytes_left -= bytes_read;
        }

        let mut crlfs = [0; 4];

        match self.reader.read(&mut crlfs) {
            Ok(4) => {}
            Ok(_) => return Err(Error::UnexpectedEOB),
            Err(io) => return Err(Error::ReadData(io)),
        }

        if &crlfs == b"\x0d\x0a\x0d\x0a" {
            Ok(())
        } else {
            let synthetic_err: nom::Err<(Vec<u8>, nom::error::ErrorKind)> =
                nom::Err::Failure((vec![0x0d, 0x0a, 0x0d, 0x0a], nom::error::ErrorKind::Tag));
            Err(Error::ParseHeaders(synthetic_err))
        }
    }

    /// Advance the stream to the next item.
    ///
    /// Returns one of the following:
    /// * Some(Ok(r))` is the next record read from the stream.
    /// * `Some(Err)` indicates there was a read error.
    /// * `None` indicates no more records are returned.
    pub fn next_item(&mut self) -> Option<Result<Record<StreamingBody<'_, R>>, Error>> {
        if self.first_record {
            self.first_record = false;
        } else if let Err(e) = self.skip_body() {
            return Some(Err(e));
        }

        let mut header_buffer: Vec<u8> = Vec::with_capacity(64 * KB);
        let mut found_headers = false;
        while !found_headers {
            let bytes_read = match self.reader.read_until(b'\n', &mut header_buffer) {
                Err(io) => return Some(Err(Error::ReadData(io))),
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
            Err(e) => return Some(Err(Error::ParseHeaders(e.to_owned()))),
            Ok(parsed) => parsed.1,
        };
        let version_ref = headers_parsed.0;
        let headers_ref = headers_parsed.1;
        self.current_item_size = headers_parsed.2 as u64;

        let headers = RawRecordHeader {
            version: version_ref.to_owned(),
            headers: headers_ref
                .into_iter()
                .map(|(token, value)| (token.into(), value.to_owned()))
                .collect(),
        };
        match headers.try_into() {
            Ok(b) => {
                let record: Record<_> = b;
                let fixed_stream_result = record
                    .add_fixed_stream(self.reader, &mut self.current_item_size)
                    .map_err(Error::ReadData);
                Some(fixed_stream_result)
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod iter_raw_tests {
    use std::collections::HashMap;
    use std::io::{BufReader, Cursor};
    use std::iter::FromIterator;

    use crate::{WarcHeader, WarcReader};
    macro_rules! create_reader {
        ($raw:expr) => {{
            BufReader::new(Cursor::new($raw.get(..).unwrap()))
        }};
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
        let expected_headers: HashMap<WarcHeader, Vec<u8>> = HashMap::from_iter(
            vec![
                (WarcHeader::WarcType, b"dunno".to_vec()),
                (WarcHeader::ContentLength, b"5".to_vec()),
                (
                    WarcHeader::RecordID,
                    b"<urn:test:basic-record:record-0>".to_vec(),
                ),
                (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
            ]
            .into_iter(),
        );
        let expected_body: &[u8] = b"12345";

        let mut reader = WarcReader::new(create_reader!(raw)).iter_raw_records();
        let (headers, body) = reader.next().unwrap().unwrap();
        assert_eq!(headers.version, expected_version);
        assert_eq!(headers.as_ref(), &expected_headers);
        assert_eq!(body, expected_body);
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

        let mut reader = WarcReader::new(create_reader!(raw)).iter_raw_records();
        {
            let expected_version = "1.0";
            let expected_headers: HashMap<WarcHeader, Vec<u8>> = HashMap::from_iter(
                vec![
                    (WarcHeader::WarcType, b"dunno".to_vec()),
                    (WarcHeader::ContentLength, b"5".to_vec()),
                    (
                        WarcHeader::RecordID,
                        b"<urn:test:two-records:record-0>".to_vec(),
                    ),
                    (WarcHeader::Date, b"2020-07-08T02:52:55Z".to_vec()),
                ]
                .into_iter(),
            );
            let expected_body: &[u8] = b"12345";

            let (headers, body) = reader.next().unwrap().unwrap();
            assert_eq!(headers.version, expected_version);
            assert_eq!(headers.as_ref(), &expected_headers);
            assert_eq!(body, expected_body);
        }

        {
            let expected_version = "1.0";
            let expected_headers: HashMap<WarcHeader, Vec<u8>> = HashMap::from_iter(
                vec![
                    (WarcHeader::WarcType, b"another".to_vec()),
                    (WarcHeader::ContentLength, b"6".to_vec()),
                    (
                        WarcHeader::RecordID,
                        b"<urn:test:two-records:record-1>".to_vec(),
                    ),
                    (WarcHeader::Date, b"2020-07-08T02:52:56Z".to_vec()),
                ]
                .into_iter(),
            );
            let expected_body: &[u8] = b"123456";

            let (headers, body) = reader.next().unwrap().unwrap();
            assert_eq!(headers.version, expected_version);
            assert_eq!(headers.as_ref(), &expected_headers);
            assert_eq!(body, expected_body);
        }
    }
}

#[cfg(test)]
mod next_item_tests {
    use std::collections::HashMap;
    use std::io::{BufReader, Cursor};
    use std::iter::FromIterator;

    use crate::{WarcHeader, WarcReader};

    macro_rules! create_reader {
        ($raw:expr) => {{
            BufReader::new(Cursor::new($raw.get(..).unwrap()))
        }};
    }

    #[test]
    fn first_item() {
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

        let mut reader = WarcReader::new(create_reader!(raw));
        let mut stream_iter = reader.stream_records();
        let record = stream_iter
            .next_item()
            .unwrap()
            .unwrap()
            .into_buffered()
            .unwrap();
        assert_eq!(record.warc_version(), "1.0");
        assert_eq!(record.content_length(), 5);
        assert_eq!(record.warc_id(), "<urn:test:basic-record:record-0>");
        assert_eq!(record.body(), b"12345");
    }

    #[test]
    fn both_items() {
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
        let mut stream_iter = reader.stream_records();

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 5);
            assert_eq!(record.warc_id(), "<urn:test:two-records:record-0>");
            assert_eq!(record.body(), b"12345");
        }

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 6);
            assert_eq!(record.warc_id(), "<urn:test:two-records:record-1>");
            assert_eq!(record.body(), b"123456");
        }
    }

    #[test]
    fn only_second_item() {
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
        let mut stream_iter = reader.stream_records();

        let _skipped = stream_iter.next_item().unwrap().unwrap();

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 6);
            assert_eq!(record.warc_id(), "<urn:test:two-records:record-1>");
            assert_eq!(record.body(), b"123456");
        }
    }

    #[test]
    fn triple_items() {
        let raw = b"\
            WARC/1.0\r\n\
            Warc-Type: dunno\r\n\
            Content-Length: 5\r\n\
            WARC-Record-Id: <urn:test:three-records:record-0>\r\n\
            WARC-Date: 2020-07-08T02:52:55Z\r\n\
            \r\n\
            12345\r\n\
            \r\n\
            WARC/1.0\r\n\
            Warc-Type: another\r\n\
            WARC-Record-Id: <urn:test:three-records:record-1>\r\n\
            WARC-Date: 2020-07-08T02:52:56Z\r\n\
            Content-Length: 6\r\n\
            \r\n\
            123456\r\n\
            \r\n\
            WARC/1.0\r\n\
            Warc-Type: yet another\r\n\
            WARC-Record-Id: <urn:test:three-records:record-2>\r\n\
            WARC-Date: 2020-07-08T02:52:56Z\r\n\
            Content-Length: 8\r\n\
            \r\n\
            12345678\r\n\
            \r\n\
        ";

        let mut reader = WarcReader::new(create_reader!(raw));
        let mut stream_iter = reader.stream_records();

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 5);
            assert_eq!(record.warc_id(), "<urn:test:three-records:record-0>");
            assert_eq!(record.body(), b"12345");
        }

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 6);
            assert_eq!(record.warc_id(), "<urn:test:three-records:record-1>");
            assert_eq!(record.body(), b"123456");
        }

        {
            let record = stream_iter
                .next_item()
                .unwrap()
                .unwrap()
                .into_buffered()
                .unwrap();
            assert_eq!(record.warc_version(), "1.0");
            assert_eq!(record.content_length(), 8);
            assert_eq!(record.warc_id(), "<urn:test:three-records:record-2>");
            assert_eq!(record.body(), b"12345678");
        }
    }
}
