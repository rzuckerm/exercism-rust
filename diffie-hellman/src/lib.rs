use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    module_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    module_exp(b_pub, a, p)
}

// Calculate x**y mod m
fn module_exp(x: u64, mut y: u64, m: u64) -> u64 {
    let m: u128 = m as u128;
    let mut result = 1u128;
    let mut multiplier: u128 = (x as u128) % m;
    while y > 0 {
        if y % 2 == 1 {
            result = (result * multiplier) % m;
        }

        multiplier = (multiplier * multiplier) % m;
        y /= 2;
    }

    result as u64
}
