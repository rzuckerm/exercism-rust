use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Remove all whitespace
    let cleaned_input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    // Excluding "+" and "=", get unique set of characters
    let letters: Vec<char> = cleaned_input
        .chars()
        .filter(|&c| c != '+' && c != '=')
        .unique()
        .collect();
    if letters.is_empty() || letters.len() > 10 {
        return None;
    }

    // Separate left and right sides
    let parts: Vec<&str> = cleaned_input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }

    // Separate addends and store sum
    let addends: Vec<&str> = parts[0].split("+").collect();
    let sum = parts[1];
    if sum.is_empty()
        || addends
            .iter()
            .any(|&word| word.is_empty() || word.len() > sum.len())
    {
        return None;
    }

    // Get set of first letters for addends and sum
    let first_letters: HashSet<char> = HashSet::from_iter(
        addends
            .iter()
            .chain(&[sum])
            .map(|&addend| addend.chars().next().unwrap()),
    );

    // Calculate factors for each letter in addends minus sum
    let factors: Vec<i64> = calc_factors(&addends, &letters)
        .iter()
        .zip(calc_factors(&vec![sum], &letters).iter())
        .into_iter()
        .map(|(&a, &b)| a - b)
        .collect();

    // For each permutation of the length of the number of letters
    for candidate in (0u8..=9u8).permutations(letters.len()) {
        // Skip if any of the first letters are zero
        let index = candidate.iter().position(|&x| x == 0u8);
        if index.is_some() && first_letters.contains(&letters[index.unwrap()]) {
            continue;
        }

        // If letters to answer is 0, return number to letter map
        if calc_value(&candidate, &factors) == 0 {
            return Some(HashMap::from_iter(
                letters.iter().zip(candidate.iter()).map(|(&k, &v)| (k, v)),
            ));
        }
    }

    None
}

fn calc_factors(words: &Vec<&str>, letters: &Vec<char>) -> Vec<i64> {
    let mut letter_multipliers: Vec<i64> = vec![0; letters.len()];
    for &word in words {
        for (n, letter) in word.chars().rev().enumerate() {
            let index = letters.iter().position(|&c| c == letter).unwrap();
            letter_multipliers[index] += 10i64.pow(n as u32);
        }
    }

    letter_multipliers
}

fn calc_value(numbers: &Vec<u8>, factors: &Vec<i64>) -> i64 {
    numbers
        .iter()
        .zip(factors.iter())
        .fold(0i64, |acc, (number, factor)| acc + *number as i64 * *factor)
}
