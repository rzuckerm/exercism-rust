use anyhow::Error;
use std::fs;

#[derive(Debug)]
pub struct Flags {
    n: bool,
    i: bool,
    l: bool,
    x: bool,
    v: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            n: flags.contains(&"-n"),
            i: flags.contains(&"-i"),
            l: flags.contains(&"-l"),
            x: flags.contains(&"-x"),
            v: flags.contains(&"-v"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let converter = match flags.i {
        true => str::to_lowercase,
        false => str::to_string,
    };
    let pattern = converter(pattern);
    let matcher = |pattern: &str, line: String| match flags.x {
        true => (line == pattern) ^ flags.v,
        false => line.contains(pattern) ^ flags.v,
    };
    let formatter =
        |line_num: usize, line: &str, file: &str| match (flags.l, flags.n, files.len() > 1) {
            (true, _, _) => file.to_string(),
            (false, false, false) => line.to_string(),
            (false, false, true) => format!("{}:{}", file, line),
            (false, true, false) => format!("{}:{}", line_num + 1, line),
            (false, true, true) => format!("{}:{}:{}", file, line_num + 1, line),
        };

    let mut result = Vec::<String>::new();
    for file in files {
        for (line_num, line) in fs::read_to_string(file)?.lines().enumerate() {
            if matcher(&pattern, converter(line)) {
                result.push(formatter(line_num, line, file));
                if flags.l {
                    break;
                }
            }
        }
    }

    Ok(result)
}
