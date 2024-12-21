#![allow(dead_code)]
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Equation {
    x: u64,
    a: Vec<u64>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.x, self.a)
    }
}

impl Equation {
    fn new(x: u64, a: Vec<u64>) -> Self {
        Equation { x, a }
    }

    fn check_valid(&mut self) -> bool {
        let last = self.a.pop().unwrap();
        if self.a.is_empty() {
            self.x == last
        } else if self.x % last == 0 {
            let mut clone = self.clone();
            let newx = clone.x.checked_sub(last);
            let valid = match newx {
                Some(value) => {
                    clone.x = value;
                    clone.check_valid()
                }
                None => return false,
            };
            self.x /= last;
            self.check_valid() || valid
        } else {
            if self.x < last {
                return false;
            }
            self.x -= last;
            self.check_valid()
        }
    }

    fn check_subs(&mut self) -> bool {
        if self.a.is_empty() {
            return false;
        }
        let last = self.a.pop().unwrap();
        let newx = self.x.checked_sub(last);
        match newx {
            Some(value) => {
                self.x = value;
                self.check_validn()
            }
            None => false,
        }
    }

    fn check_div(&mut self) -> bool {
        if self.a.is_empty() {
            return false;
        }
        let last = self.a.pop().unwrap();
        if self.x % last == 0 {
            self.x /= last;
            self.check_validn()
        } else {
            false
        }
    }

    fn check_poplastdig(&mut self) -> bool {
        if self.a.is_empty() {
            return false;
        }
        let last = self.a.pop().unwrap();
        let mut ss: String = format!("{}", self.x);
        let ll: String = format!("{}", last);
        if ss.ends_with(&ll) {
            // remove last digits
            for _ in 0..ll.len() {
                ss.pop();
            }
            if let Ok(value) = ss.parse() {
                self.x = value;
            } else {
                return false;
            }
            self.check_validn()
        } else {
            false
        }
    }

    /// Checks, if it is possible to make the equation valid by combining elements
    fn check_validn(&mut self) -> bool {
        let last = *self.a.last().unwrap();
        if self.a.len() == 1 {
            return self.x == last;
        }
        if self.clone().check_subs() {
            return true;
        }
        if self.clone().check_div() {
            return true;
        }
        if self.clone().check_poplastdig() {
            return true;
        }
        false
    }
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let mut parts = s.split(": ");
        let x = parts.next().unwrap().parse().unwrap();
        let a = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        Equation::new(x, a)
    }
}

fn read_equations_from_file(file_name: &str) -> Vec<Equation> {
    let contents = read_file(file_name).unwrap();
    contents.lines().map(Equation::from).collect()
}

fn read_file(file_name: &str) -> std::io::Result<String> {
    log::info!("Attempting to read file: {}", file_name);
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        log::error!("Failed to read file: {}", e);
        return Err(e);
    }
    log::info!("Successfully read file: {}", file_name);
    Ok(contents)
}

fn main() {
    let start = Instant::now();
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let equations = if args.len() < 2 {
        // log::error!("Please provide a filename as a command line argument.");
        vec![Equation::from("7290: 6 8 6 15")]
    } else {
        read_equations_from_file(&args[1])
    };
    log::debug!("equations: {:?}", equations);

    use rayon::prelude::*;
    let res: u64 = equations
        .clone()
        .into_par_iter()
        .filter_map(|mut eq| {
            log::debug!("{}", eq);
            let orig = eq.x;
            let valid = eq.check_valid();
            if valid {
                log::debug!("valid: {}", valid);
                Some(orig)
            } else {
                None
            }
        })
        .sum();

    log::info!("result: {}", res);

    let res: u64 = equations
        .into_par_iter()
        .filter_map(|mut eq| {
            log::debug!("{}", eq);
            let orig = eq.x;
            let valid = eq.check_validn();
            if valid {
                log::debug!("valid: {}", valid);
                Some(orig)
            } else {
                None
            }
        })
        .sum();

    log::info!("result 2: {}", res);

    log::info!("time elapsed: {:?}", start.elapsed());
}
