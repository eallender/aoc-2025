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
        let dx = (self.x - other.x).abs() as u64 + 1;
        let dy = (self.y - other.y).abs() as u64 + 1;
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

fn part_2(input: Vec<String>) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    let mut rg: Vec<(i64, i64)> = Vec::new();
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for line in input {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

        points.push(Point {
            x: parts[0],
            y: parts[1],
        });

        rg.push((parts[0], parts[1]));
    }

    // get all points in the outline
    info!("Getting points in outline...");
    for i in 0..points.len() {
        let mut prev = 0;
        let mut next = 0;
        if i == 0 {
            prev = points.len() - 1;
            next = i + 1;
        } else if i == points.len() - 1 {
            prev = i - 1;
            next = 0;
        } else {
            prev = i - 1;
            next = i + 1;
        }

        let to_add = vec![prev, next];
        debug!("Prev: {}, {}", points[prev].x, points[prev].y);
        debug!("Curr: {}, {}", points[i].x, points[i].y);
        debug!("Next: {}, {}", points[next].x, points[next].y);
        for point in to_add {
            if points[i].x == points[point].x {
                for j in
                    (points[i].y + 1).min(points[point].y + 1)..points[i].y.max(points[point].y)
                {
                    rg.push((points[i].x, j));
                    debug!("Inserting: {}, {}", points[i].x, j);
                }
            } else if points[i].y == points[point].y {
                for j in
                    (points[i].x + 1).min(points[point].x + 1)..points[i].x.max(points[point].x)
                {
                    rg.push((j, points[i].y));
                    debug!("Inserting: {}, {}", j, points[i].y);
                }
            }
        }
    }
    debug!("RG:\n{:?}", rg);

    info!("Getting rectangles...");
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            rectangles.push(Rectangle {
                point1: i,
                point2: j,
                area: points[i].rectangle_with(&points[j]),
            });
        }
    }

    info!("Checking rectangles...");
    let mut best_area = 0;

    for rectangle in rectangles {
        if rectangle.area < best_area {
            continue;
        }
        let p1 = &points[rectangle.point1];
        let p2 = &points[rectangle.point2];

        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        let mut has_inside_point = false;

        for &(x, y) in &rg {
            if x > min_x && x < max_x && y > min_y && y < max_y {
                has_inside_point = true;
                break;
            }
        }

        if !has_inside_point {
            if rectangle.area > best_area {
                best_area = rectangle.area;
            }
        }
    }

    info!("Largest Rectangle: {}", best_area);
    return best_area;
    0
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

#[test]
fn part2_test() {
    if let Ok(input) = read_lines("./inputs/day09/test.txt") {
        assert_eq!(part_2(input), 24);
    }
}
#[test]
fn part2_input() {
    if let Ok(input) = read_lines("./inputs/day09/input.txt") {
        assert_eq!(part_2(input), 1516897893);
    }
}
