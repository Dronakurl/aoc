#![allow(dead_code)]
use std::collections::VecDeque;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

struct Map {
    data: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Map: {}x{}", self.height, self.width)?;
        for row in &self.data {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn read(contents: &str) -> Self {
        let data: Vec<Vec<u8>> = contents
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
        let height = data.len();
        let width = {
            if data.is_empty() {
                0
            } else {
                data[0].len()
            }
        };
        Map {
            data,
            height,
            width,
        }
    }
    fn read_file(file_name: &str) -> Self {
        let contents = read_file(file_name).unwrap();
        Self::read(&contents)
    }
    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if col >= self.width || row >= self.height {
            return None;
        }
        Some(self.data[row][col])
    }
    fn get_all(&self, row: usize, col: usize) -> Option<(u8, (usize, usize))> {
        if col >= self.width || row >= self.height {
            return None;
        }
        Some((self.data[row][col], (row, col)))
    }

    fn get_direction(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<(u8, (usize, usize))> {
        match direction {
            Direction::North => {
                if row > 0 {
                    self.get_all(row - 1, col)
                } else {
                    None
                }
            }
            Direction::South => self.get_all(row + 1, col),
            Direction::East => self.get_all(row, col + 1),
            Direction::West => {
                if col > 0 {
                    self.get_all(row, col - 1)
                } else {
                    None
                }
            }
        }
    }

    fn get_valid_next(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if let Some(cur) = self.get(row, col) {
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                if let Some(new) = self.get_direction(row, col, direction) {
                    if new.0 == cur + 1 {
                        result.push(new.1);
                    }
                }
            }
        }
        log::trace!("row: {}, col: {}, valid_next: {:?}", row, col, result);
        result
    }
    fn get_trailheads(&self) -> Vec<(usize, usize)> {
        (0..self.height)
            .flat_map(|row| (0..self.width).map(move |col| (row, col)))
            .filter(|&(row, col)| self.data[row][col] == 0)
            .collect()
    }

    fn search_trails(&self, trailhead: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; self.width]; self.height]; // Assuming self.width and self.height are defined
        q.push_back(trailhead);
        visited[trailhead.0][trailhead.1] = true;

        while let Some(next) = q.pop_front() {
            log::trace!("next: {:?}", next);
            for n in self.get_valid_next(next.0, next.1) {
                if !visited[n.0][n.1] {
                    log::trace!("new: {:?}", n);
                    q.push_back(n);
                    visited[n.0][n.1] = true;
                }
            }
            if self.get(next.0, next.1) == Some(9) {
                result.push(next);
            }
        }
        result
    }
    fn search_all_trails(&self) -> Vec<usize> {
        use rayon::prelude::*;
        self.get_trailheads()
            .par_iter()
            .map(|&trailhead| self.search_trails(trailhead).len())
            .collect()
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn read_file(file_name: &str) -> std::io::Result<String> {
    log::info!("Attempting to read file: {}", file_name);
    let mut file = File::open(file_name).map_err(|e| {
        log::error!("Failed to open file: {}", e);
        e
    })?;
    log::info!("Successfully opened file: {}", file_name);
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
    env_logger::builder()
        .format_source_path(true)
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log::error!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }
    let map = Map::read_file(&args[1]);
    log::debug!("{}", map);
    let res = map.search_all_trails();
    log::info!("Result: {:?}", res.iter().sum::<usize>());

    log::info!("time elapsed: {:?}", start.elapsed());
}
