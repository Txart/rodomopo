use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use chrono::{NaiveDateTime, Utc};
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
const TIMESTAMP_FORMAT: &str = "%d/%m/%Y--%H:%M:%S";
const OPEN_TIMESTAMP_KEYWORD: &str = "OPEN";

#[derive(Debug)]
enum Timestamp {
    // If the timestamp is open, it is defined by the time when it was opened
    // If it is closed, no more info is needed.
    Open(NaiveDateTime),
    Closed,
}

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

fn get_two_words_from_line(line: &str) -> [&str; 2] {
    let mut words = line.split(' ');
    let first = words.next().unwrap();
    let second = words.next().unwrap();

    [first, second]
}

fn read_last_timestamp() -> Timestamp {
    let timestamps_first_line: String = read_first_line_from_file(TIMESTAMPS_FILENAME);
    let [status, timestamp] = get_two_words_from_line(&timestamps_first_line);

    if status == OPEN_TIMESTAMP_KEYWORD {
        let timestamp = NaiveDateTime::parse_from_str(timestamp, TIMESTAMP_FORMAT)
            .expect("Error reading latest timestamp from file!");
        Timestamp::Open(timestamp)
    } else {
        Timestamp::Closed
    }
}

fn main() {
    // let args = Cli::parse();

    let last_timestamp = read_last_timestamp();
    println!("Last timestamp: {:?}", last_timestamp);

    match last_timestamp {
        Timestamp::Open(ts) => {
            let dur = ts.signed_duration_since(Utc::now().naive_local());
            println!("Duration since last open timestamp: {}", dur);
        }
        Timestamp::Closed => {
            println!("TODO: add new timestamp!");
        }
    }
}
