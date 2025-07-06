pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|r| {
            (0..minefield[r].len())
                .map(|c| get_minefield_char(minefield, r as i32, c as i32))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn get_minefield_char(minefield: &[&str], r: i32, c: i32) -> char {
    match minefield[r as usize].as_bytes()[c as usize] {
        b'*' => '*',
        _ => {
            let height = minefield.len() as i32;
            let width = minefield[r as usize].len() as i32;
            match ((r - 1)..=(r + 1))
                .flat_map(|newr| ((c - 1)..=(c + 1)).map(move |newc| (newr, newc)))
                .filter(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width)
                .filter(|&(r, c)| minefield[r as usize].as_bytes()[c as usize] == b'*')
                .count()
            {
                0 => ' ',
                count => (count as u8 + b'0') as char,
            }
        }
    }
}
