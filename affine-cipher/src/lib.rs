#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

static INVERSES: &[i32] = &[1, 9, 21, 15, 3, 19, 0, 7, 23, 11, 5, 17, 25];

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    mmi(a)?;
    Ok(mutate_text::<Vec<char>>(plaintext, |x| a * x + b)
        .chunks(5)
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join(" "))
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_inv = mmi(a)?;
    Ok(mutate_text::<String>(ciphertext, |y| a_inv * (y - b)))
}

fn mmi(a: i32) -> Result<i32, AffineCipherError> {
    (a.rem_euclid(2) != 0 && a.rem_euclid(13) != 0)
        .then_some(INVERSES[a.rem_euclid(26) as usize / 2])
        .ok_or(AffineCipherError::NotCoprime(a))
}

fn mutate_text<T: std::iter::FromIterator<char>>(s: &str, func: impl Fn(i32) -> i32) -> T {
    s.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| match c.to_ascii_lowercase() {
            'a'..='z' => (func((c as i32 & 0x1f) - 1).rem_euclid(26) as u8 + b'a') as char,
            _ => c,
        })
        .collect::<T>()
}
