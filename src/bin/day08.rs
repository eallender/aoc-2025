use std::collections::HashMap;

use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x).abs() as f64;
        let dy = (self.y - other.y).abs() as f64;
        let dz = (self.z - other.z).abs() as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct Edge {
    box1: usize,
    box2: usize,
    distance: f64,
}

#[derive(Debug)]
struct Circuit {
    boxes: Vec<usize>,
}

impl Circuit {
    fn size(&self) -> usize {
        self.boxes.len()
    }

    fn join(&mut self, other: usize) {
        self.boxes.push(other);
    }
}

fn part_1(input: Vec<String>, num_conn: usize) -> usize {
    let mut points: Vec<Point> = Vec::new();
    for point in input {
        let parts: Vec<i64> = point
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        points.push(Point {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        });
    }

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_to(&points[j]);
            edges.push(Edge {
                box1: i,
                box2: j,
                distance,
            });
        }
    }

    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    let mut con_points: HashMap<usize, usize> = HashMap::new();
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut num_edges = 0;
    for edge in edges {
        let box1_circuit = con_points.get(&edge.box1).copied();
        let box2_circuit = con_points.get(&edge.box2).copied();
        debug!("Circuits: \n{:?}", circuits);

        num_edges += 1;
        if num_edges > num_conn {
            break;
        }

        match (box1_circuit, box2_circuit) {
            (None, None) => {
                let circuit_id = circuits.len();
                let mut circuit = Circuit { boxes: Vec::new() };
                circuit.join(edge.box1);
                circuit.join(edge.box2);
                con_points.insert(edge.box1, circuit_id);
                con_points.insert(edge.box2, circuit_id);
                circuits.push(circuit);
            }
            (Some(c1), None) => {
                circuits[c1].join(edge.box2);
                con_points.insert(edge.box2, c1);
            }
            (None, Some(c2)) => {
                circuits[c2].join(edge.box1);
                con_points.insert(edge.box1, c2);
            }
            (Some(c1), Some(c2)) => {
                if c1 == c2 {
                    continue;
                } else {
                    let boxes_to_move = circuits[c2].boxes.clone();
                    for box_id in boxes_to_move {
                        circuits[c1].join(box_id);
                        con_points.insert(box_id, c1);
                    }
                    circuits[c2].boxes.clear();
                }
            }
        }
    }

    let mut total: usize = 1;
    circuits.sort_by(|a, b| b.size().partial_cmp(&a.size()).unwrap());
    for i in 0..3 {
        debug!("{}", circuits[i].boxes.len());
        total *= circuits[i].boxes.len();
    }

    info!("Total: {total}");
    total
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
            let num_conn: usize;
            if &args.input == "./inputs/day08/test.txt" {
                num_conn = 10
            } else {
                num_conn = 1000
            }
            if let Ok(input) = read_lines(&args.input) {
                part_1(input, num_conn);
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
    if let Ok(input) = read_lines("./inputs/day08/test.txt") {
        assert_eq!(part_1(input, 10), 40);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day08/input.txt") {
        assert_eq!(part_1(input, 1000), 50568);
    }
}
