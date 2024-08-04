use std::collections::HashSet;

// sum = p = a + b + c
// c = p - a - b
// a^2 + b^2 = c^2
// a^2 + b^2 = [p - (a + b)]^2
// a^2 + b^2 = p^2 - 2*p*(a + b) + (a + b)^2
// a^2 + b^2 = p^2 - 2*p*(a + b) + a^2 + 2*a*b + b^2
// p^2 - 2*p*(a + b) + 2*a*b = 0
// p^2 - 2*p*a - 2*p*b + 2*a*b = 0
// p*(p - 2*a) = 2*b*(p - a)
// b = [p*(p - 2*a)] / [2*(p - a)]
//
// Find value of a that equals b:
//
// [p*(p - 2*a)] / [2*(p - a)] = a
// p*(p - 2*a) = 2*a*(p - a)
// p^2 - 2*p*a = 2*p*a - 2*a^2
// p^2 - 4*p*a + 2*a^2 = 0
//
// a = [4*p +/- sqrt(16*p^2 - 4*2*p^2)] / (2*2)
//   = [4*p +/- sqrt(8*p^2)] / 4
//   = p*[1 +/- 1/sqrt(2)]
//
// Since a < p:
//
// a = p*[1 - 1/sqrt(2)]
//
// Therefore:
//
// a <= p*[1 - 1/sqrt(2)]
//
// The smallest Pythagorean Triplet is [3, 4, 5], so a >= 3
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (3..=(sum as f32 * (1.0 - 1.0 / 2.0f32.sqrt())) as u32)
        .map(|a| (a, sum * (sum - 2 * a), 2 * (sum - a)))
        .filter_map(|(a, n, d)| (n % d == 0).then_some([a, n / d, sum - a - n / d]))
        .collect()
}
