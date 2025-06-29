use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, factors: (u64, u64)) -> Palindrome {
        Self {
            value,
            factors: HashSet::from([factors]),
        }
    }

    pub fn add_factors(&mut self, factors: (u64, u64)) -> bool {
        self.factors.insert(factors)
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p: Option<Palindrome> = None;
    let mut max_p: Option<Palindrome> = None;
    for i in min..=max {
        for j in i..=max {
            let value = i * j;
            if is_palindrome(value) {
                min_p = match min_p.as_ref().map(|x| x.value.cmp(&value)) {
                    Some(Ordering::Equal) => {
                        min_p.as_mut().unwrap().add_factors((i, j));
                        min_p
                    }
                    Some(Ordering::Greater) | None => Some(Palindrome::new(value, (i, j))),
                    _ => min_p,
                };

                max_p = match max_p.as_ref().map(|x| x.value.cmp(&value)) {
                    Some(Ordering::Equal) => {
                        max_p.as_mut().unwrap().add_factors((i, j));
                        max_p
                    }
                    Some(Ordering::Less) | None => Some(Palindrome::new(value, (i, j))),
                    _ => max_p,
                };
            }
        }
    }

    min_p.zip(max_p)
}

#[inline]
fn is_palindrome(value: u64) -> bool {
    let mut reverse_value: u64 = 0;
    let mut temp = value;
    while temp != 0 {
        reverse_value = 10 * reverse_value + temp % 10;
        temp /= 10;
    }

    reverse_value == value
}
