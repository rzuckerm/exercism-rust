use std::borrow::Borrow;
use std::io::{Read, Result, Write};
use std::iter::Cycle;
use std::slice::Iter;

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

// This fixes "the hidden type for `impl std::iter::Iterator<Item = u8> + 'b`
// captures lifetime that does not appear in bounds" error
// Reference: https://docs.rs/fix-hidden-lifetime-bug/latest/fix_hidden_lifetime_bug/trait.Captures.html
pub trait Captures<'a> {}
impl<'a, T> Captures<'a> for T {}

impl<'a> Xorcism<'a> {
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Self {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|b| *b ^= self.key.next().unwrap());
    }

    pub fn munge<'b, Data: IntoIterator<Item = impl Borrow<u8>> + 'b>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + Captures<'a> + 'b {
        data.into_iter()
            .map(move |b| b.borrow() ^ self.key.next().unwrap())
    }

    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        XorcismReader {
            xorcism: self,
            reader,
        }
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        XorcismWriter {
            xorcism: self,
            writer,
        }
    }
}

struct XorcismReader<'a, R> {
    xorcism: Xorcism<'a>,
    reader: R,
}

impl<'a, R: Read> Read for XorcismReader<'a, R> {
    fn read(&mut self, data: &mut [u8]) -> Result<usize> {
        let n = self.reader.read(data)?;
        self.xorcism.munge_in_place(data);
        Ok(n)
    }
}

struct XorcismWriter<'a, W> {
    xorcism: Xorcism<'a>,
    writer: W,
}

impl<'a, W: Write> Write for XorcismWriter<'a, W> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.writer
            .write(&self.xorcism.munge(data).collect::<Vec<u8>>())
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
