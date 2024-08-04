use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    num_bytes_read: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            data: wrapped,
            num_bytes_read: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.num_reads += 1;
        let n = self.data.read(buf)?;
        self.num_bytes_read += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    data: W,
    num_bytes_written: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            data: wrapped,
            num_bytes_written: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.num_bytes_written
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.num_writes += 1;
        let n = self.data.write(buf)?;
        self.num_bytes_written += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
