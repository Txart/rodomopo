use chrono::{NaiveDate, NaiveDateTime};
use rustyline::error::ReadlineError;

pub enum TimestampStatus {
    // If the timestamp is open, it is defined by the time when it was opened
    // If it is closed, no more info is needed.
    Open(NaiveDateTime),
    Closed,
}

#[derive(Debug)]
struct Timestamp {
    date: NaiveDate,
    duration: i64,
}
mod config;

mod files_io {
    use std::fs::{File, OpenOptions};
    use std::io::{prelude::*, BufRead, BufReader};
    use std::path::PathBuf;

    pub fn read_file_into_buffer(filename: &PathBuf) -> BufReader<File> {
        let file = File::open(filename).expect("file not found!");
        BufReader::new(file)
    }

    pub fn read_first_line_from_file(filename: &PathBuf) -> String {
        let mut reader = read_file_into_buffer(filename);

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

pub mod timestamping {
    use std::io::BufRead;
    use std::path::PathBuf;

    use crate::config::constants;
    use crate::files_io;
    use crate::Timestamp;
    use crate::TimestampStatus;
    use chrono::{Local, NaiveDate, NaiveDateTime};

    pub fn close_timestamp(timestamp_duration: i64) {
        set_timestamp_status_closed();
        add_timestamp_to_history(timestamp_duration);
    }

    pub fn get_current_status() -> TimestampStatus {
        let status_filepath = constants::get_status_filepath();
        let status_line: String = files_io::read_first_line_from_file(&status_filepath);
        let [status, datetime_string] = get_two_words_from_line(&status_line);

        if status == constants::OPEN_TIMESTAMP_KEYWORD {
            let datetime = datetime_from_string(datetime_string);
            TimestampStatus::Open(datetime)
        } else if status == constants::CLOSED_TIMESTAMP_KEYWORD {
            TimestampStatus::Closed
        } else {
            panic!("Cannot read status from status.txt file. File corrupted.")
        }
    }

    pub fn open_timestamp() {
        let status_filepath = constants::get_status_filepath();
        let line_to_write: String = constants::OPEN_TIMESTAMP_KEYWORD.to_owned()
            + " "
            + &datetime_to_string(get_current_datetime());

        files_io::write_line_to_file(line_to_write.as_str(), &status_filepath)
    }

    pub fn minutes_since_last_datetime(datetime: NaiveDateTime) -> i64 {
        Local::now()
            .naive_local()
            .signed_duration_since(datetime)
            .num_minutes()
    }

    fn get_two_words_from_line(line: &str) -> [&str; 2] {
        let mut words = line.split(' ');
        let first = words.next().expect("could not read first word!");
        let second = words.next().expect("could not read second word!");

        [first, second]
    }

    fn datetime_from_string(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, constants::DATETIME_FORMAT)
            .expect("Error parsing datetime from string")
    }

    fn date_from_string(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, constants::DATE_FORMAT)
            .expect("Error parsing date from string")
    }

    fn get_current_datetime() -> NaiveDateTime {
        Local::now().naive_local()
    }

    fn datetime_to_string(dt: NaiveDateTime) -> String {
        dt.format(constants::DATETIME_FORMAT).to_string()
    }
    fn date_to_string(dt: NaiveDate) -> String {
        dt.format(constants::DATE_FORMAT).to_string()
    }

    pub fn set_timestamp_status_closed() {
        let status_filepath = constants::get_status_filepath();
        let line: String = constants::CLOSED_TIMESTAMP_KEYWORD.to_owned() + " TIMESTAMP";
        files_io::write_line_to_file(&line, &status_filepath);
    }

    fn get_date_of_today() -> NaiveDate {
        Local::now().date_naive()
    }

    fn add_timestamp_to_history(timestamp_duration: i64) {
        let timestamps_filepath = constants::get_timestamps_filepath();
        let date_of_today: String = date_to_string(get_date_of_today());
        let line_to_write: String = date_of_today + " " + &timestamp_duration.to_string();
        files_io::append_line_to_file(&line_to_write, &timestamps_filepath);
    }

    fn compute_working_hours_in_a_day(day: NaiveDate) -> i64 {
        let timestamps: Vec<Timestamp> = get_all_timestamps_in_a_day_from_file(day);
        sum_minutes_in_timestamps(timestamps)
    }

