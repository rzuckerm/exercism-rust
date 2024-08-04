pub fn encode(plain: &str) -> String {
    decode(plain)
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| match c.to_ascii_lowercase() {
            'a'..='z' => (b'z' + 1 - (c as u8 & 0x1f)) as char,
            _ => c,
        })
        .collect::<String>()
}
