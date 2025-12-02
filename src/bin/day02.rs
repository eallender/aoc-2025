use std::ascii::AsciiExt;

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
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}

#[test]
fn part1_test1() {
    if let Ok(input) = read_lines("./inputs/day02/test.txt") {
        assert_eq!(part_1(input), 1227775554);
    }
}

#[test]
fn part1_test2() {
    if let Ok(input) = read_lines("./inputs/day02/input.txt") {
        assert_eq!(part_1(input), 16793817782);
    }
}
