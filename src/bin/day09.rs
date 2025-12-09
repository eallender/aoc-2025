use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn rectangle_with(&self, other: &Point) -> u64 {
        let dx = (self.x - other.x + 1).abs() as u64;
        let dy = (self.y - other.y + 1).abs() as u64;
        dx * dy
    }
}

struct Rectangle {
    point1: usize,
    point2: usize,
    area: u64,
}

fn part_1(input: Vec<String>) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for line in input {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

        points.push(Point {
            x: parts[0],
            y: parts[1],
        })
    }

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            rectangles.push(Rectangle {
                point1: i,
                point2: j,
                area: points[i].rectangle_with(&points[j]),
            });
        }
    }

    rectangles.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap());

    info!("Largest Area: {}", rectangles[0].area);
    info!("Point1: {:?}", points[rectangles[0].point1]);
    info!("Point2: {:?}", points[rectangles[0].point2]);
    rectangles[0].area
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
    if let Ok(input) = read_lines("./inputs/day09/test.txt") {
        assert_eq!(part_1(input), 50);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day09/input.txt") {
        assert_eq!(part_1(input), 4763509452);
    }
}