    fn get_timestamp_from_line(line: String) -> Timestamp {
        let binding = line.to_string();
        let [date, duration] = get_two_words_from_line(&binding);

        Timestamp {
            date: date_from_string(date),
            duration: duration
                .parse::<i64>()
                .expect("could not parse number from string"),
        }
    }

    fn get_all_timestamps_in_a_day_from_file(day: NaiveDate) -> Vec<Timestamp> {
        let timestamps_filepath = constants::get_timestamps_filepath();
        let reader = files_io::read_file_into_buffer(&timestamps_filepath);

        let mut timestamps_of_today: Vec<Timestamp> = Vec::new();
        for line in reader.lines() {
            let line_ts = get_timestamp_from_line(line.expect("could not parse line!"));
            if day == line_ts.date {
                timestamps_of_today.push(line_ts);
            }
        }
        timestamps_of_today
    }

    fn sum_minutes_in_timestamps(timestamps: Vec<Timestamp>) -> i64 {
        let mut total_minutes: i64 = 0;
        for ts in timestamps {
            total_minutes += ts.duration
        }
        total_minutes
    }

    fn create_progress_bar(completed: usize, total: usize) -> String {
        // The completed part of the bar is made of x-1 "=" and 1 ">", e.g., ====>
        // If nothing has been completed, show 0, not -1 "=".
        let completed_to_display: usize = match completed {
            0 => 0,
            completed => completed - 1,
        };
        let completed_part: String = "=".repeat(completed_to_display);
        let remaining_part: String = "-".repeat(total - completed_to_display);

        "[".to_string() + &completed_part + ">" + &remaining_part + "]"
    }

    fn show_progress_bar(progress: f64, goal: f64, bar_char_length: usize) {
        let completed: usize = (progress * bar_char_length as f64 / goal).round() as usize;

        let progress_bar = create_progress_bar(completed, bar_char_length);

        println!("{}", progress_bar);
    }

    pub fn show_progress_for_today() {
        let today = get_date_of_today();
        let working_hours = compute_working_hours_in_a_day(today);
        let goal = constants::DAILY_WORK_GOAL_MINUTES;

        show_progress_bar(working_hours as f64, goal as f64, 20);

        println!("{} minutes out of {} worked today.\n", working_hours, goal);
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
            assert_eq!(sum_minutes_in_timestamps(timestamps), 70);

            let timestamps = Vec::new();
            assert_eq!(sum_minutes_in_timestamps(timestamps), 0);
        }

        #[test]
        fn gets_timestamp_from_line() {
            let line: String = String::from("21/02/2024 67");
            let ts = get_timestamp_from_line(line);

            let date: NaiveDate = date_from_string("21/02/2024");

            assert_eq!(date, ts.date);
            assert_eq!(67, ts.duration);
        }
    }
}

fn trigger_yes_no_question(question: &str) -> bool {
    let mut rl = rustyline::DefaultEditor::new().expect("could not trigger for user input!");

    loop {
        let readline = rl.readline(question);
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match line.trim().to_lowercase().as_str() {
                    "yes" | "y" => return true,
                    "no" | "n" => return false,
                    &_ => println!("Invalid input. It should be yes or no"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                panic!("Program stopped by CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                panic!("Program stopped by CTRL-D");
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }
}

pub fn run() {
    use config::constants;
    // let args = Cli::parse();
    let last_timestamp_status = crate::timestamping::get_current_status();

    match last_timestamp_status {
        TimestampStatus::Open(datetime) => {
            let dur = crate::timestamping::minutes_since_last_datetime(datetime);

            if dur >= constants::DAILY_WORK_GOAL_MINUTES {
                crate::timestamping::open_timestamp();

                println!(
                    "You left a timestamp open {} minutes ago.\nI am assuming it is not valid: I will delete it and open a new timestamp.",
                    dur
                );
            } else if dur < constants::MINIMUM_WORK_BLOCK_DURATION_MINUTES {
                println!(
                    "Not enough time has passed. You have been working only for {dur} minutes"
                );
                let answer = trigger_yes_no_question(
                    "Do you want to betray your principles? [y]es or [n]o.",
                );
                match answer {
                    true => {
                        crate::timestamping::close_timestamp(dur);
                        println!(
                            "Betraying your principles and closing timestamp. Time for a break!"
                        );
                    }
                    false => println!("Good! Keep working!"),
                }
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

    println!("\n");
    crate::timestamping::show_progress_for_today();
}
