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
            part_2();
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}

#[test]
fn part1_test1() {
    if let Ok(input) = read_lines("./inputs/day01/test.txt") {
        assert_eq!(part_1(input), 3);
    }
}

#[test]
fn part1_test2() {
    if let Ok(input) = read_lines("./inputs/day01/input.txt") {
        assert_eq!(part_1(input), 982);
    }
}
