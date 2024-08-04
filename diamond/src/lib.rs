pub fn get_diamond(c: char) -> Vec<String> {
    let n = c.to_ascii_uppercase() as i8 - 'A' as i8;
    (-n..=n)
        .map(|i| {
            let letter = (b'A' + n as u8 - i.unsigned_abs()) as char;
            (-n..=n)
                .map(|j| if j.abs() == n - i.abs() { letter } else { ' ' })
                .collect()
        })
        .collect()
}
