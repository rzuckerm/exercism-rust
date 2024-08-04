pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let s = self.to_string();
        let chars: Vec<char> = s.chars().filter(|&c| !c.is_whitespace()).collect();
        chars.len() >= 2
            && chars.iter().all(|&c| c.is_ascii_digit())
            && chars.iter().rev().enumerate().fold(0, |acc, (n, c)| {
                let val = (1 + n % 2) * c.to_digit(10).unwrap() as usize;
                (acc + val + val / 10) % 10
            }) == 0
    }
}
