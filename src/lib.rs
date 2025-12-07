use clap::Parser;
use std::fs::read_to_string;
use std::io;

#[derive(Parser)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: String,

    /// Which part to run (1 or 2)
    #[arg(short, long)]
    pub part: Option<u8>,
}

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let contents = read_to_string(filename)?;
    Ok(contents.lines().map(String::from).collect())
}

pub fn create_grid(input: Vec<String>) -> Vec<Vec<char>> {
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
