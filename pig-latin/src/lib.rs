pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let mut split_point = word.len();
    if !["a", "e", "i", "o", "u", "xr", "yt"]
        .iter()
        .any(|&p| word.starts_with(p))
    {
        if word.starts_with("y") {
            split_point = 1
        } else {
            split_point = word.find(|c| "aeiouy".contains(c)).unwrap();
            if word.get(split_point - 1..=split_point) == Some("qu") {
                split_point += 1;
            }
        }
    }

    let (prefix, suffix) = word.split_at(split_point);
    suffix.to_owned() + prefix + "ay"
}
