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
        let mut put = |bytes: &[u8]| -> io::Result<()> {
            self.writer.write_all(bytes)?;
            bytes_written += bytes.len();
            Ok(())
        };

        put(b"WARC/")?;
        put(headers.version.as_bytes())?;
        put(b"\r\n")?;

        for (token, value) in ordered_headers(&headers) {
            put(token.as_bytes())?;
            put(b": ")?;
            put(value)?;
            put(b"\r\n")?;
        }
        put(b"\r\n")?;

        put(body.as_ref())?;
        put(b"\r\n\r\n")?;

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
            .truncate(false)
            .open(&path)?;
        let writer = BufWriter::with_capacity(MB, file);

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
            .truncate(false)
            .open(&path)?;
        let gzip_stream = GzipWriter::new(file)?;
        let writer = BufWriter::with_capacity(MB, gzip_stream);

        Ok(WarcWriter::new(writer))
    }
}

/// The headers in a stable order: the four every record must carry (WARC 1.1 section 4), in the order the standard
/// lists them, then the rest by name. The same record is therefore always written as the same bytes.
fn ordered_headers(headers: &RawRecordHeader) -> Vec<(String, &Vec<u8>)> {
    const MANDATORY: [&str; 4] = ["warc-type", "warc-record-id", "warc-date", "content-length"];
    let mut out: Vec<(String, &Vec<u8>)> = headers
        .as_ref()
        .iter()
        .map(|(token, value)| (token.to_string(), value))
        .collect();
    out.sort_by(|(a, _), (b, _)| {
        let rank = |name: &str| {
            MANDATORY
                .iter()
                .position(|m| *m == name)
                .unwrap_or(MANDATORY.len())
        };
        rank(a).cmp(&rank(b)).then_with(|| a.cmp(b))
    });
    out
}

#[cfg(test)]
mod writer_tests {
    use super::WarcWriter;
    use crate::{header::WarcHeader, RecordType};
    use chrono::TimeZone;
    use std::io::{self, Write};

    /// A sink that accepts at most three bytes per call, as a pipe or socket may.
    struct Trickle(Vec<u8>);

    impl Write for Trickle {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let n = buf.len().min(3);
            self.0.extend_from_slice(&buf[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn record() -> crate::Record<crate::BufferedBody> {
        crate::RecordBuilder::default()
            .warc_type(RecordType::Resource)
            .warc_id("<urn:uuid:00000000-0000-4000-8000-000000000000>")
            .date(chrono::Utc.timestamp_opt(0, 0).unwrap())
            .header(WarcHeader::TargetURI, "https://example.org/")
            .header(WarcHeader::ContentType, "text/plain")
            .header(WarcHeader::BlockDigest, "sha1:AAAA")
            .body(b"hello".to_vec())
            .build()
            .unwrap()
    }

    #[test]
    fn short_writes_lose_nothing() {
        let mut w = WarcWriter::new(Trickle(Vec::new()));
        let n = w.write(&record()).unwrap();
        let bytes = w.writer.0;
        assert_eq!(n, bytes.len());
        assert!(bytes.ends_with(b"\r\n\r\nhello\r\n\r\n"));
    }

    #[test]
    fn headers_are_written_in_a_stable_order() {
        let mut w = WarcWriter::new(Vec::new());
        w.write(&record()).unwrap();
        let text = String::from_utf8(w.writer).unwrap();
        let names: Vec<&str> = text
            .lines()
            .skip(1)
            .take_while(|l| !l.is_empty())
            .map(|l| l.split(':').next().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "warc-type",
                "warc-record-id",
                "warc-date",
                "content-length",
                "content-type",
                "warc-block-digest",
                "warc-target-uri"
            ]
        );
    }
}
