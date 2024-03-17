use core::panic;
use std::fs;
use std::path::Path;

mod config;
use config::CONFIG;

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
        create_folder(path)
    }
}

fn create_file(file_path: &Path) {
    // Create the file
    fs::File::create(file_path).expect("Could not create file");
}

fn create_file_if_it_does_not_exist(file_path: &Path, file_contents: Option<String>) {
    if !file_exists(file_path) {
        create_file(file_path);

        if let Some(content) = file_contents {
            std::fs::write(file_path, content).expect("could not write the new status file!")
        }
    }
}

fn if_first_time_set_up_app_files() {
    let home_dir = dirs::home_dir().expect("Failed to get user's home directory");
    let app_dir = home_dir.join(".rodomopo/");

    let config_dir = dirs::config_dir()
        .expect("Failed to get the config directory")
        .join("rodomopo/");

    let status_file_path = app_dir.join(&CONFIG.status_filepath);
    let timestamps_file_path = app_dir.join(&CONFIG.status_filepath);
    let config_file_path = app_dir.join(&CONFIG.config_filepath);

    println!("Creating application files at {:?}", &app_dir);
    create_folder_if_it_does_not_exist(&app_dir);
    create_file_if_it_does_not_exist(&timestamps_file_path, None);
    let default_status_file_content: String =
        CONFIG.closed_timestamp_keyword.to_owned() + " TIMESTAMP";
    create_file_if_it_does_not_exist(&status_file_path, Some(default_status_file_content));

    println!("Creating config files at {:?}", &config_dir);
    let default_config_file_content: String = String::from("lalalla: jajajja, \n tititi: 42");
    create_folder_if_it_does_not_exist(&config_dir);
    create_file_if_it_does_not_exist(&config_file_path, Some(default_config_file_content));
}

fn main() {
    if_first_time_set_up_app_files();

    rodomopo::run()
}
