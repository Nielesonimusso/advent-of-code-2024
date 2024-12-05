use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines, Result};

fn file_path(day: u8) -> String {
    format!("/workspaces/rust/advent-of-code-2024/input/{}.txt", day)
}

pub fn get_full_input(day: u8) -> Result<String> {
    read_to_string(file_path(day))
}

pub fn get_lines(day: u8) -> Lines<BufReader<File>> {
    let file: File = File::open(file_path(day)).expect("Input does not exist");
    BufReader::new(file).lines()
}
