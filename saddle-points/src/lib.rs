pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for (i, row) in input.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if row.iter().all(|&x| cell >= x) && input.iter().all(|v| cell <= v[j]) {
                points.push((i, j));
            }
        }
    }

    points
}
