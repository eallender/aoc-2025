use aoc_2025::{Args, read_lines, create_grid};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn part_1(input: Vec<String>) -> u32 {
    let mut grid = create_grid(input);
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut total_splits: u32 = 0;
    
    for x in 0..grid[0].len() {
        if grid[1][x] == 'S' {
            queue.push((2, x));
        }
    }
    
    while !queue.is_empty() {
        let (y, x) = queue.remove(0);
        if y == grid.len()-1 {
            continue;
        }
        match grid[y][x] {
            '.' => {
                grid[y][x] = '|';
                queue.push((y+1, x));
            }
            '^' => {
                if x > 0 {
                    queue.push((y, x-1));
                }
                if x + 1 <= grid[0].len() - 1 {
                    queue.push((y, x+1));
                } 
                total_splits += 1;
            }
            '|' => {
                continue;
            }
            _ => {
                panic!("Unexpected char in grid");
            }
        }
    }
    debug!("Grid:");
    for row in &grid {
        debug!("{:?}", row);
    }
    info!("Total Splits: {}", total_splits);
    total_splits
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
    if let Ok(input) = read_lines("./inputs/day07/test.txt") {
        assert_eq!(part_1(input), 21);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day07/input.txt") {
        assert_eq!(part_1(input), 1651);
    }
}