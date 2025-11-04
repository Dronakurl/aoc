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

#[derive(Clone)]
struct Binary {
    number: u64,
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
        let number = u64::from_str_radix(s, 2)?;
        Ok(Binary { number })
    }
}

#[derive(Clone)]
struct BinaryVec(Vec<Binary>);

impl BinaryVec {
    fn from_file<P: AsRef<Path>>(path: P, number_of_bits: &mut u8) -> io::Result<Self> {
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

        Ok(BinaryVec(binaries))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Binary) -> bool,
    {
        self.0.retain(f);
    }

    fn count_bits_at_position(&self, bit_position: u8) -> usize {
        self.0
            .iter()
            .map(|number| number.get_nth_bit(bit_position) as usize)
            .sum()
    }

    fn get_most_common_bit(&self, bit_position: u8) -> bool {
        let threshold = self.len() / 2;
        let count = self.count_bits_at_position(bit_position);
        count >= threshold
    }

    fn calculate_gamma(&self, number_of_bits: u8) -> u64 {
        let mut result: u64 = 0;
        for n in 0..number_of_bits {
            if self.get_most_common_bit(n) {
                result |= 1 << n;
            }
        }
        result
    }
}

impl std::ops::Deref for BinaryVec {
    type Target = [Binary];

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
        for (i, binary) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:b}", binary.number)?;
        }
        write!(f, "]")
    }
}

impl<'a> IntoIterator for &'a BinaryVec {
    type Item = &'a Binary;
    type IntoIter = std::slice::Iter<'a, Binary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

fn main() {
    let args = Args::parse();

    let number_of_bits = &mut 0;
    let numbers = match BinaryVec::from_file(&args.filename, number_of_bits) {
        Ok(binary_vec) => {
            println!("Read {} instructions from file", binary_vec.len());
            binary_vec
        }
        Err(e) => {
            eprintln!("Could not read {e} {}", args.filename);
            exit(2);
        }
    };

    // Process instructions here
    println!("{:?} numbers parsed", numbers.len());
    println!("{number_of_bits} bits per number");
    println!("{numbers:b}");

    let gamma_rate = numbers.calculate_gamma(*number_of_bits);

    // Bitwise invert, but only the first number of bits
    let mask = (1u64 << *number_of_bits) - 1;
    println!("mask {mask:b}");
    let epsilon_rate = (!gamma_rate) & mask;

    println!("{gamma_rate}");
    println!("{epsilon_rate}");

    // print in binary
    println!("gamma rate = {gamma_rate:b}");
    println!("epsilon rate = {epsilon_rate:b}");
    println!("result of the first {}", epsilon_rate * gamma_rate);

    // let n = *number_of_bits;
    // let number_of_first_bits: Vec<i32> = numbers.iter().map(|x| (x >> (n - 1)) & 1).collect();
    // println!("{number_of_first_bits:?}");

    let mut filtered_numbers = numbers.clone();

    for n in (0..*number_of_bits).rev() {
        println!("  before filtering for bit {n}: {filtered_numbers:b}");
        println!(
            "bit position {}, remaining numbers: {}",
            n,
            filtered_numbers.len()
        );
        let new_gamma = filtered_numbers.calculate_gamma(n);
        println!("new gamma = {new_gamma:b}");
        filtered_numbers
            .retain(|binary| ((binary.number >> n) & 1) == ((new_gamma as u64 >> n) & 1));
        println!("  after  filtering for bit {n}: {filtered_numbers:b}");
    }
    println!("final filtered numbers {filtered_numbers:b}");
}
