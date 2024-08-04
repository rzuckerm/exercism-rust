pub struct Matrix(Vec<Vec<u32>>);

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
                .collect(),
        )
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        (row_no >= 1 && row_no <= self.0.len()).then(|| self.0[row_no - 1].clone())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        (col_no >= 1 && col_no <= self.0[0].len())
            .then(|| self.0.iter().map(|row| row[col_no - 1]).collect())
    }
}
