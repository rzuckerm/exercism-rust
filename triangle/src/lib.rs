use std::ops::Add;

pub struct Triangle<T>(T, T, T);

impl<T: PartialOrd + Add<Output = T> + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let mut sides: Vec<T> = Vec::from(sides);
        sides.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        match sides[0] + sides[1] > sides[2] {
            true => Some(Triangle(sides[0], sides[1], sides[2])),
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2
    }
}
