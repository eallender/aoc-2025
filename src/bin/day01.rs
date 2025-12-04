use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u32 {
    let mut pos: i32 = 50;
    let mut num_zero: u32 = 0;

    for line in input {
        let dir = line.chars().nth(0).unwrap();
        let num: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        debug!("Direction: {:?}, Number: {:?}", dir, num);

        match dir {
            'L' => pos -= num,
            'R' => pos += num,
            _ => error!("Invalid direction received in input"),
        }

        pos %= 100;

        if pos == 0 {
            num_zero += 1;
        }
    }

    info!("Password: {}", num_zero);
    num_zero
}

fn part_2(input: Vec<String>) -> u32 {
    let mut pos: i32 = 50;
    let mut num_zero: u32 = 0;

    for line in input {
        let dir = line.chars().nth(0).unwrap();
        let num: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        debug!("Direction: {:?}, Number: {:?}", dir, num);

        num_zero += (num / 100) as u32;
        let remaining = num % 100;

        match dir {
            'L' => {
                if pos > 0 && pos <= remaining {
                    num_zero += 1;
                }
                pos = (pos - num).rem_euclid(100);
            }
            'R' => {
                if pos + remaining >= 100 {
                    num_zero += 1;
                }
                pos = (pos + num).rem_euclid(100);
            }
            _ => error!("Invalid direction received in input"),
        }
    }

    info!("Password: {}", num_zero);
    num_zero
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
    if let Ok(input) = read_lines("./inputs/day01/test.txt") {
        assert_eq!(part_1(input), 3);
    }
}

#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day01/input.txt") {
        assert_eq!(part_1(input), 982);
    }
}

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day01/test.txt") {
        assert_eq!(part_2(input), 6);
    }
}

#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day01/input.txt") {
        assert_eq!(part_2(input), 6106);
    }
}
