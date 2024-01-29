use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::Parser;

/// Log working hours
#[derive(Parser)]
struct Cli {
    // /// The pattern to look for
    // pattern: String,
    // /// The path to the file to read
    // path: std::path::PathBuf,
}

fn main() {
    // let args = Cli::parse();

    // Read first line of file
    let f = File::open("timestamps.dat").expect("timestamps file not found!");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line!");
    println!("First line is {:?}", line);
}
