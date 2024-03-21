use chrono::{Local, NaiveDate, NaiveDateTime};
use std::io;
use std::io::BufRead;

use crate::config::internal::CONFIG;
use crate::config::user::UserConfig;
use crate::file_io;
use crate::run::Timestamp;
use crate::run::TimestampStatus;

pub fn close_timestamp(timestamp_duration: i64) {
    set_timestamp_status_closed();
    add_timestamp_to_history(timestamp_duration);
}

pub fn get_current_status() -> Result<TimestampStatus, io::Error> {
    let status_line: String = file_io::read_first_line_from_file(&CONFIG.status_filepath)?;
    let [status, datetime_string] = get_two_words_from_line(&status_line)?;

    if status == CONFIG.open_timestamp_keyword {
        let datetime = datetime_from_string(datetime_string, CONFIG.datetime_format);
        Ok(TimestampStatus::Open(datetime))
    } else if status == CONFIG.closed_timestamp_keyword {
        Ok(TimestampStatus::Closed)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid status"))
    }
}

pub fn open_timestamp() {
    let line_to_write: String = CONFIG.open_timestamp_keyword.to_owned()
        + " "
        + &datetime_to_string(get_current_datetime());

    file_io::write_line_to_file(line_to_write.as_str(), &CONFIG.status_filepath)
}

pub fn minutes_since_last_datetime(datetime: NaiveDateTime) -> i64 {
    Local::now()
        .naive_local()
        .signed_duration_since(datetime)
        .num_minutes()
}

fn get_two_words_from_line(line: &str) -> Result<[&str; 2], io::Error> {
    let mut words = line.split(' ');
    let first = words.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Could not read first word",
    ))?;
    let second = words.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Could not read second word",
    ))?;

    Ok([first, second])
}

fn datetime_from_string(s: &str, fmt: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, fmt).expect("Error parsing datetime from string")
}

fn date_from_string(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, CONFIG.date_format).expect("Error parsing date from string")
}

fn get_current_datetime() -> NaiveDateTime {
    Local::now().naive_local()
}

fn datetime_to_string(dt: NaiveDateTime) -> String {
    dt.format(CONFIG.datetime_format).to_string()
}
fn date_to_string(dt: NaiveDate) -> String {
    dt.format(CONFIG.date_format).to_string()
}

pub fn set_timestamp_status_closed() {
    let line: String = CONFIG.closed_timestamp_keyword.to_owned() + " TIMESTAMP";
    file_io::write_line_to_file(&line, &CONFIG.status_filepath);
}

fn get_date_of_today() -> NaiveDate {
    Local::now().date_naive()
}

fn add_timestamp_to_history(timestamp_duration: i64) {
    let date_of_today: String = date_to_string(get_date_of_today());
    let line_to_write: String = date_of_today + " " + &timestamp_duration.to_string();
    file_io::append_line_to_file(&line_to_write, &CONFIG.timestamps_filepath);
}

fn compute_working_hours_in_a_day(day: NaiveDate) -> Result<i64, io::Error> {
    let timestamps: Vec<Timestamp> = get_all_timestamps_in_a_day_from_file(day)?;
    Ok(sum_minutes_in_timestamps(timestamps))
}

fn get_timestamp_from_line(line: String) -> Result<Timestamp, io::Error> {
    let binding = line.to_string();
    let [date, duration] = get_two_words_from_line(&binding)?;

    Ok(Timestamp {
        date: date_from_string(date),
        duration: duration
            .parse::<i64>()
            .expect("could not parse number from string"),
    })
}

fn get_all_timestamps_in_a_day_from_file(day: NaiveDate) -> Result<Vec<Timestamp>, std::io::Error> {
    let reader = file_io::read_file_into_buffer(&CONFIG.timestamps_filepath)?;

    let mut timestamps_of_today: Vec<Timestamp> = Vec::new();
    for line in reader.lines() {
        let line_ts = get_timestamp_from_line(line.expect("could not parse line!"))?;
        if day == line_ts.date {
            timestamps_of_today.push(line_ts);
        }
    }
    Ok(timestamps_of_today)
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

pub fn show_progress_for_today(user_config: &UserConfig) -> Result<(), io::Error> {
    let today = get_date_of_today();
    let working_hours = compute_working_hours_in_a_day(today)?;
    let goal = user_config.daily_work_goal_in_minutes;

    show_progress_bar(working_hours as f64, goal as f64, 20);

    println!("{} minutes out of {} worked today.\n", working_hours, goal);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn it_gets_two_words() -> Result<(), io::Error> {
        let line = "two words";
        let [first, second] = get_two_words_from_line(line)?;
        assert_eq!(first, "two");
        assert_eq!(second, "words");

        Ok(())
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
    fn gets_timestamp_from_line() -> Result<(), io::Error> {
        let line: String = String::from("21/02/2024 67");
        let ts = get_timestamp_from_line(line)?;

        let date: NaiveDate = date_from_string("21/02/2024");

        assert_eq!(date, ts.date);
        assert_eq!(67, ts.duration);

        Ok(())
    }

    #[test]
    fn gets_status_from_line() -> Result<(), io::Error> {
        let line: String = String::from("OPEN 14/03/2024--10:28:01");
        let [_status, datetime_string] = get_two_words_from_line(&line)?;
        let datetime: NaiveDateTime = datetime_from_string(datetime_string, CONFIG.datetime_format);

        let dt = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(),
            NaiveTime::from_hms_opt(10, 28, 1).unwrap(),
        );

        assert_eq!(datetime, dt);

        Ok(())
    }

    #[test]
    fn gets_datetime_from_string() {
        let s_datetime: String = String::from("14/03/2024--10:28:01");
        let s_date: String = String::from("14/03/2024");
        let d = NaiveDate::from_ymd_opt(2024, 3, 14).unwrap();
        let t = NaiveTime::from_hms_opt(10, 28, 1).unwrap();

        let dt = NaiveDateTime::new(d, t);

        assert_eq!(date_from_string(&s_date), d);
        assert_eq!(
            datetime_from_string(&s_datetime, CONFIG.datetime_format),
            dt
        );
    }
}
