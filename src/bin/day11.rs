use std::collections::HashMap;

use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn visit_neighbors(
    start: &str,
    end: &str,
    map: &HashMap<String, Vec<String>>,


    memo: &mut HashMap<(String, String), i64>,
) -> i64 {
    debug!("{start}");

    if start == end {
        return 1;
    }

    let key = (start.to_string(), end.to_string());
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = map.get(start) {
        for neighbor in neighbors {
            total += visit_neighbors(neighbor, end, map, memo);
        }
    }

    memo.insert(key, total);
    total
}

fn part_1(input: Vec<String>) -> i64 {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let mut key = parts[0].clone();
        key.truncate(key.len() - 1);

        let mut devices: Vec<String> = Vec::new();
        for index in 1..parts.len() {
            devices.push(parts[index].clone());
        }
        map.insert(key, devices);
    }

    let mut memo: HashMap<(String, String), i64> = HashMap::new();
    let total_paths: i64 = visit_neighbors("you", "out", &map, &mut memo);

    info!("Total Paths: {total_paths}");
    total_paths

}

fn part_2(input: Vec<String>) -> i64 {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let mut key = parts[0].clone();
        key.truncate(key.len() - 1);

        let mut devices: Vec<String> = Vec::new();
        for index in 1..parts.len() {
            devices.push(parts[index].clone());
        }
        map.insert(key, devices);
    }

    let mut memo: HashMap<(String, String), i64> = HashMap::new();
    let svr_to_dac = visit_neighbors("svr", "dac", &map, &mut memo);
    info!("SVR to DAC: {}", svr_to_dac);
    let dac_to_fft = visit_neighbors("dac", "fft", &map, &mut memo);
    info!("DAC to FFT: {}", dac_to_fft);
    let fft_to_out = visit_neighbors("fft", "out", &map, &mut memo);
    info!("FFT to OUT: {}", fft_to_out);

    let svr_to_fft = visit_neighbors("svr", "fft", &map, &mut memo);
    info!("SVR to FFT: {}", svr_to_fft);
    let fft_to_dac = visit_neighbors("fft", "dac", &map, &mut memo);
    info!("FFT to DAC: {}", fft_to_dac);
    let dac_to_out = visit_neighbors("dac", "out", &map, &mut memo);
    info!("DAC to OUT: {}", dac_to_out);

    let total = svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out;
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
    if let Ok(input) = read_lines("./inputs/day11/part1/test.txt") {
        assert_eq!(part_1(input), 5);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day11/input.txt") {
        assert_eq!(part_1(input), 670);
    }
}

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day11/part2/test.txt") {
        assert_eq!(part_2(input), 2);
    }
}
#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day11/input.txt") {
        assert_eq!(part_2(input), 332052564714990);
    }
}
