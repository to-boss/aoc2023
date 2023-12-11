use std::str::FromStr;

use crate::util::read_input;

#[derive(Debug)]
enum Cell {
    Num(usize),
    Dot,
    Symbol(char),
}

impl Cell {
    fn inner_num(&self) -> Option<usize> {
        match self {
            Cell::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Dot,
            c if c.is_digit(10) => Cell::Num(c.to_digit(10).unwrap() as usize),
            _ => Self::Symbol(c),
        }
    }
}

#[derive(Debug)]
struct Number {
    number: String,
    positions: Vec<(usize, usize)>,
    has_symbol_neighbour: bool,
}

impl Number {
    fn push(&mut self, x: usize, y: usize, n: usize) {
        self.number.push_str(&format!("{n}"));
        self.positions.push((x, y));
    }

    fn number(&self) -> usize {
        self.number.parse().unwrap()
    }

    fn count(&self) -> Option<usize> {
        if self.has_symbol_neighbour {
            Some(self.number.parse().unwrap())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Symbol {
    character: char,
    position: (usize, usize),
}

#[derive(Debug)]
struct Engine {
    schematic: Vec<Vec<Cell>>,
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Engine {
    fn gear_ratio(&self) -> usize {
        let mut ratio = 0;
        for symbol in self.symbols.iter() {
            if symbol.character == '*' {
                let mut hits = vec![];
                let (x, y) = symbol.position;
                for num in self.numbers.iter() {
                    for (num_x, num_y) in num.positions.iter() {
                        if x.abs_diff(*num_x) <= 1 && y.abs_diff(*num_y) <= 1 {
                            hits.push(num.number());
                            println!(
                                "Hit at: [{num_x}, {num_y}], added {}, hits={}",
                                num.number(),
                                hits.len()
                            );
                            break;
                        }
                    }
                    if hits.len() == 2 {
                        ratio += hits.iter().product::<usize>();
                        break;
                    }
                }
            }
        }
        ratio
    }

    fn check_numbers_for_symbols(&mut self) {
        for number in self.numbers.iter_mut() {
            for (x, y) in number.positions.iter() {
                let mut has_symbol_neighbour: bool = false;
                let x = *x as i32;
                let y = *y as i32;
                for i in -1..2 {
                    for j in -1..2 {
                        let new_x = (x + i) as usize;
                        let new_y = (y + j) as usize;

                        if let Some(row) = self.schematic.get(new_y) {
                            if let Some(cell) = row.get(new_x) {
                                if let Cell::Symbol(_) = cell {
                                    has_symbol_neighbour = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if has_symbol_neighbour {
                    number.has_symbol_neighbour = true;
                    break;
                }
            }
        }
    }

    fn part_number_sum(&self) -> usize {
        self.numbers.iter().filter_map(|n| n.count()).sum()
    }

    fn has_neighbour_symbol(&self, x: usize, y: usize) -> bool {
        let x = x as i32;
        let y = y as i32;

        for i in -1..2 {
            for j in -1..2 {
                let new_x = (x + i) as usize;
                let new_y = (y + j) as usize;

                if let Some(row) = self.schematic.get(new_y) {
                    if let Some(cell) = row.get(new_x) {
                        if let Cell::Symbol(_) = cell {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut numbers: Vec<Number> = vec![];
        let mut schematic = vec![];
        let mut symbols = vec![];

        let mut last_was_number = false;
        for (y, line) in s.lines().enumerate() {
            let mut cells = vec![];
            for (x, c) in line.trim().chars().enumerate() {
                let cell = Cell::from_char(c);
                match (&cell, last_was_number) {
                    (Cell::Num(n), true) => {
                        numbers.last_mut().unwrap().push(x, y, *n);
                    }
                    (Cell::Num(n), false) => {
                        last_was_number = true;
                        let mut number = Number {
                            number: String::new(),
                            positions: vec![],
                            has_symbol_neighbour: false,
                        };
                        number.push(x, y, *n);
                        numbers.push(number);
                    }
                    (Cell::Symbol(_), _) => {
                        last_was_number = false;
                        symbols.push(Symbol {
                            character: c,
                            position: (x, y),
                        });
                    }
                    (_, _) => last_was_number = false,
                }
                cells.push(cell);
            }
            schematic.push(cells);
        }

        Ok(Engine {
            schematic,
            numbers,
            symbols,
        })
    }
}

fn solve1(input: &str) -> usize {
    let mut engine = Engine::from_str(input).unwrap();
    engine.check_numbers_for_symbols();
    engine.part_number_sum()
}

fn solve2(input: &str) -> usize {
    let engine = Engine::from_str(input).unwrap();
    engine.gear_ratio()
}

pub fn answer1() {
    let input = read_input(3);
    println!("day03 part1: {}", solve1(input.trim()));
}

pub fn answer2() {
    let input = read_input(3);
    println!("day03 part2: {}", solve2(input.trim()));
}

#[test]
fn test1() {
    let input = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";
    assert_eq!(4361, solve1(input));
}

#[test]
fn test2() {
    let input = "467..114..
    ...*......
    ..35...633
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."
        .trim();
    assert_eq!(4361, solve1(input));
}

#[test]
fn test3() {
    let input = "
        12.......*..
        +.........34
        .......-12..
        ..78........
        ..*....60...
        78..........
        .......23...
        ....90*12...
        ............
        2.2......12.
        .*.........*
        1.1.......56
    ";

    assert_eq!(solve1(&input), 413);
}

#[test]
fn test4() {
    let input = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";
    assert_eq!(467835, solve2(input));
}
