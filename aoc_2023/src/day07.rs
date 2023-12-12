use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn score(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn from_values(v: &Vec<u8>) -> Self {
        match v.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if v.contains(&4) {
                    HandType::FourOfAKind
                } else if v.contains(&3) && v.contains(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            3 => {
                if v.contains(&3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    pub cards: [u8; 5],
    pub hand: HandType,
    pub bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand.score().cmp(&other.hand.score()) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    if a < b {
                        return Some(Ordering::Less);
                    } else if b < a {
                        return Some(Ordering::Greater);
                    }
                }
                Some(Ordering::Equal)
            }
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn card_value(s: char) -> u8 {
    match s {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unexpected Card {}", s),
    }
}

fn process_hand(cards: [u8; 5], bid: usize, p2: bool) -> Hand {
    let mut occurances = HashMap::<u8, u8>::new();
    for c in cards {
        occurances.entry(c).or_insert(0).add_assign(1);
    }
    let j_count = *occurances.get(&1).unwrap_or(&0);
    if p2 {
        occurances.remove(&1);
    }
    let mut values = occurances.values().map(|v| *v).collect::<Vec<u8>>();
    if p2 {
        values.sort_unstable();
        if values.is_empty() {
            values = vec![0];
        }
        values.last_mut().unwrap().add_assign(j_count);
    }
    let hand_type = HandType::from_values(&values);
    Hand {
        cards,
        hand: hand_type,
        bid,
    }
}

#[allow(dead_code)]
pub fn day07(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut hands = Vec::<Hand>::new();
    let mut hands_p2 = Vec::<Hand>::new();
    for line in input.split("\n") {
        let (hand, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let cards: [u8; 5] = hand
            .chars()
            .map(|c| card_value(c))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let cards_p2: [u8; 5] = cards.map(|c| match c {
            11 => 1,
            _ => c,
        });
        hands.push(process_hand(cards, bid, false));
        hands_p2.push(process_hand(cards_p2, bid, true));
    }
    hands.sort_unstable();
    hands_p2.sort_unstable();
    let p1: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid).sum();
    let p2: usize = hands_p2
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid).sum();
    (p1.to_string(), p2.to_string())
}
