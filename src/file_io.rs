use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufRead, BufReader};
use std::path::PathBuf;

pub fn read_file_into_buffer(filename: &PathBuf) -> BufReader<File> {
    let file = File::open(filename).expect("file not found!");
    BufReader::new(file)
}

pub fn read_first_line_from_file(filename: &PathBuf) -> String {
    let reader = read_file_into_buffer(filename);

    let mut line = String::new();
    if let Some(Ok(first_line)) = reader.lines().next() {
        line.push_str(first_line.trim_end());
    }

    line
}

pub fn append_line_to_file(line: &str, filename: &PathBuf) {
    let mut file = OpenOptions::new().append(true).open(filename).unwrap();

    writeln!(file, "{}", line).unwrap();
}

pub fn write_line_to_file(line: &str, filename: &PathBuf) {
    //overwrites all file contents!
    std::fs::write(filename, line).unwrap();
}
