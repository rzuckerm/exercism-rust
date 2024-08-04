pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| match c.is_ascii_alphabetic() {
            true => (((c as u8 & 0x1f) - 1 + key) % 26 + (b'A' | c as u8 & 0x20)) as char,
            false => c,
        })
        .collect()
}
