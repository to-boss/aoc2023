use std::str::FromStr;

use crate::util::read_input;

pub fn answer2() {
    let input = read_input(2);
    let answer = solve2(&input);
    println!("day02 part1: {answer}");
}

fn solve2(input: &str) -> usize {
    let games: Vec<Game> = input
        .lines()
        .filter_map(|l| Game::from_str(l).ok())
        .collect();

    games.iter().fold(0, |acc, game| {
        let cubes = game.find_min_cubes();
        let cubes_power: usize = cubes.iter().product();
        acc + cubes_power
    })
}

pub fn answer1() {
    let input = read_input(2);
    let cubes = vec![12, 13, 14];
    let answer = solve1(&input, cubes);
    println!("day02 part1: {answer}");
}

fn solve1(input: &str, cubes: Vec<usize>) -> usize {
    let games: Vec<Game> = input
        .lines()
        .filter_map(|l| Game::from_str(l).ok())
        .collect();

    games.iter().fold(0, |acc, game| {
        if game.valid(&cubes) {
            acc + game.index
        } else {
            acc
        }
    })
}

#[derive(Debug)]
struct Game {
    index: usize,
    sets: Vec<Set>,
}

impl Game {
    fn valid(&self, cubes: &Vec<usize>) -> bool {
        self.sets.iter().all(|set| set.check(cubes))
    }

    fn find_min_cubes(&self) -> Vec<usize> {
        let first_set = self.sets.first().unwrap();
        let mut red = first_set.red;
        let mut green = first_set.green;
        let mut blue = first_set.blue;

        for set in self.sets.iter().skip(1) {
            if set.red > red {
                red = set.red;
            }
            if set.green > green {
                green = set.green;
            }
            if set.blue > blue {
                blue = set.blue;
            }
        }

        vec![red, green, blue]
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (game, rest) = s.split_once(":").unwrap();
        let (_, index) = game.trim().split_once(" ").unwrap();
        let index: usize = index.parse()?;

        let sets = rest
            .split(";")
            .filter_map(|s| Set::from_str(s).ok())
            .collect();

        Ok(Game { index, sets })
    }
}

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn check(&self, cubes: &Vec<usize>) -> bool {
        cubes[0] >= self.red && cubes[1] >= self.green && cubes[2] >= self.blue
    }
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;

        for color_pair in s.trim().split(", ") {
            let (num, color) = color_pair.split_once(" ").unwrap();
            let num: usize = num.parse()?;
            match color {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => panic!("Shouldnt happen"),
            }
        }

        Ok(Set { red, green, blue })
    }
}

#[test]
fn test1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    //red, blue, green
    let cubes = vec![12, 13, 14];
    assert_eq!(8, solve1(input, cubes));
}

#[test]
fn test2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(2286, solve2(input));
}
