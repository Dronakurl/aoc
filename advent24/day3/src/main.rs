#![allow(dead_code)]
use log::{debug, error, info};
use regex::Regex;
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
    debug!("File contents: {}", contents);

    let mut result: u64 = 0;
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    for cap in re.captures_iter(&contents) {
        let num1: u64 = cap[1].parse().unwrap();
        let num2: u64 = cap[2].parse().unwrap();
        result += num1 * num2;
        debug!("Found: {} * {} = {}", num1, num2, num1 * num2);
    }
    println!("Total sum of all multiplications: {}", result);

    let mut result: u64 = 0;
    let chars: Vec<char> = contents.chars().collect();
    let mut i = 0;
    let mut activated = true;
    while i < chars.len() {
        if chars[i..].starts_with(&['d', 'o', '(', ')']) {
            debug!("Found: do()");
            activated = true;
            i += 4;
        } else if chars[i..].starts_with(&['d', 'o', 'n', '\'', 't', '(', ')']) {
            debug!("Found: don't()");
            activated = false;
            i += 7;
        } else if chars[i..].starts_with(&['m', 'u', 'l', '(']) {
            if !activated {
                i += 4;
                continue;
            }
            i += 4; // Skip "mul("
            let mut num1 = String::new();
            while chars[i].is_ascii_digit() {
                num1.push(chars[i]);
                i += 1;
            }
            if chars[i] == ',' {
                i += 1;
            } else {
                continue;
            }
            let mut num2 = String::new();
            while chars[i].is_ascii_digit() {
                num2.push(chars[i]);
                i += 1;
            }
            if chars[i] == ')' {
                i += 1; // Skip ")"
            } else {
                continue;
            }
            let number1: u64 = num1.parse().unwrap();
            let number2: u64 = num2.parse().unwrap();
            debug!("Found: mul({}, {})", number1, number2);
            result += number1 * number2;
        } else {
            i += 1;
        }
    }
    println!("Total sum of all activated multiplications: {}", result);
}
