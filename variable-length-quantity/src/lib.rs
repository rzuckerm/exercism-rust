use std::iter::once;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|v| {
            let num_bytes = (31 - (*v | 1).leading_zeros()) / 7 + 1;
            (1..num_bytes)
                .rev()
                .map(|i| (0x80 | ((v.to_owned() >> (7 * i)) & 0x7f)) as u8)
                .chain(once(*v as u8 & 0x7f))
        })
        .collect()
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .split_inclusive(|&b| b < 0x80)
        .map(|arr| {
            match (*arr.last().unwrap(), arr.len()) {
                (0x80..=0xff, _) => Err(Error::IncompleteNumber)?,
                (_, 6..) => Err(Error::Overflow)?,
                (_, 5) if arr[0] > 0x8f => Err(Error::Overflow)?,
                (_, _) => Ok(arr
                    .iter()
                    .fold(0u32, |value, b| value << 7 | (b & 0x7f) as u32)),
            }
        })
        .collect()
}
