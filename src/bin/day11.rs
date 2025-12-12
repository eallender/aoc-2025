use std::collections::{HashMap, HashSet};

use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn visit_neighbors(
    curr: String,
    map: &HashMap<String, Vec<String>>,
    visited: HashSet<String>,
) -> i64 {
    debug!("{curr}");
    let neighbors = map.get(&curr).unwrap();
    let mut total_paths = 0;
    for neighbor in neighbors {
        if neighbor.clone() == "out".to_string() {
            total_paths += 1;
        } else if !visited.contains(neighbor) {
            let mut new_visited = visited.clone();
            new_visited.insert(curr.clone());
            total_paths += visit_neighbors(neighbor.clone(), map, new_visited);
        }
    }
    total_paths
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

    let visited: HashSet<String> = HashSet::new();
    let total_paths: i64 = visit_neighbors("you".to_string(), &map, visited);

    info!("Total Paths: {total_paths}");
    total_paths
}

fn visit_dac_fft(
    curr: String,
    map: &HashMap<String, Vec<String>>,
    visited: HashSet<String>,
) -> i64 {
    debug!("{curr}");
    let neighbors = map.get(&curr).unwrap();
    let mut total_paths = 0;
    for neighbor in neighbors {
        if neighbor.clone() == "out".to_string() {
            if visited.contains("dac") && visited.contains("fft") {
                total_paths += 1;
            }
        } else if !visited.contains(neighbor) {
            let mut new_visited = visited.clone();
            new_visited.insert(curr.clone());
            total_paths += visit_dac_fft(neighbor.clone(), map, new_visited);
        }
    }
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

    let visited: HashSet<String> = HashSet::new();
    let total_paths: i64 = visit_dac_fft("svr".to_string(), &map, visited);

    info!("Total Paths: {total_paths}");
    total_paths
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
        assert_eq!(part_2(input),);
    }
}
