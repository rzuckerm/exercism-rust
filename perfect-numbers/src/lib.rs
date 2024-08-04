use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => {
            let q = (num as f64).sqrt() as u64;
            let mut total = (2..=q)
                .filter(|&factor| num % factor == 0)
                .fold(1, |total, factor| total + factor + num / factor);
            if q * q == num {
                total -= q;
            }

            match total.cmp(&num) {
                Ordering::Less => Some(Classification::Deficient),
                Ordering::Equal => Some(Classification::Perfect),
                Ordering::Greater => Some(Classification::Abundant),
            }
        }
    }
}
