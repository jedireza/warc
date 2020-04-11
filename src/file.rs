use crate::WarcRecord;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::marker::PhantomData;
use std::path;

pub struct WarcFile<'a> {
    file: File,
    phantom: PhantomData<&'a ()>,
}

impl<'a> WarcFile<'a> {
    /// Opens a file for both reading and writing, as well as creating if it doesn't already exist.
    pub fn open<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        file.seek(io::SeekFrom::Start(0))?;

        Ok(WarcFile {
            file: file,
            phantom: PhantomData,
        })
    }

    /// Writes a `WarcRecord` to the end of the file. Note that this function moves the cursor.
    pub fn write(&mut self, record: WarcRecord) -> io::Result<usize> {
        self.file.seek(io::SeekFrom::End(0))?;

        let mut bytes_written = 0;

        bytes_written += self.file.write("WARC/".as_bytes())?;
        bytes_written += self.file.write(record.version)?;
        bytes_written += self.file.write("\r\n".as_bytes())?;

        for header in record.headers.iter() {
            bytes_written += self.file.write(header.token.as_bytes())?;
            bytes_written += self.file.write(header.delim_left)?;
            bytes_written += self.file.write(":".as_bytes())?;
            bytes_written += self.file.write(header.delim_right)?;
            bytes_written += self.file.write(header.value)?;
            bytes_written += self.file.write("\r\n".as_bytes())?;
        }
        bytes_written += self.file.write("\r\n".as_bytes())?;

        bytes_written += self.file.write(record.body)?;
        bytes_written += self.file.write("\r\n".as_bytes())?;
        bytes_written += self.file.write("\r\n".as_bytes())?;

        Ok(bytes_written)
    }
}
