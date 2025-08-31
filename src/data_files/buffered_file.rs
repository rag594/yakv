use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct BufferedFile {
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

impl BufferedFile {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // Open two independent handles (read & write have separate cursors)
        let file_for_write = OpenOptions::new().create(true).append(true).open(&path)?;
        let file_for_read = OpenOptions::new().read(true).open(&path)?;

        Ok(Self {
            reader: BufReader::new(file_for_read),
            writer: BufWriter::new(file_for_write),
        })
    }

    /// Append bytes to the file
    pub fn append(&mut self, data: &[u8]) -> io::Result<()> {
        self.writer.write_all(data)?;
        Ok(())
    }

    /// Flush write buffer to disk
    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()?;
        // for durability: self.writer.get_ref().sync_all()?;
        Ok(())
    }

    /// Read from an offset into buffer
    pub fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        let inner = self.reader.get_mut();
        inner.seek(SeekFrom::Start(offset))?;
        self.reader.read(buf)
    }
}
