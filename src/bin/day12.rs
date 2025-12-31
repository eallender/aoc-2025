use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
struct Tree {
    length: usize,
    width: usize,
    capacity: Vec<usize>,
}

#[derive(Debug)]
struct Present {
    points: Vec<(usize, usize)>,
    variations: Vec<Vec<Vec<bool>>>,
    num_spaces: usize,
}

fn read_input(input: Vec<String>) -> (Vec<Present>, Vec<Tree>) {
    let mut presents = Vec::new();
    let mut trees = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let re_present = Regex::new(r"^(\d+):").unwrap();
    let re_points = Regex::new(r"^[#.]+$").unwrap();
    let re_tree = Regex::new(r"(\d+)x(\d+):").unwrap();
    for line in input {
        debug!("{:?}", line);
        if re_present.is_match(&line) {
            debug!("Creating new present");
            presents.push(Present {
                points: Vec::new(),
                variations: Vec::new(),
                num_spaces: 0,
            });
            x = 0;
            y = 0;
        } else if re_points.is_match(&line) {
            debug!("Adding points to last present");
            for char in line.chars() {
                if char == '#' {
                    presents.last_mut().unwrap().points.push((x, y));
                    presents.last_mut().unwrap().num_spaces += 1;
                }
                x += 1;
            }
            x = 0;
            y += 1;
        } else if let Some(caps) = re_tree.captures(&line) {
            debug!("Adding tree");
            let length = caps[1].parse().unwrap();
            let width = caps[2].parse().unwrap();

            let parts: Vec<&str> = line.split_whitespace().collect();
            let capacity: Vec<usize> = parts.iter().skip(1).map(|s| s.parse().unwrap()).collect();

            trees.push(Tree {
                length,
                width,
                capacity,
            });
        } else {
            debug!("No match");
            continue;
        }
    }
    (presents, trees)
}

fn rotate_90_cw(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_shape = vec![vec![false; 3]; 3];

    for row in 0..3 {
        for col in 0..3 {
            new_shape[col][2 - row] = shape[row][col];
        }
    }

    new_shape
}

fn flip_horizontal(shape: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_shape = vec![vec![false; 3]; 3];

    for row in 0..3 {
        new_shape[row][0] = shape[row][2];
        new_shape[row][1] = shape[row][1];
        new_shape[row][2] = shape[row][0];
    }

    new_shape
}

fn debug_shape(shape: &Vec<Vec<bool>>) {
    debug!("---");
    for row in shape {
        let line: String = row.iter().map(|&b| if b { '#' } else { '.' }).collect();
        debug!("{:?}", line);
    }
}

fn debug_grid(grid: &Vec<Vec<Option<usize>>>) {
    debug!("=== Grid State ===");
    for row in grid {
        let line: String = row
            .iter()
            .map(|cell| match cell {
                None => '.'.to_string(),
                Some(id) => id.to_string(),
            })
            .collect();
        debug!("{}", line);
    }
    debug!("=================");
}

fn get_variations(presents: &mut Vec<Present>) {
    for idx in 0..presents.len() {
        debug!("{:?}", presents[idx].points);
        let mut seen: HashSet<Vec<Vec<bool>>> = HashSet::new();
        let mut current = vec![vec![false; 3]; 3];

        for (x, y) in &presents[idx].points {
            current[*y][*x] = true;
        }

        for _ in 0..4 {
            if seen.insert(current.clone()) {
                presents[idx].variations.push(current.clone());
            }

            let flipped = flip_horizontal(current.clone());
            if seen.insert(flipped.clone()) {
                presents[idx].variations.push(flipped.clone());
            }

            current = rotate_90_cw(current);
        }
    }
}

fn can_place(
    grid: &Vec<Vec<Option<usize>>>,
    shape: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
) -> bool {
    for (row_offset, row) in shape.iter().enumerate() {
        for (col_offset, &cell) in row.iter().enumerate() {
            if cell {
                let grid_row = start_row + row_offset;
                let grid_col = start_col + col_offset;

                if grid_row >= grid.len() || grid_col >= grid[0].len() {
                    return false;
                }

                if grid[grid_row][grid_col].is_some() {
                    return false;
                }
            }
        }
    }
    true
}

