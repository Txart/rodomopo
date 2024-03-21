use chrono::{NaiveDate, NaiveDateTime};
use rustyline::error::ReadlineError;
use std::io;

use crate::config;

pub enum TimestampStatus {
    // If the timestamp is open, it is defined by the time when it was opened
    // If it is closed, no more info is needed.
    Open(NaiveDateTime),
    Closed,
}

#[derive(Debug)]
pub struct Timestamp {
    pub date: NaiveDate,
    pub duration: i64,
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

pub fn run() -> Result<(), io::Error> {
    let user_config = config::user::UserConfig::new();
    // let args = Cli::parse();
    let last_timestamp_status = crate::timestamping::get_current_status()?;

    match last_timestamp_status {
        TimestampStatus::Open(datetime) => {
            let dur = crate::timestamping::minutes_since_last_datetime(datetime);

            if dur >= user_config.daily_work_goal_in_minutes {
                crate::timestamping::open_timestamp();

                println!(
                    "You left a timestamp open {} minutes ago.\nI am assuming it is not valid: I will delete it and open a new timestamp.",
                    dur
                );
            } else if dur < user_config.minimum_work_block_duration_in_minutes {
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
    let _ = crate::timestamping::show_progress_for_today(&user_config);

    Ok(())
}
