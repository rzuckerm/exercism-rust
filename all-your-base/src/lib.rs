#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base < 2, to_base < 2) {
        (true, _) => Err(Error::InvalidInputBase),
        (_, true) => Err(Error::InvalidOutputBase),
        (_, _) => match number.iter().find(|&&digit| digit >= from_base) {
            Some(&bad_digit) => Err(Error::InvalidDigit(bad_digit)),
            _ => {
                let mut from_value = number
                    .iter()
                    .fold(0, |acc, &digit| acc * from_base as u128 + digit as u128);
                let mut result: Vec<u32> = if from_value == 0 { vec![0] } else { vec![] };
                while from_value > 0 {
                    result.push((from_value % to_base as u128) as u32);
                    from_value /= to_base as u128;
                }
                result.reverse();

                Ok(result)
            }
        },
    }
}
