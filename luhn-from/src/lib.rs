pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let chars: Vec<char> = self.0.chars().filter(|&c| !c.is_whitespace()).collect();
        chars.len() >= 2
            && chars.iter().all(|&c| c.is_ascii_digit())
            && chars.iter().rev().enumerate().fold(0, |acc, (n, c)| {
                let val = (1 + n % 2) * c.to_digit(10).unwrap() as usize;
                (acc + val + val / 10) % 10
            }) == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
