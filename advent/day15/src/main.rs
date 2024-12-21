use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn hashme(string: &String) -> u8 {
    let mut hash: u8 = 0;
    for byte in string.bytes() {
        let tmp: u64 = (hash as u64 + byte as u64) * 17;
        hash = tmp as u8;
    }
    hash
}

struct FocalOrder {
    focal: u8,
    order: u8,
}

struct Hashy {
    string: String,
    hash: u8,
    label: String,
    boxnum: u8,
    delete: bool,
    focal: Option<u8>,
}

impl Hashy {
    fn parse(&mut self) {
        let re = Regex::new(r"(\w+)(=|-)(\d*)").unwrap();
        if let Some(captures) = re.captures(&self.string) {
            self.label = captures.get(1).unwrap().as_str().to_string();
            self.boxnum = hashme(&self.label);
            self.delete = captures.get(2).unwrap().as_str().to_string() == "-";
            self.focal = captures.get(3).unwrap().as_str().parse::<u8>().ok();
            // for cap in captures.iter() {
            //     println!("{:?}", cap);
            // }
        } else {
            println!("No captures");
        }
    }
}

impl std::fmt::Display for Hashy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hashy {{ string: {:4}, hash: {:4}, label: {:3} , boxno: {:3}, focal: {:?}}}",
            self.string, self.hash, self.label, self.boxnum, self.focal
        )
    }
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let inputtxt;
    match read_file(&args[1]) {
        Ok(contents) => {
            inputtxt = contents;
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
    let mut hashlst: Vec<Hashy> = Vec::new();
    let mut total: u64 = 0;
    for line in inputtxt.lines() {
        if args.len() >= 3 && args[2] == "--debug" {
            println!("{}", line);
        }
        for value in line.split(',') {
            let mut hsh = Hashy {
                string: value.to_string(),
                hash: 0,
                boxnum: 0,
                label: String::from(""),
                delete: false,
                focal: None,
            };
            hsh.hash = hashme(&hsh.string);
            hsh.parse();
            if args.len() >= 3 && args[2] == "--debug" {
                println!("{}", hsh);
            }
            total += hsh.hash as u64;
            hashlst.push(hsh);
        }
    }
    println!("part 1 : {}", total);
    println!("time elapsed: {:?}", start.elapsed());
    let mut boxes: HashMap<u8, HashMap<String, FocalOrder>> = HashMap::new();
    for hsh in hashlst {
        if hsh.delete == false {
            if let Some(focal) = hsh.focal {
                let boxy = boxes.entry(hsh.boxnum).or_insert_with(HashMap::new);
                if boxy.contains_key(&hsh.label) {
                    if let Some(focal_order) = boxy.get_mut(&hsh.label) {
                        focal_order.focal = focal;
                    }
                } else {
                    let mut maxorder = 0;
                    for (_, value) in boxy.iter() {
                        if value.order > maxorder {
                            maxorder = value.order;
                        }
                    }
                    boxy.insert(
                        hsh.label,
                        FocalOrder {
                            order: maxorder + 1,
                            focal,
                        },
                    );
                }
            }
        } else {
            let boxy = boxes.entry(hsh.boxnum).or_insert_with(HashMap::new);
            if boxy.contains_key(&hsh.label) {
                let ord = boxy[&hsh.label].order;
                for (_, boxm) in boxy.iter_mut() {
                    if boxm.order > ord {
                        boxm.order -= 1;
                    }
                }
                boxy.remove(&hsh.label);
            }
        }
        if args.len() >= 3 && args[2] == "--debug" {
            println!("After {}", hsh.string);
            for (i, boxm) in boxes.iter() {
                if boxm.len() == 0 {
                    continue;
                }
                print!("Box {}: ", i);
                for (ll, lense) in boxm.iter() {
                    print!("[{} order={} f={}] ", ll, lense.order, lense.focal);
                }
                println!("");
            }
        }
    }
    let mut total: u64 = 0;
    for (i, boxm) in boxes.iter() {
        for (k, boxy) in boxm.iter() {
            let res: u64 = (*i as u64 + 1) * boxy.order as u64 * boxy.focal as u64;
            if args.len() >= 3 && args[2] == "--debug" {
                println!(
                    "{} (box {}) * {} slot * {} focal = {}",
                    k,
                    i + 1,
                    boxy.order,
                    boxy.focal,
                    res
                );
            }
            total += res;
        }
    }
    println!("part 2 = {}", total);
    println!("time elapsed: {:?}", start.elapsed());
}
