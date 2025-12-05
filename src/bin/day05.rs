use std::collections::HashMap;

use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> i32 {
    let mut read_ranges = false;
    let mut ranges = HashMap::new();
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
                    info!("Valid ID: {}, Range: {}, {}", id, *key, *value);
                    total_valid += 1;
                    break;
                }
            }
        } else {
            let range: Vec<u64> = line.split("-").map(|s: &str| s.parse().unwrap()).collect();
            ranges.insert(range[0], range[1]);
        }
    }

    info!("Total Valid IDs: {total_valid}");
    total_valid
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
            // if let Ok(input) = read_lines(&args.input) {
            //     part_2(input);
            // }
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}
