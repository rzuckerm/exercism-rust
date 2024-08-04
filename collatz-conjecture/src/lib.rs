pub fn collatz(n: u64) -> Option<u64> {
    let mut x = n;
    for i in 0.. {
        match x {
            0 => return None,
            1 => return Some(i),
            is_even if is_even % 2 == 0 => x /= 2,
            _ => x = x.checked_mul(3)?.checked_add(1)?,
        }
    }

    None
}
