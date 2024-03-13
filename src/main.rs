use core::panic;
use std::fs;
use std::path::Path;

mod config;
use crate::config::constants;

fn file_exists(path: &Path) -> bool {
    Path::new(path).exists()
}

fn folder_exists(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() {
            true
        } else {
            panic!(
                "Path to the folder {:?} exists, but it is not a folder. Aborting!",
                path
            );
        }
    } else {
        false
    }
}

fn create_folder(path: &Path) {
    fs::create_dir(path).expect("Could not create {:?} folder!");
}

fn create_file(file_path: &Path) {
    // Create the file
    fs::File::create(file_path).expect("Could not create file");
}

fn write_new_status_file(path: &Path) {
    let line: String = constants::CLOSED_TIMESTAMP_KEYWORD.to_owned() + " TIMESTAMP";
    std::fs::write(path, line).expect("could not write the new status file!");
}

fn if_first_time_set_up_app_files() {
    // Get the path to the app directory, which is under home/user/.boss
    let home_dir = dirs::home_dir().expect("Failed to get user's home directory");
    let app_dir = home_dir.join(".boss/");

    // Check if folder exists; create if not.
    if !folder_exists(&app_dir) {
        println!(
            "The app folder does not exist. Creating one at {:?}",
            app_dir
        );
        create_folder(&app_dir)
    }

    let status_file_path = app_dir.join(constants::get_status_filepath());
    let timestamps_file_path = app_dir.join(constants::get_timestamps_filepath());

    // Check if the status and timestamp files exist; create if not.
    if !file_exists(&status_file_path) {
        println!("Status file not found. Creating it...");
        create_file(&status_file_path);
        write_new_status_file(&status_file_path)
    }
    if !file_exists(&timestamps_file_path) {
        println!("Status file not found. Creating it...");
        create_file(&timestamps_file_path);
    }
}

fn main() {
    if_first_time_set_up_app_files();

    boss::run()
}
