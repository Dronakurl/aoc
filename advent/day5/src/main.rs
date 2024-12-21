use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

struct Override {
    dest_range_start: u64,
    source_range_start: u64,
    length: u64,
}

impl Override {
    fn mapper(&self, input: u64) -> (u64, bool) {
        if input >= self.source_range_start && (input < self.source_range_start + self.length) {
            // println!("{} {}", input, self);
            let offset = input - self.source_range_start;
            return (self.dest_range_start + offset, true);
        } else {
            return (input, false);
        }
    }

    fn parse_str(input_str: &str) -> Result<Override, String> {
        let parts: Vec<&str> = input_str.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(format!("Invalid input format : {}", input_str).to_string());
        }

        let dest_range_start = parts[0]
            .parse::<u64>()
            .map_err(|e| format!("Invalid input: {}", e))?;
        let source_range_start = parts[1]
            .parse::<u64>()
            .map_err(|e| format!("Invalid input: {}", e))?;
        let length = parts[2]
            .parse::<u64>()
            .map_err(|e| format!("Invalid input: {}", e))?;

        Ok(Override {
            dest_range_start,
            source_range_start,
            length,
        })
    }
}

impl fmt::Display for Override {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Destination Range Start: {}, Source Range Start: {}, Length: {}",
            self.dest_range_start, self.source_range_start, self.length
        )
    }
}

struct NumberMapper {
    overrides: Vec<Override>,
    source: String,
    dest: String,
}

impl NumberMapper {
    fn map_number(&self, input: u64) -> u64 {
        let mut mapped_number = input;
        let mut swapped: bool;
        for override_item in &self.overrides {
            (mapped_number, swapped) = override_item.mapper(mapped_number);
            if swapped {
                break;
            }
        }
        // println!(
        //     "{} {} --> {} {}",
        //     self.source, input, self.dest, mapped_number,
        // );
        mapped_number
    }

    fn parse_str(paragraph: &Vec<String>) -> Result<NumberMapper, String> {
        let parts: Vec<&str> = paragraph[0].split_whitespace().collect();
        if parts.get(1) != Some(&"map:") {
            return Err(format!("Parse error with line {}", paragraph[0]));
        }
        let pparts: Vec<&str> = parts[0].split("-").collect();
        if pparts.len() != 3 {
            return Err(String::from("Parse error"));
        }
        let mut ovr: Vec<Override> = Vec::new();
        for n in 1..paragraph.len() {
            match Override::parse_str(&paragraph[n]) {
                Ok(myovr) => ovr.push(myovr),
                Err(e) => {
                    println!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Ok(NumberMapper {
            overrides: ovr,
            source: String::from(pparts[0]),
            dest: String::from(pparts[2]),
        })
    }
}

impl std::fmt::Display for NumberMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Source: {:<14}, Destination: {:<14}, overrides = {}",
            self.source,
            self.dest,
            self.overrides.len()
        )?;
        for override_item in &self.overrides {
            write!(f, "\n{}", override_item)?;
        }
        Ok(())
    }
}

fn main() {
    let start = Instant::now();
    let inputtxt;
    match read_file("day5.txt") {
        // match read_file("test.txt") {
        Ok(contents) => {
            inputtxt = contents;
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    // println!("{}", inputtxt);
    let mut seeds: Vec<u64> = Vec::new();
    let mut paragraphs: Vec<Vec<String>> = Vec::new();
    let mut cur_par: Vec<String> = Vec::new();

    for line in inputtxt.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() && parts[0] == "seeds:" {
            for seed_str in &parts[1..] {
                match seed_str.parse::<u64>() {
                    Ok(value) => seeds.push(value),
                    Err(error) => println!("Parsing error: {:?}", error),
                }
            }
        } else if line.is_empty() {
            if !cur_par.is_empty() {
                paragraphs.push(cur_par);
                cur_par = Vec::new();
            }
        } else {
            cur_par.push(String::from(line));
        }
    }
    if !cur_par.is_empty() {
        paragraphs.push(cur_par);
    }
    // println!("paragraphs {:?}", paragraphs);
    let mut nms: Vec<NumberMapper> = Vec::new();
    for p in paragraphs.iter() {
        match NumberMapper::parse_str(p) {
            Ok(nm) => {
                // println!("Found NumberMapper {}", nm);
                nms.push(nm);
            }
            Err(e) => {
                println!("Error parsing: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    let mut vals: Vec<u64> = Vec::new();
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    for (n, seed) in seeds.iter().enumerate() {
        let mut val: u64 = *seed;
        if n > 0 && (n - 1) % 2 == 0 && n > 0 {
            pairs.push((seeds[n - 1], seeds[n]))
        }
        for nm in nms.iter() {
            val = nm.map_number(val)
        }
        println!("seed = {}, val = {} ", seed, val);
        vals.push(val);
    }
    // println!("seeds. {:?}", seeds);
    // println!("pairs. {:?}", pairs);
    println!("part 1: {:?}", vals.iter().min());
    println!("time elapsed: {:?}", start.elapsed());
    let start = Instant::now();
    vals = Vec::new();
    let mut val: u64;
    for (start, len) in pairs {
        println!("seed = {}, len = {} ", start, len);
        for seed in start..(start + len) {
            val = seed;
            for nm in nms.iter() {
                val = nm.map_number(val)
            }
            vals.push(val);
        }
    }
    println!("part 2: {:?}", vals.iter().min());
    println!("time elapsed: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper_within_range() {
        let override_instance = Override {
            dest_range_start: 5,
            source_range_start: 10,
            length: 3,
        };
        assert_eq!(override_instance.mapper(11), (6, true));
    }

    #[test]
    fn test_mapper_on_range() {
        let override_instance = Override {
            dest_range_start: 49,
            source_range_start: 53,
            length: 8,
        };
        assert_eq!(override_instance.mapper(53), (49, true));
    }

    #[test]
    fn test_mapper_another_range() {
        let override_instance = Override {
            dest_range_start: 0,
            source_range_start: 11,
            length: 42,
        };
        assert_eq!(override_instance.mapper(53), (53, false));
    }

    #[test]
    fn test_mapper_outside_range() {
        let override_instance = Override {
            dest_range_start: 5,
            source_range_start: 10,
            length: 3,
        };
        assert_eq!(override_instance.mapper(9), (9, false));
    }
}
