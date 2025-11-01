use clap::Parser;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Reads numbers from a text file and stores them in a vector")]
struct Args {
    /// Path to the input file
    #[arg(default_value = "input_example.txt")]
    filename: String,
}

struct Coordinates {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(horizontal: {}, depth: {}, aim: {}) = {}",
            self.horizontal,
            self.depth,
            self.aim,
            self.horizontal * self.depth
        )
    }
}

impl Coordinates {
    fn reducer(self, instruction: &Instruction) -> Coordinates {
        match instruction {
            Instruction::Forward(value) => Coordinates {
                horizontal: self.horizontal + value,
                ..self
            },
            Instruction::Up(value) => Coordinates {
                depth: self.depth - value,
                ..self
            },
            Instruction::Down(value) => Coordinates {
                depth: self.depth + value,
                ..self
            },
        }
    }

    fn aim_reducer(self, instruction: &Instruction) -> Coordinates {
        match instruction {
            Instruction::Forward(value) => Coordinates {
                horizontal: self.horizontal + value,
                depth: self.depth + value * self.aim,
                ..self
            },
            Instruction::Up(value) => Coordinates {
                aim: self.aim - value,
                ..self
            },
            Instruction::Down(value) => Coordinates {
                aim: self.aim + value,
                ..self
            },
        }
    }
}

fn calculate_coordinates(instructions: &[Instruction]) -> Coordinates {
    instructions.iter().fold(
        Coordinates {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        Coordinates::reducer,
    )
}

fn aim_calculate_coordinates(instructions: &[Instruction]) -> Coordinates {
    instructions.iter().fold(
        Coordinates {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        Coordinates::aim_reducer,
    )
}

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(ParseInstructionError);
        }

        let value = parts[1].parse::<i32>().map_err(|_| ParseInstructionError)?;

        match parts[0] {
            "forward" => Ok(Instruction::Forward(value)),
            "up" => Ok(Instruction::Up(value)),
            "down" => Ok(Instruction::Down(value)),
            _ => Err(ParseInstructionError),
        }
    }
}

fn read_instructions_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Instruction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            line.parse::<Instruction>()
                .map_err(|_| eprintln!("Warning: Could not parse instruction '{line}'"))
                .ok()
        })
        .collect();

    Ok(instructions)
}

fn main() {
    let args = Args::parse();
    let instructions = match read_instructions_from_file(&args.filename) {
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
    println!("{:?} instructions parsed", instructions.len());
    println!("Coordinates: {}", calculate_coordinates(&instructions));
    println!(
        "Aim Coordinates: {}",
        aim_calculate_coordinates(&instructions)
    );
}
