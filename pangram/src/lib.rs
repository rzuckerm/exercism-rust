pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .fold(0u32, |acc, c| acc | (1 << ((c as u32 & 0x1f) - 1)))
        == (1 << 26) - 1
}
