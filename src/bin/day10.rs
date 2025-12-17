use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn read_input(input: Vec<String>) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();

    for line in input {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let mut target: Vec<bool> = Vec::new();
        let mut joltage: Vec<usize> = Vec::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for i in 0..parts.len() {
            let mut part = parts[i].clone();
            debug!("{:?}", part);
            if i == 0 {
                part = part.strip_prefix('[')
                    .and_then(|s| s.strip_suffix(']'))
                    .unwrap().to_string();
                for char in part.chars() {
                    if char == '#' {
                        target.push(true);
                    } else {
                        target.push(false);
                    }
                }
                continue
            }
            if i == parts.len() - 1 {
                part = part.strip_prefix('{')
                    .and_then(|s| s.strip_suffix('}'))
                    .unwrap().to_string();
                joltage = part.split(',').map(|s: &str| s.parse().unwrap()).collect();
                continue;
            }
            part = part.strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap().to_string();
            let button: Vec<usize> = part.split(',').map(|s: &str| s.parse().unwrap()).collect();
            buttons.push(button);
        }
        machines.push(Machine { target, buttons, joltage });
    }
    debug!("Machines: \n{:?}", machines);
    machines
}

fn part_1(input: Vec<String>) {
    let machines = read_input(input);
    let mut min_presses: Vec<u64> = Vec::new();

    for machine in machines {
        let mut min_press = usize::MAX;
        let mut states: Vec<(Vec<bool>, usize)> = Vec::new();
        debug!("-=- Next machine -=-");

        let initial_state = vec![false; machine.target.len()];
        states.push((initial_state, 0));

        while let Some((state, depth)) = states.first().cloned() {
            states.remove(0);

            debug!("State: {:?}, Depth: {}", state, depth);

            if depth >= min_press {
                break;
            }

            for button in &machine.buttons {
                let mut new_state = state.clone();
                for i in button {
                    new_state[*i] = !new_state[*i];
                }
                debug!("Button: {:?}", button);
                debug!("New state: {:?}", new_state);

                if new_state == machine.target {
                    min_press = depth + 1;
                    debug!("Target found!");
                    debug!("Num presses: {}", min_press);
                    break;
                } else {
                    states.push((new_state, depth + 1));
                }
            }
        }

        if min_press == usize::MAX {
            panic!("No solution found for machine!");
        }

        min_presses.push(min_press as u64);
    }
    debug!("Result: {:?}", min_presses);
    debug!("Sum: {}", min_presses.iter().sum::<u64>());
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