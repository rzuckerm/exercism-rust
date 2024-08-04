pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve: Vec<u64> = (0..=upper_bound).collect();
    sieve[1] = 0;
    let q = (upper_bound as f64).sqrt() as usize;
    let upper_bound = upper_bound as usize;
    for k in 2..=q {
        if sieve[k] != 0 {
            (k * k..=upper_bound).step_by(k).for_each(|m| sieve[m] = 0);
        }
    }

    sieve.iter().filter(|&n| n != &0).copied().collect()
}
