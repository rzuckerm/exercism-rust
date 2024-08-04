pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = (student.to_uppercase().chars().next().unwrap() as usize - 65) * 2;
    diagram
        .lines()
        .flat_map(|line| {
            assert!(index + 1 < line.len());
            line[index..=(index + 1)].chars().map(|c| match c {
                'C' => "clover",
                'G' => "grass",
                'R' => "radishes",
                'V' => "violets",
                x => panic!("Invalid character {x}"),
            })
        })
        .collect()
}
