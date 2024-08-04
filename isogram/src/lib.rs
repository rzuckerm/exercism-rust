use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letter_set: HashSet<char> = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .all(|c| !c.is_alphabetic() || letter_set.insert(c))
}
