use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u64 {
    let id_range: Vec<String> = input[0].split(",").map(|s: &str| s.to_string()).collect();
    let mut total: u64 = 0;

    for range in id_range.iter() {
        if let Some((start, end)) = range.split_once("-") {
            let start_idx: u64 = start.parse().unwrap();
            let end_idx: u64 = end.parse().unwrap();
            for id in start_idx..=end_idx {
                let id_str = id.to_string();
                let digits = id_str.len();
                if digits.rem_euclid(2) == 0 {
                    let (first, last) = id_str.split_at(digits / 2);
                    if first.to_string() == last.to_ascii_lowercase() {
                        total += id;
                    }
                }
            }
        }
    }

    info!("Total: {}", total);
    total
}

fn check_repeating(id: String) -> bool {
    let mut repeating: bool = false;
    let mut target: String = "".to_string();
    for (i, c) in id.chars().enumerate() {
        target.push(c);
        if (id.len() % (i + 1)) == 0 && i + 1 != id.len() {
            repeating = true;
            for idx in 0..(id.len() / (i + 1)) {
                let curr: &str = &id[idx * (i + 1)..idx * (i + 1) + i + 1];
                if target != curr {
                    repeating = false
                }
            }
            if repeating == true {
                return repeating;
            }
        }
    }
    repeating
}

fn part_2(input: Vec<String>) -> u64 {
    let id_range: Vec<String> = input[0].split(",").map(|s: &str| s.to_string()).collect();
    let mut total: u64 = 0;

    for range in id_range.iter() {
        if let Some((start, end)) = range.split_once("-") {
            let start_idx: u64 = start.parse().unwrap();
            let end_idx: u64 = end.parse().unwrap();
            for id in start_idx..=end_idx {
                let id_str = id.to_string();
                if check_repeating(id_str.clone()) {
                    debug!("Found repeating: {}", id_str);
                    total += id
                }
            }
        }
    }

    info!("Total: {}", total);
    total
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let args = Args::parse();
    info!("File: {}", args.input);

    match args.part {
        Some(1) => {
            info!("-- Executing Part 1 --");
            if let Ok(input) = read_lines(&args.input) {
                part_1(input);
            }
        }
        Some(2) => {
            info!("-- Executing Part 2 --");
            if let Ok(input) = read_lines(&args.input) {
                part_2(input);
            }
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}

#[test]
fn part1_test() {
    if let Ok(input) = read_lines("./inputs/day02/test.txt") {
        assert_eq!(part_1(input), 1227775554);
    }
}

#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day02/input.txt") {
        assert_eq!(part_1(input), 16793817782);
    }
}

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day02/test.txt") {
        assert_eq!(part_2(input), 4174379265);
    }
}

#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day02/input.txt") {
        assert_eq!(part_2(input), 27469417404);
    }
}
