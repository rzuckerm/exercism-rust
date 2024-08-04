pub fn encode(source: &str) -> String {
    let (mut s, run_len, prev_c) =
        source
            .chars()
            .fold(("".to_string(), 0, None), |(s, run_len, prev_c), c| {
                if run_len > 0 && prev_c != Some(c) {
                    (
                        s + &encode_char(run_len, prev_c.unwrap()),
                        1,
                        Some(c),
                    )
                } else {
                    (s, run_len + 1, Some(c))
                }
            });

    if run_len > 0 && prev_c.is_some() {
        s += &encode_char(run_len, prev_c.unwrap());
    }

    s
}

fn encode_char(run_len: usize, c: char) -> String {
    match run_len {
        1 => c.to_string(),
        _ => format!("{run_len}{c}"),
    }
}
pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold(("".to_string(), 0), |(s, run_len), c| match c {
            '0'..='9' => (s, 10 * run_len + c as usize - '0' as usize),
            _ => (s + c.to_string().repeat(run_len.max(1)).as_ref(), 0),
        })
        .0
}
