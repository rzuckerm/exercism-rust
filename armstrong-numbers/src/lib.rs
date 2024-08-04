pub fn is_armstrong_number(num: u32) -> bool {
    let num_as_str = num.to_string();
    let expon = num_as_str.len() as u32;
    num as u64 == num_as_str
        .chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(expon))
        .sum()
}
