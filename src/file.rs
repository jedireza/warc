use crate::WarcRecord;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path::Path;

pub struct WarcFile {
    file: File,
}

impl WarcFile {
    /// Opens a file for both reading and writing, as well as creating if it doesn't already exist.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(WarcFile { file: file })
    }

    /// Writes a `WarcRecord` to the end of the file. Note that this function moves the cursor.
    pub fn write(&mut self, record: WarcRecord) -> io::Result<usize> {
        self.file.seek(io::SeekFrom::End(0))?;

        let mut bytes_written = 0;

        bytes_written += self.file.write("WARC/".as_bytes())?;
        bytes_written += self.file.write(record.version.as_bytes())?;
        bytes_written += self.file.write("\r\n".as_bytes())?;

        for header in record.headers.iter() {
            bytes_written += self.file.write(header.token.as_bytes())?;
            bytes_written += self.file.write(&header.delim_left)?;
            bytes_written += self.file.write(":".as_bytes())?;
            bytes_written += self.file.write(&header.delim_right)?;
            bytes_written += self.file.write(&header.value)?;
            bytes_written += self.file.write("\r\n".as_bytes())?;
        }
        bytes_written += self.file.write("\r\n".as_bytes())?;

        bytes_written += self.file.write(&record.body)?;
        bytes_written += self.file.write("\r\n".as_bytes())?;
        bytes_written += self.file.write("\r\n".as_bytes())?;

        Ok(bytes_written)
    }
}
