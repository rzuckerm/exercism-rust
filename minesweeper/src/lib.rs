pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|r| {
            (0..minefield[r].len())
                .map(|c| get_minefield_char(minefield, r, c))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn get_minefield_char(minefield: &[&str], r: usize, c: usize) -> char {
    match minefield[r].as_bytes()[c] {
        b'*' => '*',
        _ => {
            let height = minefield.len();
            let width = minefield[r].len();
            match (-1..=1)
                .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
                .filter(|&(dr, dc)| dr != 0 || dc != 0)
                .map(|(dr, dc)| (r.wrapping_add_signed(dr), c.wrapping_add_signed(dc)))
                .filter(|&(r, c)| r < height && c < width)
                .filter(|&(r, c)| minefield[r].as_bytes()[c] == b'*')
                .count()
            {
                0 => ' ',
                count => (count as u8 + b'0') as char,
            }
        }
    }
}
