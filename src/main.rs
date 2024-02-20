use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

use chrono::{Local, NaiveDate, NaiveDateTime};
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
const STATUS_FILENAME: &str = "status.txt"; // current timestamp status
const TIMESTAMPS_FILENAME: &str = "timestamps.dat"; // historical timestamps
const DATETIME_FORMAT: &str = "%d/%m/%Y--%H:%M:%S";
const DATE_FORMAT: &str = "%d/%m/%Y";
const OPEN_TIMESTAMP_KEYWORD: &str = "OPEN";
const CLOSED_TIMESTAMP_KEYWORD: &str = "CLOSED";
const MINIMUM_WORK_DURATION_MINUTES: i64 = 25;

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

fn read_timestamp_from_string(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, DATETIME_FORMAT)
        .expect("Error reading latest timestamp from file!")
}

fn get_current_status() -> Timestamp {
    let status_line: String = read_first_line_from_file(STATUS_FILENAME);
    let [status, timestamp] = get_two_words_from_line(&status_line);

    if status == OPEN_TIMESTAMP_KEYWORD {
        let timestamp = read_timestamp_from_string(timestamp);
        Timestamp::Open(timestamp)
    } else if status == CLOSED_TIMESTAMP_KEYWORD {
        Timestamp::Closed
    } else {
        panic!("Cannot read status from status.txt file. File corrupted.")
    }
}

fn get_current_datetime() -> NaiveDateTime {
    Local::now().naive_local()
}

fn datetime_to_string(dt: NaiveDateTime) -> String {
    dt.format(DATETIME_FORMAT).to_string()
}
fn date_to_string(dt: NaiveDate) -> String {
    dt.format(DATE_FORMAT).to_string()
}

fn append_line_to_file(line: &str, filename: &str) {
    let mut file = OpenOptions::new().append(true).open(filename).unwrap();

    writeln!(file, "{}", line).unwrap();
}

fn write_line_to_file(line: &str, filename: &str) {
    //overwrites all file contents!
    std::fs::write(line, filename).unwrap();
}

fn set_timestamp_status_open() {
    let line_to_write: String =
        OPEN_TIMESTAMP_KEYWORD.to_owned() + " " + &datetime_to_string(get_current_datetime());

    write_line_to_file(STATUS_FILENAME, line_to_write.as_str())
}

fn minutes_since_last_timestamp(ts: NaiveDateTime) -> i64 {
    Local::now()
        .naive_local()
        .signed_duration_since(ts)
        .num_minutes()
}

fn set_timestamp_status_closed() {
    let line: String = CLOSED_TIMESTAMP_KEYWORD.to_owned() + " TIMESTAMP";
    write_line_to_file(STATUS_FILENAME, &line)
}

fn add_timestamp_to_history(timestamp_duration: i64) {
    let date_of_today: String = date_to_string(Local::now().date_naive());
    let line_to_write: String = date_of_today + " " + &timestamp_duration.to_string();
    append_line_to_file(&line_to_write, TIMESTAMPS_FILENAME);
}

fn main() {
    // let args = Cli::parse();

    let last_timestamp = get_current_status();

    match last_timestamp {
        Timestamp::Open(ts) => {
            let dur = minutes_since_last_timestamp(ts);

            if dur < MINIMUM_WORK_DURATION_MINUTES {
                println!(
                    "Not enough time has passed. You have been working for only {} minutes",
                    dur
                )
            } else {
                println!("Closing timestamp. Time for a break!");
                set_timestamp_status_closed();
                add_timestamp_to_history(dur);
            }
        }
        Timestamp::Closed => {
            println!("Opening timestamp. Time for deep work!");
            set_timestamp_status_open();
        }
    }
}
