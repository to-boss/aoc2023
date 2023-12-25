use std::{ops::Range, str::FromStr};

use crate::util::read_input;

#[derive(Debug)]
struct ConversionMap {
    from: String,
    to: String,
    src_ranges: Vec<Range<u128>>,
    dst_ranges: Vec<Range<u128>>,
    range_lens: Vec<u128>,
}

impl ConversionMap {
    fn new(from: &str, to: &str) -> Self {
        ConversionMap {
            from: from.to_owned(),
            to: to.to_owned(),
            src_ranges: Vec::new(),
            dst_ranges: Vec::new(),
            range_lens: Vec::new(),
        }
    }

    fn insert(&mut self, conversion: Conversion) {
        let dest_end = conversion.dest_start + conversion.range_len;
        let dest_range = conversion.dest_start..dest_end;

        let source_end = conversion.source_start + conversion.range_len;
        let source_range = conversion.source_start..source_end;

        self.src_ranges.push(source_range);
        self.dst_ranges.push(dest_range);
        self.range_lens.push(conversion.range_len);
    }

    fn get_range(&self, range: Range<u128>) -> Range<u128> {
        for (i, src_range) in self.src_ranges.iter().enumerate() {
            if src_range.start >= range.start
                && range.end <= src_range.end
                && range.end >= src_range.start
            {
                println!("{:?} is in {:?}", range, src_range);
                let diff = range.end - src_range.start;
                if diff == 0 {
                    return self.dst_ranges[i].clone();
                } else {
                    return (self.dst_ranges[i].start + diff)..(self.dst_ranges[i].end + diff);
                }
            }
        }
        range
    }

    fn get(&self, n: u128) -> u128 {
        for (i, range) in self.src_ranges.iter().enumerate() {
            if range.contains(&n) {
                let diff = n - range.start;
                if diff == 0 {
                    return self.dst_ranges[i].start;
                } else {
                    return self.dst_ranges[i].start + diff;
                }
            }
        }

        n
    }
}

#[derive(Debug)]
struct Conversion {
    dest_start: u128,
    source_start: u128,
    range_len: u128,
}

impl FromStr for Conversion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (dest_start, rest) = s.split_once(" ").unwrap();
        let (source_start, range_len) = rest.split_once(" ").unwrap();

        Ok(Conversion {
            dest_start: dest_start.parse().unwrap(),
            source_start: source_start.parse().unwrap(),
            range_len: range_len.parse().unwrap(),
        })
    }
}

fn preprocess(input: &str) -> (Vec<u128>, Vec<ConversionMap>) {
    let mut seeds: Vec<u128> = vec![];
    let mut maps: Vec<ConversionMap> = vec![];
    let mut current_map = None;
    let mut in_map = false;

    let lines: Vec<_> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();

        // println!("line {i} ({in_map}): {}", line);

        if i == 0 {
            let (_, list) = line.split_once(":").unwrap();
            seeds = list
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            continue;
        }

        if i == 1 {
            continue;
        }

        if line.is_empty() {
            maps.push(current_map.take().unwrap());
            in_map = false;
            continue;
        }

        if in_map == false {
            in_map = true;
            let (left, _) = line.split_once(" ").unwrap();
            let (from, rest) = left.split_once("-").unwrap();
            let (_, to) = rest.split_once("-").unwrap();
            current_map = Some(ConversionMap::new(from, to));
        } else {
            let conversion = Conversion::from_str(line).unwrap();
            if let Some(ref mut map) = current_map {
                map.insert(conversion);
            }
        }

        if i == lines.len() - 1 {
            maps.push(current_map.take().unwrap());
        }
    }

    (seeds, maps)
}

fn solve1(input: &str) -> u128 {
    let (mut seeds, maps) = preprocess(input);
    for map in maps.iter() {
        for seed in seeds.iter_mut() {
            *seed = map.get(*seed);
        }
    }
    *seeds.iter().min().unwrap()
}

fn solve2(input: &str) -> u128 {
    let (seeds, maps) = preprocess(input);
    let mut seeds: Vec<Range<u128>> = seeds
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1] - 1))
        .collect();
    println!("{:?}", seeds);

    for map in maps.iter() {
        for seed_range in seeds.iter_mut() {
            let start = map.get(seed_range.start);
            let end = map.get(seed_range.end);
            //println!("new_range: {:?}", start..end);
            *seed_range = start..end;
        }
    }

    let mut number = seeds.iter().map(|r| r.start).min().unwrap();
    println!(" seeds at the end: {:?}", seeds);
    println!("lowest number start: {:?}", number);
    for map in maps.iter() {
        number = map.get(number);
        println!("corresponds to: {} {:?}", map.to, number);
    }
    number
    //seeds.iter().map(|r| r.start).min().unwrap()
}

pub fn answer1() {
    let input = read_input(5);
    println!("day05 part1: {}", solve1(&input));
}

pub fn answer2() {
    let input = read_input(5);
    println!("day05 part2: {}", solve2(&input));
}

#[test]
fn test1() {
    let input = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    let (seeds, maps) = preprocess(input);
    assert_eq!(maps[0].get(seeds[0]), 81);
    assert_eq!(maps[0].get(seeds[1]), 14);
    assert_eq!(maps[0].get(seeds[2]), 57);
    assert_eq!(maps[0].get(seeds[3]), 13);

    assert_eq!(35, solve1(input));
}

#[test]
fn test2() {
    let input = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    assert_eq!(46, solve2(input));
}
