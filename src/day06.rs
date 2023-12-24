use crate::util::read_input;

#[derive(Debug)]
struct Race {
    lasts: usize,
    distance: usize,
}

fn race(s: &str) -> Race {
    let (time, distance) = s.split_once("\n").unwrap();
    let (_, nums) = time.split_once(": ").unwrap();
    let mut lasts = String::new();
    for n in nums.split_whitespace() {
        lasts.push_str(n);
    }

    let (_, nums) = distance.split_once(": ").unwrap();
    let mut distance = String::new();
    for n in nums.split_whitespace() {
        distance.push_str(n);
    }

    Race {
        lasts: lasts.parse().unwrap(),
        distance: distance.parse().unwrap(),
    }
}

fn races(s: &str) -> Vec<Race> {
    let (times, distances) = s.split_once("\n").unwrap();

    times
        .split_whitespace()
        .zip(distances.split_whitespace())
        .skip(1)
        .map(|(t, d)| Race {
            lasts: t.parse().unwrap(),
            distance: d.parse().unwrap(),
        })
        .collect()
}

struct Boat {
    inital_speed: usize,
    acceleration: usize,
}

impl Boat {
    fn simulate(&mut self, race: &Race, hold_button: usize) -> bool {
        let speed = self.inital_speed + self.acceleration * hold_button;
        let time_left = race.lasts - hold_button;
        let moved = time_left * speed;

        time_left < race.lasts && moved > race.distance
    }
}

fn solve1(s: &str) -> usize {
    let races = races(s);
    let mut boat = Boat {
        inital_speed: 0,
        acceleration: 1,
    };

    races
        .iter()
        .map(|race| {
            (0..race.lasts + 1)
                .into_iter()
                .map(|n| boat.simulate(race, n) as usize)
                .sum::<usize>()
        })
        .product()
}

fn solve2(s: &str) -> usize {
    let race = race(s);
    let mut boat = Boat {
        inital_speed: 0,
        acceleration: 1,
    };

    (0..race.lasts + 1)
        .into_iter()
        .map(|n| boat.simulate(&race, n) as usize)
        .sum()
}

pub fn answer1() {
    let input = read_input(6);
    println!("day06 part1: {}", solve1(&input));
}

pub fn answer2() {
    let input = read_input(6);
    println!("day06 part2: {}", solve2(&input));
}

#[test]
fn test1() {
    let input = "Time:      7  15   30
        Distance:  9  40  200";
    assert_eq!(288, solve1(input));
}

#[test]
fn test2() {
    let input = "Time:      7  15   30
        Distance:  9  40  200";
    assert_eq!(71503, solve2(input));
}
