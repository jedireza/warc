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
                found_headers = true;
            }
        }

        let headers = match parser::headers(&header_buffer) {
            Err(err) => {
                println!("Error parsing headers: {}", err);
                return None;
            }
            Ok(parsed_headers) => parsed_headers.1,
        };

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

            if bytes_read == 2 && body_bytes_read == headers.2 + 4 {
                found_body = true;
            }
        }

        let body = &body_buffer[..headers.2];

        let warc_record = WarcRecord {
            version: headers.0.to_owned(),
            headers: headers
                .1
                .into_iter()
                .map(|parsed_header| WarcHeader {
                    token: parsed_header.token.to_owned(),
                    value: parsed_header.value.to_owned(),
                    delim_left: parsed_header.delim_left.to_owned(),
                    delim_right: parsed_header.delim_right.to_owned(),
                })
                .collect(),
            body: body.to_owned(),
        };

        return Some(warc_record);
    }
}
