use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

pub struct WAL {
    inner: File,
}

impl WAL {
    pub fn new() -> Result<Self, io::Error> {
        let inner = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open("target/wal.txt")?;

        Ok(WAL { inner })
    }

    pub fn into_buf_reader(self) -> io::BufReader<File> {
        io::BufReader::new(self.inner)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        self.inner.write_all(buf)
    }
}
