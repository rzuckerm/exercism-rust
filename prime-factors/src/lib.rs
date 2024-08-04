pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    let mut k = n;
    let mut candidate = 2;
    while k > 1 {
        while k % candidate == 0 {
            prime_factors.push(candidate);
            k /= candidate;
        }

        if candidate == 2 {
            candidate = 3;
        } else {
            candidate += 2;
        }
    }

    prime_factors
}
