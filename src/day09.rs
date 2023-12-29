use std::{collections::VecDeque, str::FromStr};

use crate::util::read_input;

#[derive(Debug)]
struct History {
    nums: VecDeque<i32>,
}

impl History {
    fn value_back(&self) -> i32 {
        let mut placeholders = vec![0];
        let mut diffs = vec![];

        let mut curr_diff: Vec<i32> = self
            .nums
            .iter()
            .enumerate()
            .take(self.nums.len() - 1)
            .map(|(i, n)| self.nums[i + 1] - n)
            .collect();
        diffs.push(curr_diff.clone());

        loop {
            curr_diff = curr_diff
                .iter()
                .enumerate()
                .take(curr_diff.len() - 1)
                .map(|(i, n)| curr_diff[i + 1] - n)
                .collect();
            diffs.push(curr_diff.clone());

            if curr_diff.iter().sum::<i32>() == 0 {
                break;
            }
        }

        for _ in 0..diffs.len() {
            let num = *diffs.pop().unwrap().last().unwrap();
            let num2 = placeholders.last().unwrap();
            let placeholder = num2 + num;
            placeholders.push(placeholder);
        }

        let num = *self.nums.back().unwrap();
        let num2 = placeholders.last().unwrap();
        let curr_placeholder = num2 + num;
        placeholders.push(curr_placeholder);

        *placeholders.last().unwrap()
    }

    fn value_front(&self) -> i32 {
        let mut placeholders = vec![0];
        let mut diffs = vec![];

        let mut curr_diff: Vec<i32> = self
            .nums
            .iter()
            .enumerate()
            .take(self.nums.len() - 1)
            .map(|(i, n)| self.nums[i + 1] - n)
            .collect();
        diffs.push(curr_diff.clone());

        loop {
            curr_diff = curr_diff
                .iter()
                .enumerate()
                .take(curr_diff.len() - 1)
                .map(|(i, n)| curr_diff[i + 1] - n)
                .collect();
            diffs.push(curr_diff.clone());

            if curr_diff.iter().sum::<i32>() == 0 {
                break;
            }
        }

        for _ in 0..diffs.len() {
            let first = *diffs.pop().unwrap().first().unwrap();
            let old_placeholder = placeholders.last().unwrap();
            let placeholder = first - old_placeholder;
            placeholders.push(placeholder);
        }

        let first = *self.nums.front().unwrap();
        let old_placeholder = placeholders.last().unwrap();
        let placeholder = first - old_placeholder;
        placeholders.push(placeholder);

        *placeholders.last().unwrap()
    }
}

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let nums = s
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(History { nums })
    }
}

#[derive(Debug)]
struct Report {
    histories: Vec<History>,
}

impl Report {
    fn sum_back(&self) -> i32 {
        self.histories
            .iter()
            .map(|history| history.value_back())
            .sum()
    }

    fn sum_front(&self) -> i32 {
        self.histories
            .iter()
            .map(|history| history.value_front())
            .sum()
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let histories = s
            .lines()
            .map(|line| History::from_str(line).unwrap())
            .collect();

        Ok(Report { histories })
    }
}

fn solve1(s: &str) -> i32 {
    let report = Report::from_str(s).unwrap();
    report.sum_back()
}

fn solve2(s: &str) -> i32 {
    let report = Report::from_str(s).unwrap();
    report.sum_front()
}

pub fn answer1() {
    let input = read_input(9);
    println!("day09 part1: {}", solve1(&input));
}

pub fn answer2() {
    let input = read_input(9);
    println!("day09 part2: {}", solve2(&input));
}

#[test]
fn test1() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    assert_eq!(114, solve1(input));
}

#[test]
fn test2() {
    let input = "10 13 16 21 30 45";
    let h = History::from_str(input).unwrap();
    assert_eq!(5, h.value_front());
}

#[test]
fn test3() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    assert_eq!(2, solve2(input));
}
