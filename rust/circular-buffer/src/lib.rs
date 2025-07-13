use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer.len() == self.buffer.capacity() {
            return Err(Error::FullBuffer);
        }
        self.buffer.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        match self.buffer.pop_front() {
            Some(element) => Ok(element),
            None => Err(Error::EmptyBuffer),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() == self.buffer.capacity() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(element);
    }
}
