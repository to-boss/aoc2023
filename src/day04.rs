use std::str::FromStr;

use crate::util::read_input;

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    winning: Vec<usize>,
    owned: Vec<usize>,
}

impl Card {
    fn matching(&self) -> usize {
        self.owned
            .iter()
            .map(|n| if self.winning.contains(n) { 1 } else { 0 })
            .sum()
    }

    fn worth(&self) -> usize {
        self.owned.iter().fold(0, |acc, n| {
            let contained = self.winning.contains(n);
            if contained && acc == 0 {
                acc + 1
            } else if contained && acc > 0 {
                acc * 2
            } else {
                acc
            }
        })
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (card_nr, lists) = s.split_once(":").unwrap();
        let (_, index) = card_nr.split_once(" ").unwrap();
        let index = index.trim().parse().unwrap();
        let (winning, owned) = lists.split_once("|").unwrap();

        let winning = winning
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let owned = owned
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Card {
            index,
            winning,
            owned,
        })
    }
}

fn solve1(input: &str) -> usize {
    let cards: Vec<_> = input
        .lines()
        .map(|line| Card::from_str(line.trim()).unwrap())
        .collect();

    /*
    for card in &cards {
        println!("{:?} {}", card, card.worth());
    }
    */

    cards.iter().map(|c| c.worth()).sum()
}

pub fn answer1() {
    let input = read_input(4);
    println!("day04 part1: {}", solve1(&input));
}

pub fn answer2() {
    let input = read_input(4);
    println!("day04 part2: {}", solve2(&input));
}

fn solve2(input: &str) -> usize {
    let cards: Vec<_> = input
        .lines()
        .map(|line| Card::from_str(line.trim()).unwrap())
        .collect();

    let mut amount_cards: Vec<usize> = (0..cards.len()).map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        let matching = card.matching();
        //println!("Card {} has {matching} matching numbers.", card.index);

        for _ in 0..amount_cards[i] {
            for n in 0..matching {
                if let Some(num) = amount_cards.get_mut(card.index + n) {
                    //println!("Won copy of card {}", card.index + n + 1);
                    *num += 1;
                }
            }
        }
        //println!("{:?}", amount_cards);
    }

    amount_cards.iter().sum()
}

#[test]
fn test2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(30, solve2(input));
}

#[test]
fn test1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards: Vec<_> = input
        .lines()
        .map(|line| Card::from_str(line.trim()).unwrap())
        .collect();

    let sum: usize = cards.iter().map(|c| c.worth()).sum();
    assert_eq!(13, sum);
}
