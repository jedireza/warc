use crate::parser;
use crate::{Error, Record};
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

const KB: usize = 1_024;
const MB: usize = 1_048_576;

pub struct File {
    reader: BufReader<fs::File>,
}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let reader = BufReader::with_capacity(1 * MB, file);

        Ok(File { reader: reader })
    }

    pub fn write(&mut self, record: &Record) -> io::Result<usize> {
        let mut bytes_written = 0;

        let mut file = self.reader.get_ref();
        file.seek(SeekFrom::End(0))?;

        bytes_written += file.write(&[87, 65, 82, 67, 47])?;
        bytes_written += file.write(record.version.as_bytes())?;
        bytes_written += file.write(&[13, 10])?;

        for (token, value) in record.headers.iter() {
            bytes_written += file.write(token.as_bytes())?;
            bytes_written += file.write(&[58, 32])?;
            bytes_written += file.write(&value)?;
            bytes_written += file.write(&[13, 10])?;
        }
        bytes_written += file.write(&[13, 10])?;

        bytes_written += file.write(&record.body)?;
        bytes_written += file.write(&[13, 10])?;
        bytes_written += file.write(&[13, 10])?;

        Ok(bytes_written)
    }
}

impl Iterator for File {
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
                .map(|(token, value)| (token.to_owned(), value.to_owned()))
                .collect(),
            body: body_ref.to_owned(),
        };

        return Some(Ok(record));
    }
}
