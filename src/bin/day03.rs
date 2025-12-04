use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u32 {
    let mut total_joltage = 0;

    for line in input {
        let mut battery1: u32 = 0;
        let mut battery2: u32 = 0;
        let last = line.chars().last();

        for char in line.chars() {
            info!("battery1: {battery1}");
            info!("battery2: {battery2}");
            info!("current battery: {char}");
            if let Some(curr) = char.to_digit(10) {
                if curr > battery1 && Some(char) != last {
                    battery1 = curr;
                    battery2 = 0;
                } else if curr > battery2 {
                    battery2 = curr;
                }
            }
        }

        let joltage = battery1 * 10 + battery2;
        info!("Current battery bank:\n{}", line);
        info!("Highest joltage: {}", joltage);
        total_joltage += joltage;
    }
    info!("Total Joltage: {}", total_joltage);
    total_joltage
}

fn part_2() {}

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
                // part_2(input);
            }
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}

#[test]
fn part1_test1() {
    if let Ok(input) = read_lines("./inputs/day03/test.txt") {
        assert_eq!(part_1(input), 357);
    }
}
