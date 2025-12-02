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
