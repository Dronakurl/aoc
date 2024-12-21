#![allow(dead_code)]
use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...

#[derive(PartialEq, Clone)]
struct Cursor {
    cursor: [u16; 2],
    direction: Direction,
}

#[derive(PartialEq, Clone)]
struct State {
    field: Vec<Vec<bool>>,
    cursor: Cursor,
    path: Vec<Cursor>,
    walk_counter: u32,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Position {
    Blocked,
    OutOfBounds,
    Cursor([u16; 2]),
}

#[derive(Debug)]
enum WalkResult {
    Loop,
    HitWall,
}

impl State {
    fn read_file(file_name: &str) -> std::io::Result<State> {
        let contents = read_file(file_name)?;
        let mut field = Vec::new();
        let mut cursor: Cursor = Cursor {
            cursor: [0, 0],
            direction: Direction::Up,
        };
        for (nrow, line) in contents.lines().enumerate() {
            let mut row = Vec::new();
            for (col, c) in line.chars().enumerate() {
                row.push(c == '#');
                if c == '^' {
                    cursor.cursor = [nrow as u16, col as u16];
                }
            }
            field.push(row);
        }
        Ok(State {
            field,
            cursor: cursor.clone(),
            path: vec![cursor],
            walk_counter: 0,
        })
    }

    #[inline]
    fn turn_right(&mut self) {
        match self.cursor.direction {
            Direction::Up => self.cursor.direction = Direction::Right,
            Direction::Down => self.cursor.direction = Direction::Left,
            Direction::Left => self.cursor.direction = Direction::Up,
            Direction::Right => self.cursor.direction = Direction::Down,
        }
    }

    fn get_position(&self, cursor: &[u16; 2]) -> Position {
        if cursor[0] >= self.field.len() as u16 || cursor[1] >= self.field[0].len() as u16 {
            Position::OutOfBounds
        } else if self.field[cursor[0] as usize][cursor[1] as usize] {
            Position::Blocked
        } else {
            Position::Cursor(*cursor)
        }
    }

    fn get_next(&self) -> Position {
        let mut next = self.cursor.clone();
        match self.cursor.direction {
            Direction::Up => {
                if next.cursor[0] == 0 {
                    return Position::OutOfBounds;
                };
                next.cursor[0] -= 1
            }
            Direction::Down => next.cursor[0] += 1,
            Direction::Left => {
                if next.cursor[1] == 0 {
                    return Position::OutOfBounds;
                };
                next.cursor[1] -= 1
            }
            Direction::Right => next.cursor[1] += 1,
        }
        self.get_position(&next.cursor)
    }

    fn walk_one(&mut self) -> Position {
        let pos = self.get_next();
        match pos {
            Position::Blocked => {
                self.turn_right();
            }
            Position::OutOfBounds => {
                self.walk_counter += 1;
                self.path.push(self.cursor.clone());
            }
            Position::Cursor(cur) => {
                self.path.push(self.cursor.clone());
                self.walk_counter += 1;
                self.cursor.cursor = cur;
            }
        }
        pos
    }

    fn at_start(&self) -> bool {
        self.cursor == self.path[0] && self.cursor.direction == Direction::Up
    }

    fn remove_row_lines(&self) {
        let rows = self.field.len();
        // remove so many lines from the terminal output (move up)
        print!("\x1b[{}A", rows + 1);
    }

    fn print_empty_lines(&self) {
        let rows = self.field.len();
        for _ in 0..rows {
            println!();
        }
    }

    fn count_unique(&self) -> usize {
        self.path
            .iter()
            .map(|x| x.cursor)
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    #[inline]
    fn is_walked(&self, cursor: &[u16; 2]) -> bool {
        self.path.iter().any(|x| x.cursor == *cursor)
    }

    #[inline]
    fn is_in_loop(&self) -> bool {
        self.path.contains(&self.cursor)
    }

    fn walk(&mut self, display: bool) -> WalkResult {
        // get log_level
        // let logit: bool = matches!(
        //     env::var("RUST_LOG").unwrap_or("info".to_string()).as_str(),
        //     "debug" | "trace"
        // );
        // if logit {
        //     log::info!("Starting walk");
        //     self.print_empty_lines();
        // }
        loop {
            let next = self.walk_one();
            if next == Position::OutOfBounds {
                return WalkResult::HitWall;
            }
            if self.is_in_loop() {
                return WalkResult::Loop;
            }
            if display {
                self.remove_row_lines();
                // wait for half a second
                std::thread::sleep(std::time::Duration::from_millis(100));
                println!("{}", self);
            }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (nrow, row) in self.field.iter().enumerate() {
            for (ncol, cell) in row.iter().enumerate() {
                if self.cursor.cursor == [nrow as u16, ncol as u16] {
                    write!(f, "👠")?;
                } else if self.is_walked(&[nrow as u16, ncol as u16]) {
                    write!(f, "💣")?;
                } else if *cell {
                    write!(f, "🪨")?;
                } else {
                    write!(f, "..")?;
                }
            }
            if nrow == self.field.len() - 1 {
                write!(f, " {} {}", self.walk_counter, self.count_unique())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn read_file(file_name: &str) -> std::io::Result<String> {
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

fn check_for_loops(s: State) -> u16 {
    use rayon::prelude::*;
    let mut work = s.clone();
    work.walk(false);
    let mut options: Vec<[u16; 2]> = work
        .path
        .iter()
        .map(|x| x.cursor)
        .filter(|x| !s.field[x[0] as usize][x[1] as usize])
        .collect();
    options = options[1..].to_vec();
    let unique_options: HashSet<_> = options.drain(..).collect();
    let options: Vec<_> = unique_options.into_iter().collect();
    log::debug!("{:?}", options);

    let pb = indicatif::ProgressBar::new(options.len() as u64);
    let count = options
        .par_iter()
        .map(|option| {
            let mut work = s.clone();
            work.field[option[0] as usize][option[1] as usize] = true;
            let res = work.walk(false);
            log::trace!("{:?}", res);
            pb.inc(1);
            match res {
                WalkResult::Loop => 1,
                WalkResult::HitWall => 0,
            }
        })
        .sum();

    pb.finish_with_message("done");
    count
}

fn main() {
    let start = Instant::now();
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log::error!("Please provide a filename as a command line argument.");
        return;
    }
    let s = State::read_file(&args[1]).unwrap();
    // log::info!("state: {}", s);

    let mut work = s.clone();
    work.walk(false);
    println!("Part 1: {}", work.count_unique());

    println!("Part 2: {}", check_for_loops(s));

    log::info!("time elapsed: {:?}", start.elapsed());
}
