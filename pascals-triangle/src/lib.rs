use std::iter::{once, successors};

pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(
            successors(Some(vec![1u32]), |row| {
                Some(
                    once(1)
                        .chain(row.windows(2).map(|w| w[0] + w[1]))
                        .chain(once(1))
                        .collect(),
                )
            })
            .take(row_count as usize)
            .collect(),
        )
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
