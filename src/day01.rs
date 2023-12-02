use crate::util::read_input;

fn digit_to_num(digit: &str) -> usize {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Can only match 1-9"),
    }
}

fn solve2(input: &str) -> usize {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    assert_eq!(digits.len(), 9);

    let mut calibration_values: Vec<usize> = vec![];
    for line in input.lines() {
        let mut nums: Vec<(usize, usize)> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_numeric())
            .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
            .collect();

        // could be a fold
        let mut string_nums = vec![];
        for digit in &digits {
            if let Some(index) = line.find(digit) {
                string_nums.push((index, digit_to_num(digit)));
            }
            if let Some(index) = line.rfind(digit) {
                string_nums.push((index, digit_to_num(digit)));
            }
        }

        nums.append(&mut string_nums);
        nums.sort_by_key(|t| t.0);

        let first = nums.first().unwrap().1;
        let last = nums.last().unwrap().1;
        let value_str = format!("{first}{last}");
        let num = value_str.parse().unwrap();

        calibration_values.push(num);
    }

    calibration_values.iter().sum()
}

pub fn answer2() {
    let input = read_input(1);
    println!("day01 part2: {}", solve2(&input));
}

fn solve1(input: &str) -> usize {
    let calibration_values: Vec<usize> = input
        .lines()
        .filter_map(|l| {
            let mut buffer = String::with_capacity(2);
            let first = l.chars().find(|c| c.is_numeric());
            let last = l.chars().rev().find(|c| c.is_numeric());
            if let (Some(first), Some(last)) = (first, last) {
                buffer.push(first);
                buffer.push(last);
            }
            buffer.parse().ok()
        })
        .collect();

    calibration_values.iter().sum()
}

pub fn answer1() {
    let input = read_input(1);
    println!("day01 part1: {}", solve1(&input));
}

#[test]
fn test1() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(142, solve1(input));
}

#[test]
fn test2() {
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    assert_eq!(281, solve2(input));
}

#[test]
fn test3() {
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    eighthree
    sevenine
    oneight";
    assert_eq!(281 + 83 + 79 + 18, solve2(input));
}
