// 0->1, 1->3, 2->3, 3->7, 4->7, 5->7, 6->7, 7->9, 8->9, 9->1
// +1,   +2,   +1,   +4,   +3,   +2,   +1,   +2,   +1,   +2
const NEXT_DIGITS: [usize; 10] = [1, 3, 3, 7, 7, 7, 7, 9, 9, 1];
const NEXT_INCS: [u32; 10] = [1, 2, 1, 4, 3, 2, 1, 2, 1, 2];

pub fn nth(n: u32) -> u32 {
    (0..=n).fold(0, |acc, _| next_prime(acc))
}

fn next_prime(curr: u32) -> u32 {
    match curr {
        0..=1 => 2,
        2 => 3,
        3..=4 => 5,
        _ => {
            let mut p = curr;
            let mut last_digit = (p % 10) as usize;
            p += NEXT_INCS[last_digit];
            while !is_prime(p) {
                last_digit = NEXT_DIGITS[last_digit];
                p += NEXT_INCS[last_digit];
            }

            p
        }
    }
}

fn is_prime(n: u32) -> bool {
    let mut k: u32 = 3;
    let mut last_digit: usize = 3;
    let mut flag = true;
    while k * k <= n && flag {
        flag = (n % k) != 0;
        k += NEXT_INCS[last_digit];
        last_digit = NEXT_DIGITS[last_digit];
    }

    flag
}
