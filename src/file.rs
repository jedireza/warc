use crate::parser;
use crate::{Error, WarcRecord, WarcRecordRef};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

const KB: usize = 1_024;
const MB: usize = 1_048_576;

pub struct WarcFile {
    reader: BufReader<File>,
}

impl WarcFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let reader = BufReader::with_capacity(1 * MB, file);

        Ok(WarcFile { reader: reader })
    }

    pub fn write(&mut self, record: &WarcRecord) -> io::Result<usize> {
        let mut bytes_written = 0;

        let mut file = self.reader.get_ref();
        file.seek(SeekFrom::End(0))?;

        bytes_written += file.write(&[87, 65, 82, 67, 47])?;
        bytes_written += file.write(record.version.as_bytes())?;
        bytes_written += file.write(&[13, 10])?;

        for header in record.headers.iter() {
            bytes_written += file.write(header.token.as_bytes())?;
            bytes_written += file.write(&[58, 32])?;
            bytes_written += file.write(&header.value)?;
            bytes_written += file.write(&[13, 10])?;
        }
        bytes_written += file.write(&[13, 10])?;

        bytes_written += file.write(&record.body)?;
        bytes_written += file.write(&[13, 10])?;
        bytes_written += file.write(&[13, 10])?;

        Ok(bytes_written)
    }
}

impl Iterator for WarcFile {
    type Item = Result<WarcRecord, Error>;

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
                let buffer_len = header_buffer.len();
                if &header_buffer[buffer_len - 2..] == b"\r\n" {
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

        let record_ref = WarcRecordRef {
            version: version_ref,
            headers: headers_ref,
            body: body_ref,
        };

        return Some(Ok(record_ref.into()));
    }
}
