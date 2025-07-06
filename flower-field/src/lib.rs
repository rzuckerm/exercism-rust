pub fn annotate(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|r| {
            (0..garden[r].len())
                .map(|c| get_garden_char(garden, r, c))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn get_garden_char(garden: &[&str], r: usize, c: usize) -> char {
    match garden[r].as_bytes()[c] {
        b'*' => '*',
        _ => {
            let height = garden.len();
            let width = garden[r].len();
            match (-1..=1)
                .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
                .filter(|&(dr, dc)| dr != 0 || dc != 0)
                .map(|(dr, dc)| (r.wrapping_add_signed(dr), c.wrapping_add_signed(dc)))
                .filter(|&(r, c)| r < height && c < width)
                .filter(|&(r, c)| garden[r].as_bytes()[c] == b'*')
                .count()
            {
                0 => ' ',
                count => (count as u8 + b'0') as char,
            }
        }
    }
}
