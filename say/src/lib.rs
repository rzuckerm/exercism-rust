static FIRST_20: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

static MULTIPLES_OF_10: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

static POWERS_OF_1000: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    match n {
        0..=19 => FIRST_20[n as usize].to_string(),
        20..=99 => match ((n - 20) / 10, n % 10) {
            (x, 0) => MULTIPLES_OF_10[x as usize].to_string(),
            (x, y) => format!("{}-{}", MULTIPLES_OF_10[x as usize], FIRST_20[y as usize]),
        },
        100..=999 => match n % 100 {
            0 => format!("{} hundred", encode(n / 100)),
            x => format!("{} hundred {}", encode(n / 100), encode(x)),
        },
        _ => match n % 1000 {
            0 => encode_powers_of_1000(n),
            m => format!("{} {}", encode_powers_of_1000(n), encode(m)),
        },
    }
}

fn encode_powers_of_1000(n: u64) -> String {
    let mut n = n;
    let mut factors = vec![];
    for power in POWERS_OF_1000 {
        n /= 1000;
        match (n, n % 1000) {
            (0, _) => break,
            (_, 0) => (),
            (_, factor) => factors.push(format!("{} {}", encode(factor), power)),
        }
    }

    factors.reverse();
    factors.join(" ")
}
