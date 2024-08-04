use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_dist = get_letter_frequency(word);
    HashSet::from_iter(
        possible_anagrams
            .iter()
            .copied()
            .filter(|candidate| {
                word.to_lowercase() != candidate.to_lowercase()
                    && word_dist == get_letter_frequency(candidate)
            })
            .collect::<Vec<_>>(),
    )
}

fn get_letter_frequency(word: &str) -> HashMap<char, i32> {
    word.to_lowercase()
        .chars()
        .fold(HashMap::<char, i32>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
