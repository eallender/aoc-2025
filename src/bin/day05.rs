use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> i32 {
    let mut read_ranges = false;
    let mut ranges = Vec::new();
    let mut total_valid = 0;

    for line in input {
        if line == "" {
            read_ranges = true;
            continue;
        }
        if read_ranges {
            for (key, value) in &ranges {
                let id: u64 = line.parse().unwrap();
                if id <= *value && id >= *key {
                    debug!("Valid ID: {}, Range: {}, {}", id, *key, *value);
                    total_valid += 1;
                    break;
                }
            }
        } else {
            let range: Vec<u64> = line.split("-").map(|s: &str| s.parse().unwrap()).collect();
            ranges.push((range[0], range[1]));
        }
    }

    info!("Total Valid IDs: {total_valid}");
    total_valid
}

fn part_2(input: Vec<String>) -> u64 {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input {
        if line == "" {
            break;
        }
        info!("{:?}", ranges);
        let range: Vec<u64> = line.split("-").map(|s: &str| s.parse().unwrap()).collect();
        let mut ranges_altered = false;
        let mut to_fix = vec![];
        info!("Lower: {}, Upper: {}", range[0], range[1]);
        for i in 0..ranges.len() {
            let (lower, upper) = ranges[i];
            if range[0] >= lower && range[0] <= upper && range[1] > upper
                || range[1] >= lower && range[1] <= upper && range[0] < lower
            {
                to_fix.push(i);
                ranges_altered = true;
            } 
        }
        if !ranges_altered {
            ranges.push((range[0], range[1]));
        } else {
            let mut min = u64::MAX;
            let mut max = u64::MIN;
            for i in to_fix.iter().rev() {
                if ranges[*i].0 < min {
                    min = ranges[*i].0;
                }
                if ranges[*i].1 > max {
                    max = ranges[*i].1;
                }
                ranges.remove(*i);
            }
            ranges.push((min, max));
        }
    }

    info!("All Ranges:\n{:?}", ranges);
    let mut total_fresh = 0;
    for (lower, upper) in &ranges {
        total_fresh += upper - lower + 1;
    }

    info!("Total Valid IDs: {total_fresh}");
    total_fresh
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
    if let Ok(input) = read_lines("./inputs/day05/test.txt") {
        assert_eq!(part_1(input), 3);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day05/input.txt") {
        assert_eq!(part_1(input), 509);
    }
}