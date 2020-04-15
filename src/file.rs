use crate::parser;
use crate::{WarcHeader, WarcRecord};
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

    pub fn write(&mut self, record: WarcRecord) -> io::Result<usize> {
        let mut bytes_written = 0;

        let mut file = self.reader.get_ref();
        file.seek(SeekFrom::End(0))?;

        bytes_written += file.write(&[87, 65, 82, 67, 47])?;
        bytes_written += file.write(record.version.as_bytes())?;
        bytes_written += file.write(&[13, 10])?;

        for header in record.headers.iter() {
            bytes_written += file.write(header.token.as_bytes())?;
            bytes_written += file.write(&header.delim_left)?;
            bytes_written += file.write(&[58])?;
            bytes_written += file.write(&header.delim_right)?;
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

// TODO: better error handling throughout
impl Iterator for WarcFile {
    type Item = WarcRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let mut header_buffer = Vec::with_capacity(64 * KB);
        let mut found_headers = false;
        while !found_headers {
            let bytes_read = match self.reader.read_until(b'\n', &mut header_buffer) {
                Err(err) => {
                    println!("Error reading buffer: {}", err);
                    return None;
                }
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

        let headers = match parser::headers(&header_buffer) {
            Err(err) => {
                println!("Error parsing headers: {}", err);
                return None;
            }
            Ok(parsed) => parsed.1,
        };
        let version_ref = headers.0;
        let headers_ref = headers.1;
        let expected_body_len = headers.2;

        let mut body_buffer = Vec::with_capacity(1 * MB);
        let mut found_body = false;
        let mut body_bytes_read = 0;
        while !found_body {
            let bytes_read = match self.reader.read_until(b'\n', &mut body_buffer) {
                Err(err) => {
                    println!("Error reading buffer: {}", err);
                    return None;
                }
                Ok(len) => len,
            };

            body_bytes_read += bytes_read;

            // we expect 4 characters (\r\n\r\n) to exist after the body
            if bytes_read == 2 && body_bytes_read == expected_body_len + 4 {
                found_body = true;
            }
        }

        let body = &body_buffer[..expected_body_len];

        let warc_record = WarcRecord {
            version: version_ref.to_owned(),
            headers: headers_ref
                .into_iter()
                .map(|header_ref| WarcHeader::from(header_ref))
                .collect(),
            body: body.to_owned(),
        };

        return Some(warc_record);
    }
}
