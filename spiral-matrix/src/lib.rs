pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut position = (0, 0);
    let mut direction = (0, 1);
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let mut n = 1u32..;
    for turns_left in (0..2 * size as i32).rev() {
        for _ in 0..(turns_left + 1) / 2 {
            matrix[position.0 as usize][position.1 as usize] = n.next().unwrap();
            position = (position.0 + direction.0, position.1 + direction.1);
        }
        position.0 += direction.1 - direction.0;
        position.1 -= direction.1 + direction.0;
        direction = (direction.1, -direction.0);
    }

    matrix
}
