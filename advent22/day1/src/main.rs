use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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

fn read_numbers_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.parse::<i32>()
                .map_err(|e| eprintln!("Warning: Could not parse '{line}': {e}"))
                .ok()
        })
        .collect();

    Ok(numbers)
}

fn main() {
    let args = Args::parse();
    let numbers = match read_numbers_from_file(&args.filename) {
        Ok(numbers) => {
            println!("Read {} numbers from file:", numbers.len());
            println!("{numbers:?}");
            numbers
        }
        Err(e) => {
            eprintln!("Could not read {e} {}", args.filename);
            exit(2);
        }
    };
    let increases = numbers.windows(2).filter(|w| w[1] > w[0]).count();
    println!("increases: {increases:?}");
}
