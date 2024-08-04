pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.replace("-", "")
        .chars()
        .rev()
        .try_fold((0u32, 1u32), |(sum, n), c| {
            if n <= 10 && (c.is_ascii_digit() || (n == 1 && c == 'X')) {
                Ok((sum + n * c.to_digit(10).unwrap_or(10), n + 1))
            } else {
                Err(())
            }
        })
        .is_ok_and(|(sum, n)| sum % 11 == 0 && n == 11)
}
