pub static STATUS_FILENAME: &str = "data/status.txt"; // current timestamp status
pub static TIMESTAMPS_FILENAME: &str = "data/timestamps.dat"; // historical timestamps

pub const DATETIME_FORMAT: &str = "%d/%m/%Y--%H:%M:%S";
pub const DATE_FORMAT: &str = "%d/%m/%Y";
pub const OPEN_TIMESTAMP_KEYWORD: &str = "OPEN";
pub const CLOSED_TIMESTAMP_KEYWORD: &str = "CLOSED";
pub const MINIMUM_WORK_BLOCK_DURATION_MINUTES: i64 = 25;
pub const DAILY_WORK_GOAL_MINUTES: i64 = 180;
