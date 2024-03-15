use lazy_static::lazy_static;
use std::path::PathBuf;

pub struct Config {
    pub status_filepath: PathBuf,     // current timestamp status
    pub timestamps_filepath: PathBuf, // historical timestamps
    pub config_filepath: PathBuf,     // config file for user
    pub datetime_format: &'static str,
    pub date_format: &'static str,
    pub open_timestamp_keyword: &'static str,
    pub closed_timestamp_keyword: &'static str,
    pub minimum_work_block_duration_in_minutes: i64,
    pub daily_work_goal_in_minutes: i64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            status_filepath: Self::prepend_app_dir("status.txt"),
            timestamps_filepath: Self::prepend_app_dir("timestamps.dat"),
            config_filepath: Self::prepend_config_dir("config.yaml"),
            datetime_format: "%d/%m/%Y--%H:%M:%S",
            date_format: "%d/%m/%Y",
            open_timestamp_keyword: "OPEN",
            closed_timestamp_keyword: "CLOSED",
            minimum_work_block_duration_in_minutes: 25,
            daily_work_goal_in_minutes: 180,
        }
    }
    fn prepend_app_dir(filename: &str) -> PathBuf {
        let home_dir = dirs::home_dir().expect("Failed to get user's home directory");
        let app_dir = home_dir.join(".rodomopo/");
        app_dir.join(filename)
    }

    fn prepend_config_dir(filename: &str) -> PathBuf {
        let config_dir = dirs::config_dir().expect("Failed to get the config directory");
        let full_config_dir = config_dir.join("rodomopo/");
        config_dir.join(filename)
    }
}

// Use CONFIG as a global variable instead of calling Config::new() everywhere.
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
