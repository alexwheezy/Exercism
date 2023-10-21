use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let read = self.wrapped.read(buf)?;
        self.bytes += read;
        Ok(read)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            writes: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let read = self.wrapped.write(buf)?;
        self.bytes += read;
        Ok(read)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()?;
        Ok(())
    }
}
