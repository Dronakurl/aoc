#![allow(dead_code)]
use log::{debug, error, info};
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Pair {
    first: u32,
    second: u32,
}

impl Pair {
    fn difference(&self) -> u32 {
        self.first.abs_diff(self.second)
    }
}

struct PairList {
    pairs: Vec<Pair>,
}

impl PairList {
    fn new() -> PairList {
        PairList { pairs: Vec::new() }
    }
    fn add(&mut self, pair: Pair) {
        self.pairs.push(pair);
    }
    fn from_vecs(firsts: Vec<u32>, seconds: Vec<u32>) -> PairList {
        let mut pairs = Vec::new();
        for i in 0..firsts.len() {
            pairs.push(Pair {
                first: firsts[i],
                second: seconds[i],
            });
        }
        PairList { pairs }
    }
    fn get_first(&self) -> Vec<u32> {
        self.pairs.iter().map(|p| p.first).collect()
    }
    fn count_number_of_occurrences_on_right(&self, number: u32) -> u32 {
        self.get_second().iter().filter(|&n| *n == number).count() as u32
    }
    fn get_second(&self) -> Vec<u32> {
        self.pairs.iter().map(|p| p.second).collect()
    }
    fn get_first_sorted(&self) -> Vec<u32> {
        let mut sorted = self.get_first();
        sorted.sort();
        sorted
    }
    fn get_second_sorted(&self) -> Vec<u32> {
        let mut sorted = self.get_second();
        sorted.sort();
        sorted
    }
    fn get_all_sorted(&self) -> Self {
        Self::from_vecs(self.get_first_sorted(), self.get_second_sorted())
    }
    fn sum_of_differences(&self) -> u32 {
        self.pairs.iter().map(|p| p.difference()).sum()
    }
    fn mult_count(&self) -> u32 {
        let mut res: u32 = 0;
        for pair in self.pairs.iter() {
            let count = self.count_number_of_occurrences_on_right(pair.first);
            res += pair.first * count;
        }
        res
    }
}

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
    let mut pairs = PairList::new();
    for line in contents.lines() {
        info!("Line: {}", line);
        let splitline = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let pair = Pair {
            first: splitline[0],
            second: splitline[1],
        };
        debug!("Pair: ({}, {})", pair.first, pair.second);
        pairs.add(pair);
    }
    let sorted_pairs = pairs.get_all_sorted();
    println!("Sum of differences: {}", sorted_pairs.sum_of_differences());
    println!("Multiplication count: {}", pairs.mult_count());
}
