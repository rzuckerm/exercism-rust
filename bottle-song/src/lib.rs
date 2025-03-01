const NUMBERS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i: u32| verse(start_bottles - i))
        .collect()
}

fn verse(n: u32) -> String {
    format!(
        "{0},\n{0},\nAnd if one green bottle should accidentally fall,\nThere'll be {1}.\n\n",
        bottles(n),
        bottles(n - 1).to_ascii_lowercase()
    )
}

fn bottles(n: u32) -> String {
    format!(
        "{} green bottle{} hanging on the wall",
        String::from(NUMBERS[n as usize]),
        (if n != 1 { "s" } else { "" })
    )
}
