use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn getnumber(cardmatches: &HashMap<i32, i32>, start: i32, all: bool) -> i32 {
    let mut total = 0;
    if !all {
        let count = cardmatches[&start];
        if count > 0 {
            for k in 1..count + 1 {
                total += getnumber(&cardmatches, start + k, false);
            }
        }
        total += 1 as i32;
    } else {
        for (&n, _) in cardmatches.iter() {
            total += getnumber(&cardmatches, n, false);
        }
    }
    return total;
}

fn main() {
    let start = Instant::now();
    let cardstext;
    // match read_file("test.txt") {
    match read_file("day4.txt") {
        Ok(contents) => {
            cardstext = contents;
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    let mut res = 0;
    let mut cardmatches = HashMap::new();
    let mut cardvalues = HashMap::new();
    for line in cardstext.lines() {
        let re = Regex::new(r"^Card\s*(\d+):").unwrap();
        let mut card_number: i32 = 0;
        if let Some(caps) = re.captures(line) {
            if let Some(card_number_match) = caps.get(1) {
                if let Ok(cn) = card_number_match.as_str().parse::<i32>() {
                    card_number = cn;
                } else {
                    std::process::exit(1)
                }
            }
        }
        let firstsplit: Vec<&str> = line.split(":").collect();
        let parts: Vec<&str> = firstsplit[1].split(" | ").collect();
        let solution: Vec<i32> = parts[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let found: Vec<i32> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut correctnum: Vec<i32> = Vec::new();
        for a in found {
            if solution.contains(&a) {
                correctnum.push(a);
            }
        }
        let base: i32 = 2;
        let mut val: i32 = 0;
        if correctnum.len() > 0 {
            val = base.pow(correctnum.len() as u32 - 1);
        }
        res += val;
        cardmatches.insert(card_number, correctnum.len() as i32);
        cardvalues.insert(card_number, val);
    }
    println!("Result of part 1:  {}", res);
    println!("{:?}", cardmatches);
    println!("Time: {:?}", start.elapsed());
    println!("Result of part 2:  {}", getnumber(&cardmatches, 1, true));
    println!("Time: {:?}", start.elapsed());
}
