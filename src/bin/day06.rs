use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u64 {
    let mut totals: Vec<u64> = Vec::new();
    let signs: Vec<String> = input[input.len() - 1].split_whitespace().map(|s: &str| s.to_string()).collect();
    debug!("Input: {:?}", input);
    debug!("Signs: {:?}", signs);

    for index in 0..input.len() - 1 {
        let line: Vec<u64> = input[index].split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
        debug!("Line: {:?}", line);

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

fn part_2(input: Vec<String>) -> u64 {
    let mut totals: Vec<u64> = Vec::new();
    let mut problems: Vec<Vec<String>> = Vec::new();
    let signs: Vec<String> = input[input.len() - 1].split_whitespace().map(|s: &str| s.to_string()).collect();
    debug!("Input: {:?}", input);
    debug!("Signs: {:?}", signs);

    for index in 0..input.len()-1 {
        let mut line: Vec<String> = vec!();
        let mut curr_string = "".to_string();
        let mut is_first = true;
        for (i,char) in input[input.len()-1].chars().enumerate() {
            if char == '+' || char == '*' {
                if !is_first {
                    curr_string.pop();
                    line.push(curr_string.clone());
                } else {
                    is_first = false;
                }
                curr_string = "".to_string();
            }
            debug!("{}", input[index].chars().nth(i).unwrap());
            curr_string.push(input[index].chars().nth(i).unwrap());
        } 
        line.push(curr_string.clone());

        debug!("Line: {:?}", line);

        if index == 0 {
            for problem in &line {
                let problem_init = vec!["".to_string(); problem.len()];
                problems.push(problem_init);
            }
        }

        for iter in 0..line.len() {
            for (i, char) in line[iter].chars().enumerate() {
                if char != ' ' {
                    problems[iter][i].push(char);
                }
                debug!("Char: {:?}", char);
                debug!("Problems: {:?}", problems);
            }
        }
    }

    for (i, problem) in problems.iter().enumerate() {
        debug!("{:?}", problem);
        if signs[i] == "+" {
            totals.push(problem.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .sum());
        } else if signs[i] == "*" {
            totals.push(problem.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .product());
        }
    }

    let mut total: u64 = 0;
    for t in totals {
        total += t;
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
#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day06/test.txt") {
        assert_eq!(part_2(input), 3263827);
    }
}
#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day06/input.txt") {
        assert_eq!(part_2(input), 12608160008022);
    }
}