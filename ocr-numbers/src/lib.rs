#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const OCR_NUMBERS: [&str; 4] = [
    " _     _  _     _  _  _  _  _ ",
    "| |  | _| _||_||_ |_   ||_||_|",
    "|_|  ||_  _|  | _||_|  ||_| _|",
    "                              ",
];

pub fn convert(input: &str) -> Result<String, Error> {
    let mut res: Vec<String> = Vec::new();
    for lines in input.lines().collect::<Vec<&str>>().chunks(4) {
        if lines.len() != 4 {
            return Err(Error::InvalidRowCount(lines.len()));
        } else if let Some(&line) = lines.iter().find(|&&line| line.len() % 3 != 0) {
            return Err(Error::InvalidColumnCount(line.len()));
        }

        let mut numbers = String::new();
        for i in (0..lines[0].len()).step_by(3) {
            numbers.push(
                (0..10)
                    .position(|k| {
                        (0..4).all(|j| lines[j][i..i + 3] == OCR_NUMBERS[j][3 * k..3 * k + 3])
                    })
                    .map_or('?', |n| (b'0' + n as u8) as char),
            )
        }

        res.push(numbers);
    }

    Ok(res.join(","))
}
