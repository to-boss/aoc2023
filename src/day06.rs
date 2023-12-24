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
    // millimeters per second
    inital_speed: usize,
    // millimeters per second
    acceleration: usize,
}

impl Boat {
    fn simulate(&mut self, race: &Race, hold_button: usize) -> (usize, usize, bool) {
        let speed = self.inital_speed + self.acceleration * hold_button;
        let time_left = race.lasts - hold_button;
        let moved = time_left * speed;
        let won = time_left < race.lasts && moved > race.distance;

        (time_left, moved, won)
    }
}

fn solve1(s: &str) -> usize {
    let races = races(s);
    let mut boat = Boat {
        inital_speed: 0,
        acceleration: 1,
    };

    let mut nums = vec![];
    for race in races.iter() {
        let mut won_races = 0;
        for n in 0..race.lasts + 1 {
            let (time_left, moved, won) = boat.simulate(race, n);
            //println!("n: {n}, time_left: {time_left}, moved: {moved}, won: {won}");
            if won {
                won_races += 1;
            }
        }
        nums.push(won_races);
    }
    nums.iter().product()
}

fn solve2(s: &str) -> usize {
    let race = race(s);
    let mut boat = Boat {
        inital_speed: 0,
        acceleration: 1,
    };

    let mut won_races = 0;
    for n in 0..race.lasts + 1 {
        let (time_left, moved, won) = boat.simulate(&race, n);
        //println!("n: {n}, time_left: {time_left}, moved: {moved}, won: {won}");
        if won {
            won_races += 1;
        }
    }
    won_races
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
