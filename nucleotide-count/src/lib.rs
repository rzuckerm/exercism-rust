use std::collections::HashMap;

static VALID_NUCLEOTIDES: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match VALID_NUCLEOTIDES.contains(nucleotide) {
        true => dna.chars().try_fold(0, |acc, c| {
            match (VALID_NUCLEOTIDES.contains(c), c == nucleotide) {
                (true, true) => Ok(acc + 1),
                (true, false) => Ok(acc),
                (false, _) => Err(c),
            }
        }),
        false => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from_iter(VALID_NUCLEOTIDES.chars().map(|c| (c, 0))),
        |mut acc, c| match VALID_NUCLEOTIDES.contains(c) {
            true => {
                *acc.entry(c).or_insert(0) += 1;
                Ok(acc)
            }
            false => Err(c),
        },
    )
}
