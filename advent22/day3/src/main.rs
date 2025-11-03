use clap::Parser;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Reads numbers from a text file and stores them in a vector")]
struct Args {
    /// Path to the input file
    #[arg(default_value = "input_example.txt")]
    filename: String,
}

struct Binary {
    number: i32,
}

impl Binary {
    fn get_nth_bit(&self, n: u8) -> u8 {
        (self.number >> n & 1) as u8
    }
}

impl Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl std::str::FromStr for Binary {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = i32::from_str_radix(s, 2)?;
        Ok(Binary { number })
    }
}

struct BinaryVec(Vec<i32>);

impl std::ops::Deref for BinaryVec {
    type Target = [i32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BinaryVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Binary for BinaryVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, n) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{n:b}")?;
        }
        write!(f, "]")
    }
}

fn read_binary_numbers_from_file<P: AsRef<Path>>(
    path: P,
    number_of_bits: &mut u8,
) -> io::Result<Vec<Binary>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let binaries: Vec<Binary> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Binary>().unwrap())
        .collect();

    // Get length of first line
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let first_line = reader.lines().next().unwrap()?;
    *number_of_bits = first_line.len() as u8;

    Ok(binaries)
}

fn main() {
    let args = Args::parse();

    let number_of_bits = &mut 0;
    let numbers = match read_binary_numbers_from_file(&args.filename, number_of_bits) {
        Ok(instructions) => {
            println!("Read {} instructions from file", instructions.len());
            instructions
        }
        Err(e) => {
            eprintln!("Could not read {e} {}", args.filename);
            exit(2);
        }
    };

    // Process instructions here
    println!("{:?} numbers parsed", numbers.len());
    println!("{number_of_bits} bits per number");
    println!("{numbers:?}");

    // Threshhold count: half of the number of bits
    let threshhold = numbers.len() / 2;
    let mut result: u64 = 0;
    for n in 0..*number_of_bits {
        let mut count: usize = 0;
        for number in &numbers {
            count += number.get_nth_bit(n) as usize;
        }
        if count > threshhold {
            result |= 1 << n;
        }
        println!("{n}: {count}");
    }

    // Bitwise invert, but only the first number of bits
    let mask = (1u64 << *number_of_bits) - 1;
    println!("mask {mask:b}");
    let inverted = (!result) & mask;

    println!("{result}");
    println!("{inverted}");

    // print in binary
    println!("{result:b}");
    println!("{inverted:b}");
    println!("result of the first {}", inverted * result);

    // let n = *number_of_bits;
    // let number_of_first_bits: Vec<i32> = numbers.iter().map(|x| (x >> (n - 1)) & 1).collect();
    // println!("{number_of_first_bits:?}");

    let numbers: Vec<i32> = numbers.iter().map(|x| x.number).collect();
    let mut filtered_numbers = BinaryVec(numbers);

    for n in (0..*number_of_bits).rev() {
        println!(
            "bit position {}, remaining numbers: {}",
            n,
            filtered_numbers.len()
        );
        filtered_numbers.0.retain(|x| ((x >> n) & 1) == ((result as i32 >> n) & 1));
        println!("  after filtering for bit {n}: {filtered_numbers:b}");
    }
    println!("final filtered numbers {filtered_numbers:b}");
}
