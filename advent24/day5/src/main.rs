#![allow(dead_code)]
use log::{debug, error, info};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

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

trait ComesBefore {
    fn comes_before(&self, mapper: &HashMap<u16, Vec<u16>>, right: u16) -> bool;
}

impl ComesBefore for u16 {
    fn comes_before(&self, mapper: &HashMap<u16, Vec<u16>>, right: u16) -> bool {
        if let Some(allowed) = mapper.get(self) {
            return allowed.contains(&right);
        }
        false
    }
}

fn sort_with_mapper(vec: &mut [u16], mapper: &HashMap<u16, Vec<u16>>) {
    vec.sort_by(|a, b| {
        if a.comes_before(mapper, *b) {
            std::cmp::Ordering::Less
        } else if b.comes_before(mapper, *a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

fn get_middle_element(vec: &[u16]) -> u16 {
    if vec.len() % 2 == 0 {
        log::trace!("Even: {:?}", vec[vec.len() / 2]);
        vec[vec.len() / 2]
    } else {
        log::trace!("Odd: {:?}", vec[(vec.len() - 1) / 2]);
        vec[(vec.len() - 1) / 2]
    }
}

fn check_sequence(seq: &[u16], mapper: &HashMap<u16, Vec<u16>>) -> bool {
    for i in (0..seq.len()).rev() {
        // get all the elements before the number
        let slice = &seq[0..i].to_vec();
        let notallowed = &mapper.get(&seq[i]);
        // Check if any of notallowed are in slice
        if let Some(notallowed) = notallowed {
            if notallowed.iter().any(|x| slice.contains(x)) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let start = Instant::now();
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

    // lists all numbers that must be after a number
    let mut mapper: HashMap<u16, Vec<u16>> = HashMap::new();

    // the sequences that need to be tested for validity
    let mut seqs: Vec<Vec<u16>> = Vec::new();

    let mut lines = contents.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let nums: Vec<u16> = line.split('|').map(|x| x.parse().unwrap()).collect();
        mapper.entry(nums[0]).or_default().push(nums[1]);
    }

    for line in lines {
        seqs.push(line.split(',').map(|x| x.parse().unwrap()).collect());
    }
    debug!("Mapper: {:?}", mapper);
    debug!("Sequences: {:?}", seqs);

    let mut result = 0;
    let mut res2 = 0;
    for mut seq in seqs {
        let res = check_sequence(&seq, &mapper);
        let xx: Vec<u16> = seq.clone();
        let sorted = xx.is_sorted_by(|a, b| {
            if a.comes_before(&mapper, *b) {
                true
            } else if b.comes_before(&mapper, *a) {
                false
            } else {
                true
            }
        });
        assert_eq!(sorted, res);

        log::trace!("Sequence: {:?} is valid: {}", seq, res);

        if res {
            result += get_middle_element(&seq)
        } else {
            log::trace!("Before: {:?}", seq);
            sort_with_mapper(&mut seq, &mapper);
            log::trace!("Sorted: {:?}", seq);
            res2 += get_middle_element(&seq);
        }
    }

    log::info!("Result: {}", result);
    log::info!("Result: {}", res2);
    log::info!("time elapsed: {:?}", start.elapsed());
}
