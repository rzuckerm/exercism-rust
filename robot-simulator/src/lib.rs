#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;
const DIRECTIONS: &[Direction; 4] = &[North, East, South, West];
const ADVANCES: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = DIRECTIONS[(self.direction as usize + 1) % 4];
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = DIRECTIONS[(self.direction as usize + 3) % 4];
        self
    }

    pub fn advance(mut self) -> Self {
        self.x += ADVANCES[self.direction as usize].0;
        self.y += ADVANCES[self.direction as usize].1;
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot, // Does not compute!
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
