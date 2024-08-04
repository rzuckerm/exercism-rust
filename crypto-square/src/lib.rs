pub fn encrypt(input: &str) -> String {
    let input = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<Vec<char>>();
    let r = 1.max((input.len() as f32).sqrt() as usize);
    let c = (input.len() + r - 1) / r;
    (0..(r * c + 1.max(c) - 1))
        .map(|n| *input.get(n / (r + 1) + (n % (r + 1)) * c).unwrap_or(&' '))
        .collect()
}
