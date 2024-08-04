#[derive(Debug, Clone, Copy)]
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    LittleStraight = 9,
    BigStraight = 10,
    Choice = 11,
    Yacht = 12,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut sd = dice;
    sd.sort_unstable();
    match category {
        Category::FullHouse => (sd[0] != sd[4]
            && ((sd[0] == sd[1] && sd[2] == sd[4]) || (sd[0] == sd[2] && sd[3] == sd[4])))
            .then_some(dice.iter().sum()),
        Category::FourOfAKind => (sd[0] == sd[3] || sd[1] == sd[4]).then_some(4 * sd[1]),
        Category::LittleStraight => (sd == [1, 2, 3, 4, 5]).then_some(30),
        Category::BigStraight => (sd == [2, 3, 4, 5, 6]).then_some(30),
        Category::Choice => Some(dice.iter().sum()),
        Category::Yacht => (sd[0] == sd[4]).then_some(50),
        _ => Some(dice.iter().filter(|&&d| d == category as u8).sum()),
    }
    .unwrap_or(0)
}
