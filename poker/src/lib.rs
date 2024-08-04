use std::collections::HashMap;

const JACK: u8 = 11;
const QUEEN: u8 = 12;
const KING: u8 = 13;
const ACE: u8 = 14;

const TWO_OF_KIND_IDX: usize = 1;
const THREE_OF_KIND_IDX: usize = 2;
const FOUR_OF_KIND_IDX: usize = 3;
const FIVE_OF_KIND_IDX: usize = 4;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
struct Card {
    value: u8,
    suit: u8,
}

type Hand = Vec<Card>;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
enum Rank {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfKind = 7,
    StraightFlush = 8,
    FiveOfKind = 9,
}

#[derive(PartialEq, PartialOrd, Ord, Eq)]
struct HandValue {
    rank: Rank,
    tie_breaker: Vec<u8>,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands_vec: Vec<Hand> = hands.iter().map(|&hand| parse_hand(hand)).collect();
    let hand_values: Vec<HandValue> = hands_vec.iter().map(|hand| get_hand_value(hand)).collect();
    let best_hand_value: &HandValue = hand_values.iter().max().unwrap();
    Vec::from_iter(
        hand_values
            .iter()
            .enumerate()
            .filter(|&(_, hand_value)| hand_value == best_hand_value)
            .map(|(n, _)| hands[n])
            .collect::<Vec<&'a str>>(),
    )
}

fn parse_hand(s: &str) -> Hand {
    let mut hand: Hand = s.split_whitespace().map(parse_card).collect();
    hand.sort_unstable_by(|a, b| b.cmp(a));
    hand
}

fn parse_card(s: &str) -> Card {
    let (value_str, suit) = s.split_at(s.len() - 1);
    let value: u8 = match value_str {
        "A" => ACE,
        "J" => JACK,
        "Q" => QUEEN,
        "K" => KING,
        v => v.parse::<u8>().unwrap(),
    };

    Card {
        value: value,
        suit: suit.as_bytes()[0],
    }
}

fn get_hand_value(hand: &Hand) -> HandValue {
    let mut card_values: Vec<u8> = hand.iter().map(|card| card.value).collect();
    if card_values == [ACE, 5, 4, 3, 2] {
        card_values = vec![5, 4, 3, 2, 1]
    };
    let rank: Rank;
    let card_dist = get_card_distribution(&card_values);
    let tie_breaker: Vec<u8> = card_dist
        .iter()
        .rev()
        .fold(Vec::<u8>::new(), |mut acc, val| {
            acc.extend(val);
            acc
        });
    if !card_dist[FIVE_OF_KIND_IDX].is_empty() {
        rank = Rank::FiveOfKind
    } else if !card_dist[FOUR_OF_KIND_IDX].is_empty() {
        rank = Rank::FourOfKind
    } else {
        let flush = hand[1..].iter().all(|card| card.suit == hand[0].suit);
        let straight = card_values.windows(2).all(|w| w[0] == w[1] + 1);
        rank = match (flush, straight) {
            (true, true) => Rank::StraightFlush,
            (true, false) => Rank::Flush,
            (false, true) => Rank::Straight,
            (false, false) => {
                match (
                    card_dist[THREE_OF_KIND_IDX].len(),
                    card_dist[TWO_OF_KIND_IDX].len(),
                ) {
                    (1, 1) => Rank::FullHouse,
                    (1, 0) => Rank::ThreeOfKind,
                    (0, 2) => Rank::TwoPair,
                    (0, 1) => Rank::OnePair,
                    (_, _) => Rank::HighCard,
                }
            }
        }
    }

    HandValue {
        rank: rank,
        tie_breaker: tie_breaker,
    }
}

fn get_card_distribution(card_values: &Vec<u8>) -> Vec<Vec<u8>> {
    let dist_by_value = card_values
        .iter()
        .fold(HashMap::<u8, u8>::new(), |mut acc, &value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        });
    let mut dist_by_count: Vec<Vec<u8>> = vec![vec![]; card_values.len()];
    for (value, count) in dist_by_value {
        dist_by_count[count as usize - 1].push(value);
    }

    for count in 0..card_values.len() {
        dist_by_count[count].sort_unstable_by(|a, b| b.cmp(a));
    }

    dist_by_count
}
