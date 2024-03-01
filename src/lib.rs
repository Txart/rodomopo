use chrono::{NaiveDate, NaiveDateTime};
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

enum TimestampStatus {
    // If the timestamp is open, it is defined by the time when it was opened
    // If it is closed, no more info is needed.
    Open(NaiveDateTime),
    Closed,
}

struct Timestamp {
    date: NaiveDate,
    duration: i64,
}

mod files_io {
    use std::fs::{File, OpenOptions};
    use std::io::{prelude::*, BufRead, BufReader};
    use std::path::PathBuf;

    pub fn read_first_line_from_file(filename: &PathBuf) -> String {
        // Read first line of file
        let f = File::open(filename).expect("file not found!");
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        reader
            .read_line(&mut line)
            .expect("Failed to read first line!");

        line
    }

    pub fn append_line_to_file(line: &str, filename: &PathBuf) {
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();

        writeln!(file, "{}", line).unwrap();
    }

    pub fn write_line_to_file(line: &str, filename: &PathBuf) {
        //overwrites all file contents!
        std::fs::write(filename, line).unwrap();
    }
}

mod timestamping {
    use crate::config;
    use crate::files_io;
    use crate::Timestamp;
    use crate::TimestampStatus;
    use chrono::{Local, NaiveDate, NaiveDateTime};

    pub fn close_timestamp(timestamp_duration: i64) {
        set_timestamp_status_closed();
        add_timestamp_to_history(timestamp_duration);
    }

    pub fn get_current_status() -> TimestampStatus {
        let status_line: String = files_io::read_first_line_from_file(&config::STATUS_FILENAME);
        let [status, datetime_string] = get_two_words_from_line(&status_line);

        if status == config::OPEN_TIMESTAMP_KEYWORD {
            let datetime = read_datetime_from_string(datetime_string);
            TimestampStatus::Open(datetime)
        } else if status == config::CLOSED_TIMESTAMP_KEYWORD {
            TimestampStatus::Closed
        } else {
            panic!("Cannot read status from status.txt file. File corrupted.")
        }
    }

    pub fn open_timestamp() {
        let line_to_write: String = config::OPEN_TIMESTAMP_KEYWORD.to_owned()
            + " "
            + &datetime_to_string(get_current_datetime());

        files_io::write_line_to_file(line_to_write.as_str(), &config::STATUS_FILENAME);
    }

    pub fn minutes_since_last_datetime(datetime: NaiveDateTime) -> i64 {
        Local::now()
            .naive_local()
            .signed_duration_since(datetime)
            .num_minutes()
    }

    fn get_two_words_from_line(line: &str) -> [&str; 2] {
        let mut words = line.split(' ');
        let first = words.next().unwrap();
        let second = words.next().unwrap();

        [first, second]
    }

    fn read_datetime_from_string(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, config::DATETIME_FORMAT)
            .expect("Error reading latest timestamp from string")
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

    fn set_timestamp_status_closed() {
        let line: String = config::CLOSED_TIMESTAMP_KEYWORD.to_owned() + " TIMESTAMP";
        files_io::write_line_to_file(&line, &config::STATUS_FILENAME);
    }

    fn get_date_of_today() -> NaiveDate {
        Local::now().date_naive()
    }

    fn add_timestamp_to_history(timestamp_duration: i64) {
        let date_of_today: String = date_to_string(get_date_of_today());
        let line_to_write: String = date_of_today + " " + &timestamp_duration.to_string();
        files_io::append_line_to_file(&line_to_write, &config::TIMESTAMPS_FILENAME);
    }

    fn compute_working_hours_in_a_day(day: NaiveDate) -> i64 {
        //TODO
        // timestamps = get_all_timestamps_in_a_day()
        // sum_minutes_in_timestamps(timestamps)
    }

    fn get_all_timestamps_in_a_day(day: NaiveDate) -> Vec<Timestamp> {
        //TODO
        //  - read all lines in timestamps.dat
        //  - Filter those whose date matches today's NaiveDate
        //  - Create a Timestamp(Naiveedate, duration) with each of those
        //  - Append them to a Vec<Timestamp>
    }

    fn sum_minutes_in_timestamps(timestamps: Vec<Timestamp>) -> i64 {
        let mut total_minutes: i64 = 0;
        for ts in timestamps {
            total_minutes += ts.duration
        }
        total_minutes
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

        #[test]
        fn sums_mins_in_timestamps() {
            let ts1 = Timestamp {
                date: get_date_of_today(),
                duration: 30,
            };
            let ts2 = Timestamp {
                date: get_date_of_today(),
                duration: 40,
            };
            let timestamps = vec![ts1, ts2];
            assert_eq!(sum_minutes_in_timestamps(timestamps), 70)
        }
    }
}

pub fn run() {
    // let args = Cli::parse();
    let last_timestamp_status = crate::timestamping::get_current_status();

    match last_timestamp_status {
        TimestampStatus::Open(datetime) => {
            let dur = crate::timestamping::minutes_since_last_datetime(datetime);

            if dur >= config::DAILY_WORK_GOAL_MINUTES {
                crate::timestamping::open_timestamp();

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
                crate::timestamping::close_timestamp(dur);
            }
        }
        TimestampStatus::Closed => {
            println!("Opening timestamp. Time for deep work!");
            crate::timestamping::open_timestamp();
        }
    }
}
