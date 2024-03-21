use core::panic;
use std::fs;
use std::path::Path;
use std::process;

mod config;
mod file_io;
mod run;
mod timestamping;

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

fn create_folder_if_it_does_not_exist(path: &Path) {
    if !folder_exists(path) {
        println!("Creating folder at {:?}", &path);
        create_folder(path)
    }
}

fn create_file(file_path: &Path) {
    // Create the file
    fs::File::create(file_path).expect("Could not create file");
}

fn create_file_if_it_does_not_exist(file_path: &Path, file_contents: Option<String>) {
    if !file_exists(file_path) {
        println!("Creating file: {:?}", &file_path);
        create_file(file_path);

        if let Some(content) = file_contents {
            std::fs::write(file_path, content).expect("could not write the new status file!")
        }
    }
}

fn check_and_set_up_app_files() {
    let app_dir = dirs::home_dir()
        .expect("Failed to get user's home directory")
        .join(".rodomopo/");
    let config_dir = dirs::config_dir()
        .expect("Failed to get the config directory")
        .join("rodomopo/");

    let status_file_path = app_dir.join(&config::internal::CONFIG.status_filepath);
    let timestamps_file_path = app_dir.join(&config::internal::CONFIG.timestamps_filepath);
    let config_file_path = app_dir.join(&config::internal::CONFIG.config_filepath);

    create_folder_if_it_does_not_exist(&app_dir);
    create_file_if_it_does_not_exist(&timestamps_file_path, None);
    let default_status_file_content: String =
        config::internal::CONFIG.closed_timestamp_keyword.to_owned() + " TIMESTAMP";
    create_file_if_it_does_not_exist(&status_file_path, Some(default_status_file_content));

    let default_config_file_content: String =
        config::user::UserConfig::serialize_default_user_config_contents();
    create_folder_if_it_does_not_exist(&config_dir);
    create_file_if_it_does_not_exist(&config_file_path, Some(default_config_file_content));
}

fn main() {
    check_and_set_up_app_files();

    run::run().unwrap_or_else(|err| {
        println!("Problem running the program: {err}");
        process::exit(1);
    })
}
