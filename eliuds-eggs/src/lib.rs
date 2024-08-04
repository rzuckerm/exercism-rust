pub fn egg_count(display_value: u32) -> usize {
    (0..=31).fold(0, |acc, n| acc + (display_value >> n & 1) as usize)
}
