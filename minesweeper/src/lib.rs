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
            let height: i32 = minefield.len() as i32;
            let width: i32 = minefield[r as usize].len() as i32;
            match (-1..=1)
                .flat_map(|r_offset| (-1..=1).map(move |c_offset| (r_offset, c_offset)))
                .filter(|&(r_offset, c_offset)| r_offset != 0 || c_offset != 0)
                .map(|(r_offset, c_offset)| (r + r_offset, c + c_offset))
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
