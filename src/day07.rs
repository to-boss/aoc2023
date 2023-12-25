use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::util::read_input;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum CardValueWithJoker {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl From<char> for CardValueWithJoker {
    fn from(value: char) -> Self {
        match value {
            'J' => CardValueWithJoker::J,
            '2' => CardValueWithJoker::Two,
            '3' => CardValueWithJoker::Three,
            '4' => CardValueWithJoker::Four,
            '5' => CardValueWithJoker::Five,
            '6' => CardValueWithJoker::Six,
            '7' => CardValueWithJoker::Seven,
            '8' => CardValueWithJoker::Eight,
            '9' => CardValueWithJoker::Nine,
            'T' => CardValueWithJoker::T,
            'Q' => CardValueWithJoker::Q,
            'K' => CardValueWithJoker::K,
            'A' => CardValueWithJoker::A,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for CardValue {
    fn from(value: char) -> Self {
        match value {
            '2' => CardValue::Two,
            '3' => CardValue::Three,
            '4' => CardValue::Four,
            '5' => CardValue::Five,
            '6' => CardValue::Six,
            '7' => CardValue::Seven,
            '8' => CardValue::Eight,
            '9' => CardValue::Nine,
            'T' => CardValue::T,
            'J' => CardValue::J,
            'Q' => CardValue::Q,
            'K' => CardValue::K,
            'A' => CardValue::A,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
enum HandType {
    Fives,
    Fours,
    FullHouse,
    Threes,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_str_with_joker(s: &str) -> Self {
        let mut hash_map = HashMap::new();
        let mut joker_count = 0;

        for c in s.chars() {
            if c == 'J' {
                joker_count += 1;
            } else {
                hash_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        if hash_map.len() == 0 {
            return HandType::Fives;
        }

        let highest_char = hash_map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k)
            .unwrap();

        hash_map
            .entry(*highest_char)
            .and_modify(|e| *e += joker_count);

        match hash_map.len() {
            1 => HandType::Fives,
            2 if hash_map.iter().any(|(_, &v)| v == 4) => HandType::Fours,
            2 => HandType::FullHouse,
            3 if hash_map.iter().any(|(_, &v)| v == 3) => HandType::Threes,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("no clue which HandType for: {}", s),
        }
    }
}

impl FromStr for HandType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut hash_map = HashMap::new();
        for c in s.chars() {
            hash_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let hand_type = match hash_map.len() {
            1 => HandType::Fives,
            2 if hash_map.iter().any(|(_, &v)| v == 4) => HandType::Fours,
            2 => HandType::FullHouse,
            3 if hash_map.iter().any(|(_, &v)| v == 3) => HandType::Threes,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("no clue which HandType for: {}", s),
        };

        Ok(hand_type)
    }
}

#[derive(Debug)]
struct Hand {
    hand: String,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn from_str_with_joker(s: &str) -> anyhow::Result<Self> {
        let (hand, bid) = s.split_once(" ").unwrap();
        let hand_type = HandType::from_str_with_joker(hand);
        let hand = hand.to_string();
        let bid = bid.parse().unwrap();

        Ok(Hand {
            hand,
            hand_type,
            bid,
        })
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (hand, bid) = s.split_once(" ").unwrap();
        let hand_type = HandType::from_str(hand).unwrap();
        let hand = hand.to_string();
        let bid = bid.parse().unwrap();

        Ok(Hand {
            hand,
            hand_type,
            bid,
        })
    }
}

fn compare_equal_hands_with_joker(a: &str, b: &str, index: usize) -> Ordering {
    let a_char = a.chars().nth(index).unwrap();
    let b_char = b.chars().nth(index).unwrap();

    let a = CardValueWithJoker::from(a_char);
    let b = CardValueWithJoker::from(b_char);

    a.partial_cmp(&b).unwrap()
}

fn compare_equal_hands(a: &str, b: &str, index: usize) -> Ordering {
    let a_char = a.chars().nth(index).unwrap();
    let b_char = b.chars().nth(index).unwrap();

    let a = CardValue::from(a_char);
    let b = CardValue::from(b_char);

    a.partial_cmp(&b).unwrap()
}

fn solve2(s: &str) -> usize {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|l| Hand::from_str_with_joker(l.trim()).unwrap())
        .collect();

    hands.sort_by(|a, b| {
        let mut ord = b.hand_type.partial_cmp(&a.hand_type).unwrap();
        let mut index = 0;
        while let Ordering::Equal = ord {
            ord = compare_equal_hands_with_joker(&a.hand, &b.hand, index);
            index += 1;
        }
        ord
    });

    /*
    for (rank, hand) in hands.iter().enumerate() {
        let rank = rank + 1;
        println!("{} {:?}, {rank} * {}", hand.hand, hand.hand_type, hand.bid);
    }
    */

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

fn solve1(s: &str) -> usize {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|l| Hand::from_str(l.trim()).unwrap())
        .collect();

    hands.sort_by(|a, b| {
        let mut ord = b.hand_type.partial_cmp(&a.hand_type).unwrap();
        let mut index = 0;
        while let Ordering::Equal = ord {
            ord = compare_equal_hands(&a.hand, &b.hand, index);
            index += 1;
        }
        ord
    });

    /*
    for (rank, hand) in hands.iter().enumerate() {
        let rank = rank + 1;
        // println!("{} {:?}, {rank} * {}", hand.hand, hand.hand_type, hand.bid);
    }
    */

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

pub fn answer1() {
    let input = read_input(7);
    println!("day07 part1: {}", solve1(&input));
}

pub fn answer2() {
    let input = read_input(7);
    println!("day07 part2: {}", solve2(&input));
}

#[test]
fn test1() {
    let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    assert_eq!(6440, solve1(input));
}

#[test]
fn test2() {
    let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    assert_eq!(5905, solve2(input));
}
