use aoc_2025::{Args, create_grid, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};
use std::collections::HashMap;

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
        if y == grid.len() - 1 {
            continue;
        }
        match grid[y][x] {
            '.' => {
                grid[y][x] = '|';
                queue.push((y + 1, x));
            }
            '^' => {
                if x > 0 {
                    queue.push((y, x - 1));
                }
                if x + 1 <= grid[0].len() - 1 {
                    queue.push((y, x + 1));
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

fn count_timelines(
    y: usize,
    x: usize,
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if y == grid.len() - 1 {
        return 1;
    }

    if let Some(&result) = memo.get(&(y, x)) {
        return result;
    }

    let result = match grid[y][x] {
        '.' | '|' => count_timelines(y + 1, x, grid, memo),
        '^' => {
            let mut total = 0;

            if x > 0 {
                total += count_timelines(y, x - 1, grid, memo);
            }

            if x + 1 < grid[0].len() {
                total += count_timelines(y, x + 1, grid, memo);
            }

            total
        }
        _ => {
            panic!("Unexpected char in grid: {}", grid[y][x]);
        }
    };

    memo.insert((y, x), result);
    result
}

fn part_2(input: Vec<String>) -> u64 {
    let grid = create_grid(input);
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    let mut total_paths = 0;
    for x in 0..grid[0].len() {
        if grid[1][x] == 'S' {
            total_paths += count_timelines(2, x, &grid, &mut memo);
        }
    }

    info!("Total Paths: {}", total_paths);
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
#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day07/test.txt") {
        assert_eq!(part_2(input), 40);
    }
}
#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day07/input.txt") {
        assert_eq!(part_2(input), 108924003331749);
    }
}
