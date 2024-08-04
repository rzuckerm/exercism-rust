pub fn answer(command: &str) -> Option<i32> {
    let words: Vec<&str> = command
        .strip_prefix("What is ")?
        .strip_suffix('?')?
        .split_whitespace()
        .collect();
    let mut result: i32 = words.first()?.parse().ok()?;
    let mut words: &[&str] = &words[1..];
    while !words.is_empty() {
        (result, words) = match words {
            ["plus", value, rest @ ..] => (result + value.parse::<i32>().ok()?, rest),
            ["minus", value, rest @ ..] => (result - value.parse::<i32>().ok()?, rest),
            ["multiplied", "by", value, rest @ ..] => (result * value.parse::<i32>().ok()?, rest),
            ["divided", "by", value, rest @ ..] => (result / value.parse::<i32>().ok()?, rest),
            ["raised", "to", "the", value, "power", rest @ ..] => (
                result.pow(
                    value
                        .trim_end_matches(char::is_alphabetic)
                        .parse::<u32>()
                        .ok()?,
                ),
                rest,
            ),
            _ => return None,
        };
    }

    Some(result)
}