fn place_shape(
    grid: &mut Vec<Vec<Option<usize>>>,
    shape: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    shape_id: usize,
) {
    for (row_offset, row) in shape.iter().enumerate() {
        for (col_offset, &cell) in row.iter().enumerate() {
            if cell {
                let grid_row = start_row + row_offset;
                let grid_col = start_col + col_offset;
                grid[grid_row][grid_col] = Some(shape_id);
            }
        }
    }
}

fn remove_shape(
    grid: &mut Vec<Vec<Option<usize>>>,
    shape: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
) {
    for (row_offset, row) in shape.iter().enumerate() {
        for (col_offset, &cell) in row.iter().enumerate() {
            if cell {
                let grid_row = start_row + row_offset;
                let grid_col = start_col + col_offset;
                grid[grid_row][grid_col] = None;
            }
        }
    }
}

fn try_place_all(
    grid: &mut Vec<Vec<Option<usize>>>,
    presents: &Vec<Present>,
    counts: &mut Vec<usize>,
    present_order: &[(usize, usize)],
    current_idx: usize,
) -> bool {
    if current_idx >= present_order.len() {
        return true;
    }

    let (present_idx, _) = present_order[current_idx];

    if counts[current_idx] == 0 {
        return try_place_all(grid, presents, counts, present_order, current_idx + 1);
    }

    let total_cells_needed: usize = present_order
        .iter()
        .enumerate()
        .skip(current_idx)
        .map(|(idx, (pidx, _))| presents[*pidx].num_spaces * counts[idx])
        .sum();

    let remaining_cells = grid.iter().flatten().filter(|cell| cell.is_none()).count();
    if remaining_cells < total_cells_needed {
        return false;
    }

    let present = &presents[present_idx];

    for variation in &present.variations {
        // Only check placements where a 3x3 present could fit
        let max_row = grid.len().saturating_sub(3);
        let max_col = grid[0].len().saturating_sub(3);

        for row in 0..=max_row {
            for col in 0..=max_col {
                if can_place(grid, variation, row, col) {
                    place_shape(grid, variation, row, col, present_idx);
                    counts[current_idx] -= 1;

                    if try_place_all(grid, presents, counts, present_order, current_idx) {
                        return true;
                    }

                    counts[current_idx] += 1;
                    remove_shape(grid, variation, row, col);
                }
            }
        }
    }

    false
}

fn can_fit(tree: &Tree, presents: &Vec<Present>) -> bool {
    let mut grid = vec![vec![None; tree.length]; tree.width];

    let required: Vec<(usize, usize)> = tree
        .capacity
        .iter()
        .enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(idx, count)| (idx, *count))
        .collect();

    let total_cells_needed: usize = required
        .iter()
        .map(|(present_idx, count)| {
            let cells_per_shape = presents[*present_idx].num_spaces;
            cells_per_shape * count
        })
        .sum();

    let grid_size = tree.width * tree.length;
    if total_cells_needed > grid_size {
        return false;
    }

    let mut counts: Vec<usize> = required.iter().map(|(_, count)| *count).collect();
    let result = try_place_all(&mut grid, presents, &mut counts, &required, 0);
    if result {
        debug_grid(&grid);
    }
    result
}

fn part_1(input: Vec<String>) -> i32 {
    let start = Instant::now();

    let (mut presents, trees) = read_input(input);
    get_variations(&mut presents);

    let mut valid_count = 0;

    for (idx, tree) in trees.iter().enumerate() {
        info!("Checking tree {}: {}x{}", idx, tree.width, tree.length);
        if can_fit(tree, &presents) {
            info!("Tree {} CAN fit all presents", idx);
            valid_count += 1;
        } else {
            info!("Tree {} CANNOT fit all presents", idx);
        }
    }

    let duration = start.elapsed();
    info!("Total regions that can fit all presents: {}", valid_count);
    info!("Execution time: {:?}", duration);
    valid_count
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
            info!("There is no part 2 for this problem.")
        }
        _ => error!("Insufficient part provided: 1 or 2 required"),
    }
}

#[test]
fn part1_test() {
    if let Ok(input) = read_lines("./inputs/day12/test.txt") {
        assert_eq!(part_1(input), 2);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day12/input.txt") {
        assert_eq!(part_1(input), 555);
    }
}
