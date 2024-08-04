use std::collections::VecDeque;

pub struct CircularBuffer<T>(VecDeque<T>);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self(VecDeque::<T>::with_capacity(capacity))
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        (self.0.len() < self.0.capacity())
            .then(|| self.0.push_back(element))
            .ok_or(Error::FullBuffer)
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.0.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        (self.0.len() >= self.0.capacity()).then(|| self.read());
        let _ = self.write(element);
    }
}
