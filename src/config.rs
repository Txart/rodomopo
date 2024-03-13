pub mod constants {
    use dirs;
    use std::path::PathBuf;

    fn preppend_app_dir(filename: &str) -> PathBuf {
        let home_dir = dirs::home_dir().expect("Failed to get user's home directory");
        let app_dir = home_dir.join(".rodomopo/");
        app_dir.join(filename)
    }

    const STATUS_FILENAME: &str = "status.txt"; // current timestamp status
    const TIMESTAMPS_FILENAME: &str = "timestamps.dat"; // historical timestamps

    pub fn get_status_filepath() -> PathBuf {
        preppend_app_dir(STATUS_FILENAME)
    }

    pub fn get_timestamps_filepath() -> PathBuf {
        preppend_app_dir(TIMESTAMPS_FILENAME)
    }

    pub(crate) const DATETIME_FORMAT: &str = "%d/%m/%Y--%H:%M:%S";
    pub(crate) const DATE_FORMAT: &str = "%d/%m/%Y";
    pub(crate) const OPEN_TIMESTAMP_KEYWORD: &str = "OPEN";
    pub(crate) const CLOSED_TIMESTAMP_KEYWORD: &str = "CLOSED";
    pub(crate) const MINIMUM_WORK_BLOCK_DURATION_MINUTES: i64 = 25;
    pub(crate) const DAILY_WORK_GOAL_MINUTES: i64 = 180;
}
