use core::panic;
use std::env;
use std::path::{Path, PathBuf};
mod config;

fn file_exists(path: &Path) -> bool {
    Path::new(path).exists()
}

fn check_project_files_exist() {
    // Get the path to the current project directory
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get project directory");

    let project_dir = PathBuf::from(project_dir);
    // Create the full path by joining the executable directory with the relative path
    let status_file_path = project_dir.join(config::STATUS_FILENAME);
    let timestamps_file_path = project_dir.join(config::TIMESTAMPS_FILENAME);

    // Check if the status and timestamp files exist
    if !file_exists(&status_file_path) {
        panic!(
            "I was looking for {:?}, but it does not exist!",
            status_file_path
        );
    }
    if !file_exists(&timestamps_file_path) {
        panic!("timestamps.dat file does not exist!");
    }
}

fn main() {
    check_project_files_exist();

    boss::run()
}
