use crate::{BufferedBody, RawRecordHeader, Record};

use std::fs;
use std::io;
use std::io::{BufWriter, Write};
use std::path::Path;

#[cfg(feature = "gzip")]
use libflate::gzip::Encoder as GzipWriter;

const MB: usize = 1_048_576;

/// A writer which writes records to an output stream.
pub struct WarcWriter<W> {
    writer: W,
}

impl<W: Write> WarcWriter<W> {
    /// Create a new writer.
    pub fn new(w: W) -> Self {
        WarcWriter { writer: w }
    }

    /// Write a single record.
    ///
    /// The number of bytes written is returned upon success.
    pub fn write(&mut self, record: &Record<BufferedBody>) -> io::Result<usize> {
        let (headers, body) = record.clone().into_raw_parts();
        self.write_raw(headers, &body)
    }

    /// Write a single raw record.
    ///
    /// The number of bytes written is returned upon success.
    pub fn write_raw<B>(&mut self, headers: RawRecordHeader, body: &B) -> io::Result<usize>
    where
        B: AsRef<[u8]>,
    {
        let mut bytes_written = 0;

        bytes_written += self.writer.write(&[87, 65, 82, 67, 47])?;
        bytes_written += self.writer.write(headers.version.as_bytes())?;
        bytes_written += self.writer.write(&[13, 10])?;

        for (token, value) in headers.as_ref().iter() {
            bytes_written += self.writer.write(token.to_string().as_bytes())?;
            bytes_written += self.writer.write(&[58, 32])?;
            bytes_written += self.writer.write(&value)?;
            bytes_written += self.writer.write(&[13, 10])?;
        }
        bytes_written += self.writer.write(&[13, 10])?;

        bytes_written += self.writer.write(body.as_ref())?;
        bytes_written += self.writer.write(&[13, 10])?;
        bytes_written += self.writer.write(&[13, 10])?;

        Ok(bytes_written)
    }
}

impl<W: Write> WarcWriter<BufWriter<W>> {
    /// Consume this writer and return the inner writer.
    ///
    /// # Flushing Compressed Data Streams
    ///
    /// This method is necessary to be called at the end of a GZIP-compressed stream. An extra call
    /// is needed to flush the buffer of data, and write a trailer to the output stream.
    ///
    /// ```ignore
    /// let gzip_stream = writer.into_inner()?;
    /// gzip_writer.finish().into_result()?;
    /// ```
    ///
    pub fn into_inner(self) -> Result<W, std::io::IntoInnerError<BufWriter<W>>> {
        self.writer.into_inner()
    }
}

impl WarcWriter<BufWriter<fs::File>> {
    /// Create a new writer which writes to a file.
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
    /// Create a new writer which writes to a GZIP-compressed file.
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
