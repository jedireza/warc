use crate::Record;

use std::fs;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

#[cfg(feature = "gzip")]
use libflate::gzip::Encoder as GzipWriter;

const MB: usize = 1_048_576;

pub struct WarcWriter<W> {
    writer: W,
}

impl<W: Write> WarcWriter<W> {
    pub fn new(w: W) -> Self {
        WarcWriter { writer: w }
    }

    pub fn write(&mut self, record: &Record) -> io::Result<usize> {
        let mut bytes_written = 0;

        bytes_written += self.writer.write(&[87, 65, 82, 67, 47])?;
        bytes_written += self.writer.write(record.version.as_bytes())?;
        bytes_written += self.writer.write(&[13, 10])?;

        for (token, value) in record.headers.iter() {
            bytes_written += self.writer.write(token.to_string().as_bytes())?;
            bytes_written += self.writer.write(&[58, 32])?;
            bytes_written += self.writer.write(&value)?;
            bytes_written += self.writer.write(&[13, 10])?;
        }
        bytes_written += self.writer.write(&[13, 10])?;

        bytes_written += self.writer.write(&record.body)?;
        bytes_written += self.writer.write(&[13, 10])?;
        bytes_written += self.writer.write(&[13, 10])?;

        Ok(bytes_written)
    }
}

impl<W: Write> WarcWriter<BufWriter<W>> {
    pub fn into_inner(self) -> Result<W, std::io::IntoInnerError<BufWriter<W>>> {
        self.writer.into_inner()
    }
}

impl WarcWriter<BufWriter<fs::File>> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let writer = BufWriter::with_capacity(1 * MB, file);

        Ok(WarcWriter::new(writer))
    }
}

#[cfg(feature = "gzip")]
impl WarcWriter<BufWriter<GzipWriter<std::fs::File>>> {
    pub fn from_path_gzip<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let gzip_stream = GzipWriter::new(file)?;
        let writer = BufWriter::with_capacity(1 * MB, gzip_stream);

        Ok(WarcWriter::new(writer))
    }
}
