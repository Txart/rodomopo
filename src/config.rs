use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
pub static ref STATUS_FILENAME: PathBuf = PathBuf::from("/home/txart/projects/boss/data/status.txt"); // current timestamp status
pub static ref TIMESTAMPS_FILENAME: PathBuf = PathBuf::from("/hometxartprojects/boss/data/timestamps.dat"); // historical timestamps
}
pub const DATETIME_FORMAT: &str = "%d/%m/%Y--%H:%M:%S";
pub const DATE_FORMAT: &str = "%d/%m/%Y";
pub const OPEN_TIMESTAMP_KEYWORD: &str = "OPEN";
pub const CLOSED_TIMESTAMP_KEYWORD: &str = "CLOSED";
pub const MINIMUM_WORK_BLOCK_DURATION_MINUTES: i64 = 25;
pub const DAILY_WORK_GOAL_MINUTES: i64 = 180;
