use std::vec;

use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u64 {
    let mut totals: Vec<u64> = Vec::new();
    let signs: Vec<String> = input[input.len() - 1].split_whitespace().map(|s: &str| s.to_string()).collect();
    info!("Input: {:?}", input);
    info!("Signs: {:?}", signs);

    for index in 0..input.len() {
        if index == input.len() - 1 {
            continue;
        }
        let line: Vec<u64> = input[index].split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
        info!("Line: {:?}", line);

        if index == 0 {
            for problem in line {
                totals.push(problem);
            }            
        } else {
            for (i, problem) in line.iter().enumerate() {
                if signs[i] == "*" {
                    totals[i] *= problem;
                } else if signs[i] == "+" {
                    totals[i] += problem;
                }
            }
        }
    }

    let mut total: u64 = 0;
    for t in totals {
        total += t;
    }

    info!("Total: {}", total);
    total
}

fn part_2(input: Vec<String>) {}

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
    if let Ok(input) = read_lines("./inputs/day06/test.txt") {
        assert_eq!(part_1(input), 4277556);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day06/input.txt") {
        assert_eq!(part_1(input), 6209956042374);
    }
}