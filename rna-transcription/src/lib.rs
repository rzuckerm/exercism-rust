#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const DNA_CHARS: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_CHARS: [char; 4] = ['C', 'G', 'A', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, &DNA_CHARS).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| RNA_CHARS[DNA_CHARS.iter().position(|&d| d == c).unwrap()])
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate(rna, &RNA_CHARS).map(Rna)
    }
}

fn validate(s: &str, valid_chars: &[char; 4]) -> Result<String, usize> {
    s.char_indices().try_fold("".to_string(), |acc, (n, c)| {
        match valid_chars.contains(&c) {
            true => Ok(acc + &c.to_string()),
            false => Err(n),
        }
    })
}
