use core::num;
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

fn part_1(input: Vec<String>) -> usize {
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
        if !con_points.contains_key(&edge.box1) && !con_points.contains_key(&edge.box2) {
            let mut circuit = Circuit { boxes: Vec::new() };
            circuit.join(edge.box1);
            con_points.insert(edge.box1, circuits.len());
            circuit.join(edge.box2);
            con_points.insert(edge.box2, circuits.len());
            circuits.push(circuit);
        } else if con_points.contains_key(&edge.box1) {
            let circuit = con_points[&edge.box1];
            circuits[circuit].join(edge.box2);
            con_points.insert(edge.box2, circuit);
        } else if con_points.contains_key(&edge.box2) {
            let circuit = con_points[&edge.box2];
            circuits[circuit].join(edge.box1);
            con_points.insert(edge.box1, circuit);
        }
        num_edges += 1;
        if num_edges == 1000 {
            break;
        }
        info!("Circuits: \n{:?}", circuits)
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
