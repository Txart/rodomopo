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

// constants
// Maybe move to config file?
const TIMESTAMPS_FILENAME: &str = "timestamps.dat";

fn read_first_line_from_file(filename: &str) -> String {
    // Read first line of file
    let f = File::open(filename).expect("file not found!");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read first line!");

    line
}

fn main() {
    // let args = Cli::parse();

    let first_line: String = read_first_line_from_file(TIMESTAMPS_FILENAME);

    // let first_word = match first_line.split(' ').next() {
    //     Some(value) => Some(value),
    //     None => None,
    // };

    let first_word: Option<&str> = first_line.split(' ').next();

    println!("{:?}", first_word);
}
