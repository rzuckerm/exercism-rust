pub fn reply(message: &str) -> &str {
    let trimmed_str = message.trim();
    let letters: String = trimmed_str.chars().filter(|c| c.is_alphabetic()).collect();
    let shout = !letters.is_empty() && letters.chars().all(|c| c.is_uppercase());
    let question = trimmed_str.ends_with("?");
    if trimmed_str.is_empty() {
        "Fine. Be that way!"
    } else if shout {
        if question {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if question {
        "Sure."
    } else {
        "Whatever."
    }
}
