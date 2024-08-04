#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    index: usize,
    frame: u8,
    roll_num: u8,
    pins_left: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: vec![0; 21],
            pins_left: 10,
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame >= 10 {
            return Err(Error::GameComplete);
        }

        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Adjust number of pins left. Reset if all pins knocked down
        self.pins_left -= pins;
        if self.pins_left == 0 || (self.roll_num == 2 && self.frame < 9) {
            self.pins_left = 10;
        }

        self.roll_num += 1;
        if self.roll_num == 1 {
            // First roll: Next frame if strike and not last frame
            if self.frame < 9 && pins == 10 {
                self.roll_num = 0;
                self.frame += 1;
            }
        } else if self.roll_num == 2 {
            // Second roll: Next frame if not last frame and reset pins left
            if self.frame < 9 {
                self.frame += 1;
                self.roll_num = 0;
                self.pins_left = 10;
            // Game over if less than 10 pins in last frame
            } else if self.rolls[self.index - 1] + pins < 10 {
                self.frame += 1;
            }
        } else {
            // Game over if third roll in last frame
            self.frame += 1;
        }

        self.rolls[self.index] = pins;
        self.index += 1;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame < 10 {
            return None;
        }

        let mut total: u16 = 0;
        let mut index: usize = 0;
        for _frame in 0..10usize {
            // Add these two rolls to total and advance two rolls
            let frame_total = self.rolls[index] + self.rolls[index + 1];
            total += frame_total;
            index += 2;
            if self.rolls[index - 2] == 10 || frame_total == 10 {
                // Strike or spare: Add next roll to total
                total += self.rolls[index];

                // Go back one roll if strike
                if self.rolls[index - 2] == 10 {
                    index -= 1;
                }
            }
        }

        Some(total)
    }
}
