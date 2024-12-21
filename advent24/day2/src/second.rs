#![allow(dead_code)]
use log::{debug, error, info};
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> std::io::Result<String> {
    info!("Attempting to read file: {}", file_name);
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        error!("Failed to read file: {}", e);
        return Err(e);
    }
    info!("Successfully read file: {}", file_name);
    Ok(contents)
}

enum Direction {
    Up,
    Down,
    Unknown,
}

fn check_valid(numbers: &[i32]) -> bool {
    let mut dir: Direction = Direction::Unknown;
    for window in numbers.windows(2) {
        if (window[1] - window[0]).abs() > 3 || window[0] == window[1] {
            return false;
        }
        dir = match dir {
            Direction::Unknown => {
                if window[0] < window[1] {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Up => {
                if window[0] < window[1] {
                    Direction::Up
                } else {
                    return false;
                }
            }
            Direction::Down => {
                if window[0] < window[1] {
                    return false;
                } else {
                    Direction::Down
                }
            }
        };
    }
    true
}

// Checks if the vector is valid if one item is excluded
fn check_valid_omit(numbers: &[i32]) -> bool {
    for n in 0..numbers.len() {
        let mut numbers_copy = numbers.to_owned();
        numbers_copy.remove(n);
        if check_valid(&numbers_copy) {
            return true;
        }
    }
    false
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Please provide a filename as a command line argument.");
        return;
    }
    let contents = match read_file(&args[1]) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Error reading file: {}", e);
            return;
        }
    };
    debug!("File contents: {}", contents);

    // If args[2] is provided, use it to set valid threshold
    let threshold: u8 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    debug!("Threshold: {}", threshold);

    let mut cnt_valid: u32 = 0;
    let mut cnt_valid_omit: u32 = 0;
    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if check_valid(&numbers) {
            cnt_valid += 1;
        }
        if check_valid_omit(&numbers) {
            cnt_valid_omit += 1
        }
        debug!("Line: {}", line);
    }
    println!("Valid lines: {}", cnt_valid);
    println!("Valid lines omitting one: {}", cnt_valid_omit);
}
