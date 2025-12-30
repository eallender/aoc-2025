use aoc_2025::{Args, read_lines};
use clap::Parser;
use dotenv::dotenv;
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, default_solver,
};
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
                part = part
                    .strip_prefix('[')
                    .and_then(|s| s.strip_suffix(']'))
                    .unwrap()
                    .to_string();
                for char in part.chars() {
                    if char == '#' {
                        target.push(true);
                    } else {
                        target.push(false);
                    }
                }
                continue;
            }
            if i == parts.len() - 1 {
                part = part
                    .strip_prefix('{')
                    .and_then(|s| s.strip_suffix('}'))
                    .unwrap()
                    .to_string();
                joltage = part.split(',').map(|s: &str| s.parse().unwrap()).collect();
                continue;
            }
            part = part
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap()
                .to_string();
            let button: Vec<usize> = part.split(',').map(|s: &str| s.parse().unwrap()).collect();
            buttons.push(button);
        }
        machines.push(Machine {
            target,
            buttons,
            joltage,
        });
    }
    debug!("Machines: \n{:?}", machines);
    machines
}

fn solve_machine(machine: &Machine) -> usize {
    let n_buttons = machine.buttons.len();
    let mut min_presses = usize::MAX;

    for mask in 0..(1 << n_buttons) {
        let mut state = vec![false; machine.target.len()];
        let mut presses = 0;

        for (i, button) in machine.buttons.iter().enumerate() {
            if mask & (1 << i) != 0 {
                presses += 1;
                for &light_idx in button {
                    state[light_idx] = !state[light_idx];
                }
            }
        }

        if state == machine.target {
            min_presses = min_presses.min(presses);
        }
    }

    min_presses
}

fn part_1(input: Vec<String>) -> usize {
    let machines = read_input(input);
    let mut min_presses: Vec<usize> = Vec::new();

    for machine in machines {
        min_presses.push(solve_machine(&machine));
    }
    debug!("Result: {:?}", min_presses);
    let sum = min_presses.iter().sum::<usize>();
    info!("Sum: {}", sum);
    sum
}

fn solve_joltage(machine: &Machine) -> usize {
    let n_buttons = machine.buttons.len();

    // Create decision variables: x[i] = number of times to press button i
    let mut vars = ProblemVariables::new();
    let x: Vec<Variable> = (0..n_buttons).map(|_| vars.add_variable()).collect();

    // Objective: minimize total button presses
    let objective: Expression = x.iter().map(|&var| var).sum();
    let mut problem = vars.minimise(objective).using(default_solver);

    // Add non-negativity constraints
    for &var in &x {
        problem = problem.with(constraint!(var >= 0));
    }

    // Add constraint for each counter
    for (counter_idx, &target_val) in machine.joltage.iter().enumerate() {
        // Sum of all buttons that affect this counter must equal target
        let constraint_expr: Expression = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.contains(&counter_idx))
            .map(|(button_idx, _)| x[button_idx])
            .sum();

        problem = problem.with(constraint!(constraint_expr == target_val as i32));
    }

    let solution = problem.solve().expect("Failed to solve");
    x.iter()
        .map(|&var| solution.value(var).round() as usize)
        .sum()
}

fn part_2(input: Vec<String>) -> usize {
    let machines = read_input(input);
    let mut min_presses: Vec<usize> = Vec::new();

    for machine in machines {
        min_presses.push(solve_joltage(&machine));
    }
    debug!("Result: {:?}", min_presses);
    let sum = min_presses.iter().sum::<usize>();
    info!("Sum: {}", sum);
    sum
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
    if let Ok(input) = read_lines("./inputs/day10/test.txt") {
        assert_eq!(part_1(input), 7);
    }
}
#[test]
fn part1_input() {
    if let Ok(input) = read_lines("./inputs/day10/input.txt") {
        assert_eq!(part_1(input), 436);
    }
}
