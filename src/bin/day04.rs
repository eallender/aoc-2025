use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

fn create_grid(input: Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    let row_len = input[0].len();
    let padding = vec!['.'; row_len + 2];
    grid.push(padding.clone());

    for line in input {
        let mut row: Vec<char> = vec![];
        for (i, char) in line.chars().enumerate() {
            if i == 0 {
                row.push('.');
            }
            row.push(char);
            if i == row_len - 1 {
                row.push('.');
            }
        }
        grid.push(row);
    }

    grid.push(padding);
    grid
}

fn is_accessible(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut num_empty = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_y = (y as i32 + i) as usize;
            let new_x = (x as i32 + j) as usize;
            if grid[new_y][new_x] == '.' {
                num_empty += 1;
            }
        }
    }

    num_empty > 4
}

fn part_1(input: Vec<String>) -> i32 {
    let grid = create_grid(input);
    let mut num_rolls = 0;

    for (y, line) in grid.iter().enumerate() {
        debug!("{:?}", line);
        for (x, index) in line.iter().enumerate() {
            if *index == '@' && is_accessible(&grid, x, y) {
                num_rolls += 1;
            }
        }
    }

    info!("Found {num_rolls} accessible rolls");
    num_rolls
}

fn part_2(input: Vec<String>) -> i32 {
    let mut grid = create_grid(input);
    let mut num_rolls = 0;

    loop {
        let mut to_remove = vec![];

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '@' && is_accessible(&grid, x, y) {
                    to_remove.push((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (x, y) in to_remove {
            grid[y][x] = '.';
            num_rolls += 1;
        }
    }

    info!("Found {num_rolls} accessible rolls");
    num_rolls
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
    if let Ok(input) = read_lines("./inputs/day04/test.txt") {
        assert_eq!(part_1(input), 13);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day04/input.txt") {
        assert_eq!(part_1(input), 1474);
    }
}

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day04/test.txt") {
        assert_eq!(part_2(input), 43);
    }
}
#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day04/input.txt") {
        assert_eq!(part_2(input), 8910);
    }
}
