#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut reverse_value: u64 = 0;
        let mut temp = value;
        while temp != 0 {
            reverse_value = 10 * reverse_value + temp % 10;
            temp /= 10;
        }

        if reverse_value == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p: Option<u64> = None;
    let mut max_p: Option<u64> = None;
    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }

        for j in i..=max {
            if j % 10 == 0 {
                continue;
            }

            if let Some(p) = Palindrome::new(i * j) {
                let p = p.into_inner();
                if min_p.is_none() || p < min_p.unwrap() {
                    min_p = Some(p)
                }

                if max_p.is_none() || p > max_p.unwrap() {
                    max_p = Some(p)
                }
            }
        }
    }

    min_p.map(|_| (Palindrome(min_p.unwrap()), Palindrome(max_p.unwrap())))
}
