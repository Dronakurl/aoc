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
    println!("File contents: {}", contents);

    #[derive(Debug, PartialEq)]
    enum Direction {
        Up,
        Down,
        Unknown,
        Invalid,
    }

    impl Direction {
        // check if the window is valid
        // Window is valid if the difference between the two numbers is less than 3
        // also the direction should be valid
        // The first entry window[0] is the previous number, the second window[1] is the current number
        fn is_valid(&self, window: &[i32]) -> Direction {
            if (window[1] - window[0]).abs() > 3 {
                debug!("Invalid difference: {:?}", window);
                return Direction::Invalid;
            }
            if window[0] == window[1] {
                debug!("Invalid same number: {:?}", window);
                return Direction::Invalid;
            }
            match self {
                Direction::Down => {
                    if window[0] < window[1] {
                        debug!("Invalid first Up then Down: {:?}", window);
                        Direction::Invalid
                    } else {
                        Direction::Down
                    }
                }
                Direction::Up => {
                    if window[0] > window[1] {
                        debug!("Invalid first Down then Up: {:?}", window);
                        Direction::Invalid
                    } else {
                        Direction::Up
                    }
                }
                Direction::Unknown => {
                    if window[0] < window[1] {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
                _ => {
                    debug!("Invalid: {:?}", self);
                    Direction::Invalid
                }
            }
        }
    }

    // iterate over lines
    let mut cnt_valid = 0;
    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut direction = Direction::Unknown;
        let valid = numbers.windows(2).all(|window| {
            direction = direction.is_valid(window);
            direction != Direction::Invalid
        });

        println!("{} - {:?}", line, direction);
        if valid {
            cnt_valid += 1;
        }
    }
    println!("Valid: {}", cnt_valid);
}
