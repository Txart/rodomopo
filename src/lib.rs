use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use chrono::{Local, NaiveDate, NaiveDateTime};
use clap::Parser;

mod config;

/// Log working hours
#[derive(Parser)]
struct Cli {
    // /// The pattern to look for
    // pattern: String,
    // /// The path to the file to read
    // path: std::path::PathBuf,
}

enum Timestamp {
    // If the timestamp is open, it is defined by the time when it was opened
    // If it is closed, no more info is needed.
    Open(NaiveDateTime),
    Closed,
}

fn read_first_line_from_file(filename: &PathBuf) -> String {
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
    NaiveDateTime::parse_from_str(s, config::DATETIME_FORMAT)
        .expect("Error reading latest timestamp from string")
}

fn get_current_status() -> Timestamp {
    let status_line: String = read_first_line_from_file(&config::STATUS_FILENAME);
    let [status, timestamp] = get_two_words_from_line(&status_line);

    if status == config::OPEN_TIMESTAMP_KEYWORD {
        let timestamp = read_timestamp_from_string(timestamp);
        Timestamp::Open(timestamp)
    } else if status == config::CLOSED_TIMESTAMP_KEYWORD {
        Timestamp::Closed
    } else {
        panic!("Cannot read status from status.txt file. File corrupted.")
    }
}

fn get_current_datetime() -> NaiveDateTime {
    Local::now().naive_local()
}

fn datetime_to_string(dt: NaiveDateTime) -> String {
    dt.format(config::DATETIME_FORMAT).to_string()
}
fn date_to_string(dt: NaiveDate) -> String {
    dt.format(config::DATE_FORMAT).to_string()
}

fn append_line_to_file(line: &str, filename: &PathBuf) {
    let mut file = OpenOptions::new().append(true).open(filename).unwrap();

    writeln!(file, "{}", line).unwrap();
}

fn write_line_to_file(line: &str, filename: &PathBuf) {
    //overwrites all file contents!
    std::fs::write(filename, line).unwrap();
}

fn open_timestamp() {
    let line_to_write: String = config::OPEN_TIMESTAMP_KEYWORD.to_owned()
        + " "
        + &datetime_to_string(get_current_datetime());

    write_line_to_file(line_to_write.as_str(), &config::STATUS_FILENAME);
}

fn minutes_since_last_timestamp(ts: NaiveDateTime) -> i64 {
    Local::now()
        .naive_local()
        .signed_duration_since(ts)
        .num_minutes()
}

fn set_timestamp_status_closed() {
    let line: String = config::CLOSED_TIMESTAMP_KEYWORD.to_owned() + " TIMESTAMP";
    write_line_to_file(&line, &config::STATUS_FILENAME);
}

fn add_timestamp_to_history(timestamp_duration: i64) {
    let date_of_today: String = date_to_string(Local::now().date_naive());
    let line_to_write: String = date_of_today + " " + &timestamp_duration.to_string();
    append_line_to_file(&line_to_write, &config::TIMESTAMPS_FILENAME);
}

fn close_timestamp(timestamp_duration: i64) {
    set_timestamp_status_closed();
    add_timestamp_to_history(timestamp_duration);
}

pub fn run() {
    // let args = Cli::parse();
    let last_timestamp = get_current_status();

    match last_timestamp {
        Timestamp::Open(ts) => {
            let dur = minutes_since_last_timestamp(ts);

            if dur >= config::DAILY_WORK_GOAL_MINUTES {
                open_timestamp();

                println!(
                    "You left a timestamp open {} minutes ago.\nI am assuming it is not valid: I will delete it and open a new timestamp.",
                    dur
                );
            } else if dur < config::MINIMUM_WORK_BLOCK_DURATION_MINUTES {
                println!(
                    "Not enough time has passed. You have been working only for {} minutes",
                    dur
                )
            } else {
                println!("Closing timestamp. Time for a break!");
                close_timestamp(dur);
            }
        }
        Timestamp::Closed => {
            println!("Opening timestamp. Time for deep work!");
            open_timestamp();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_two_words() {
        let line = "two words";
        let [first, second] = get_two_words_from_line(line);
        assert_eq!(first, "two");
        assert_eq!(second, "words");
    }
}
