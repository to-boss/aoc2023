use std::{collections::HashMap, str::FromStr};

use crate::util::read_input;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct MapDirection {
    current: String,
    left: String,
    right: String,
}

impl MapDirection {
    fn take(&self, direction: &Direction) -> String {
        match direction {
            Direction::Right => self.right.clone(),
            Direction::Left => self.left.clone(),
        }
    }

    fn last_is_z(&self) -> bool {
        self.current.chars().last().unwrap() == 'Z'
    }
}

impl FromStr for MapDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (current, rest) = s.split_once(" = ").unwrap();
        let current = current.to_string();

        let (left, right) = rest.split_once(", ").unwrap();
        let left = (&left[1..]).to_string();
        let right = (&right[..right.len() - 1]).to_string();

        Ok(MapDirection {
            current,
            left,
            right,
        })
    }
}

fn solve1(s: &str) -> usize {
    let (directions, rest) = s.split_once("\n\n").unwrap();

    let directions: Vec<Direction> = directions.chars().map(|c| Direction::from(c)).collect();

    let mut map_directions = HashMap::new();
    for line in rest.lines() {
        let line = line.trim();
        let map_dir = MapDirection::from_str(line).unwrap();
        map_directions.insert(map_dir.current.clone(), map_dir);
    }

    let mut count = 0;
    let mut current_map_dir = map_directions.get("AAA").unwrap();
    for dir in directions.iter().cycle() {
        count += 1;
        let s = current_map_dir.take(dir);
        current_map_dir = map_directions.get(&s).unwrap();
        if current_map_dir.current == "ZZZ" {
            break;
        }
    }

    count
}

fn solve2(s: &str) -> usize {
    let (directions, rest) = s.split_once("\n\n").unwrap();

    let directions: Vec<Direction> = directions.chars().map(|c| Direction::from(c)).collect();

    let mut map_directions = HashMap::new();
    for line in rest.lines() {
        let line = line.trim();
        let map_dir = MapDirection::from_str(line).unwrap();
        map_directions.insert(map_dir.current.clone(), map_dir);
    }

    let mut current_map_dirs: Vec<&MapDirection> = map_directions
        .iter()
        .filter_map(|(k, v)| {
            if k.chars().last().unwrap() == 'A' {
                Some(v)
            } else {
                None
            }
        })
        .collect();

    // This cycles, would need least common multiple to be efficient enough,
    // probably needs external library to be solved
    let mut count = 0;
    for dir in directions.iter().cycle() {
        for curr in current_map_dirs.iter_mut() {
            let s = curr.take(dir);
            *curr = map_directions.get(&s).unwrap();
        }
        count += 1;
        if current_map_dirs.iter().all(|&m| m.last_is_z()) {
            break;
        }
    }

    count
}

pub fn answer1() {
    let input = read_input(8);
    println!("day08 part1: {}", solve1(&input));
}

/*
pub fn answer2() {
    let input = read_input(8);
    println!("day08 part2: {}", solve2(&input));
}
*/

#[test]
fn test1() {
    let input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(2, solve1(input));
}

#[test]
fn test2() {
    let input = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(6, solve1(input));
}

/*
#[test]
fn test3() {
    let input = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    assert_eq!(6, solve2(input));
}
*/
