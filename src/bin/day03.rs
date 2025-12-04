use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u32 {
    let mut total_joltage = 0;

    for line in input {
        let mut battery1: u32 = 0;
        let mut battery2: u32 = 0;
        let last = line.chars().count() - 1;
        debug!("Current battery bank:\n{}", line);
        for (i, char) in line.chars().enumerate() {
            if let Some(curr) = char.to_digit(10) {
                debug!("battery1: {}", battery1);
                debug!("battery2: {}", battery2);
                debug!("Current: {}", curr);
                if curr > battery1 && i != last {
                    battery1 = curr;
                    battery2 = 0;
                } else if curr > battery2 {
                    battery2 = curr;
                }
            }
        }

        let joltage = battery1 * 10 + battery2;
        debug!("Highest joltage: {}", joltage);
        total_joltage += joltage;
    }
    info!("Total Joltage: {}", total_joltage);
    total_joltage
}

fn part_2(input: Vec<String>) -> u64 {
    let mut total_joltage: u64 = 0;

    for line in input {
        let mut batteries: Vec<u32> = vec![0; 12];
        let last_index = line.chars().count() - 1;
        debug!("Current battery bank:\n{}", line);
        for (bank_i, char) in line.chars().enumerate() {
            debug!("Batteries:\n{:?}", batteries);
            for bat_i in 0..batteries.len() {
                if let Some(curr) = char.to_digit(10) {
                    debug!("Current: {}", curr);
                    if curr > batteries[bat_i]
                        && (last_index - bank_i >= batteries.len() - 1 - bat_i)
                    {
                        batteries[bat_i] = curr;
                        for i in bat_i + 1..batteries.len() {
                            batteries[i] = 0;
                        }
                        break;
                    }
                }
            }
        }

        let joltage: u64 = batteries
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        debug!("Highest joltage: {}", joltage);
        total_joltage += joltage;
    }
    info!("Total Joltage: {}", total_joltage);
    total_joltage
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
    if let Ok(input) = read_lines("./inputs/day03/test.txt") {
        assert_eq!(part_1(input), 357);
    }
}
#[test]
fn part1_inc() {
    assert_eq!(part_1(vec!["1234567".to_string()]), 67);
}

#[test]
fn part1_dec() {
    assert_eq!(part_1(vec!["7654321".to_string()]), 76);
}

#[test]
fn part1_lhl() {
    assert_eq!(part_1(vec!["1234321".to_string()]), 43);
}

#[test]
fn part1_hlh() {
    assert_eq!(part_1(vec!["4321234".to_string()]), 44);
}

#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day03/input.txt") {
        assert_eq!(part_1(input), 17766);
    }
}

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day03/test.txt") {
        assert_eq!(part_2(input), 3121910778619);
    }
}

#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day03/input.txt") {
        assert_eq!(part_2(input), 176582889354075);
    }
}
