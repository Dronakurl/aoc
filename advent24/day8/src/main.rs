#![allow(dead_code)]
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

// ............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............

#[derive(Debug)]
struct Node {
    row: usize,
    col: usize,
    frequency: char,
    antinode: bool,
}

struct Grid {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Grid {
    /// Read a grid from a file
    fn read_from_file(filename: &str) -> Self {
        let contents = read_file(filename).unwrap();
        let mut nodes = Vec::new();
        let mut width = 0;
        for (nrow, line) in contents.lines().enumerate() {
            let mut row = Vec::new();
            width = line.len();
            for (col, c) in line.chars().enumerate() {
                row.push(Node {
                    row: nrow,
                    col,
                    frequency: c,
                    antinode: false,
                });
            }
            nodes.push(row);
        }
        let height = nodes.len();
        Grid {
            nodes,
            width,
            height,
        }
    }

    fn get_node(&self, row: usize, col: usize) -> Option<&Node> {
        if row < self.height && col < self.width {
            Some(&self.nodes[row][col])
        } else {
            None
        }
    }

    fn get_node_mut(&mut self, row: usize, col: usize) -> Option<&mut Node> {
        if row < self.height && col < self.width {
            Some(&mut self.nodes[row][col])
        } else {
            None
        }
    }

    fn get_nodes_for_freq(&self, freq: char) -> Vec<&Node> {
        let mut nodes = Vec::new();
        for row in &self.nodes {
            for node in row {
                if node.frequency == freq {
                    nodes.push(node);
                }
            }
        }
        nodes
    }

    fn calculate_antinodes(&mut self, freq: char) {
        use itertools::Itertools;
        let antinode_coords: HashSet<[usize; 2]> = {
            self.get_nodes_for_freq(freq)
                .iter()
                .combinations(2)
                .flat_map(|pair| get_antinodes(pair).into_iter())
                .flatten()
                .collect()
        };
        log::trace!("antinode_coords {:?}", antinode_coords);

        for c in antinode_coords {
            let node = self.get_node_mut(c[0], c[1]);
            if let Some(c) = node {
                log::trace!("setting antinode at {}, {}", c.row, c.col);
                c.antinode = true;
            }
        }
    }

    fn calculate_antinodes_n(&mut self, freq: char) {
        use itertools::Itertools;
        let antinode_coords: HashSet<[usize; 2]> = {
            self.get_nodes_for_freq(freq)
                .iter()
                .combinations(2)
                .flat_map(|pair| get_antinode_all(pair, self.height, self.width).into_iter())
                .collect()
        };
        log::trace!("antinode_coords {:?}", antinode_coords);

        for c in antinode_coords {
            let node = self.get_node_mut(c[0], c[1]);
            if let Some(c) = node {
                log::trace!("setting antinode at {}, {}", c.row, c.col);
                c.antinode = true;
            }
        }
    }

    fn calc_all_antinodes_n(&mut self) {
        let freqs = self.get_all_freqs();
        for freq in freqs {
            self.calculate_antinodes_n(freq);
        }
    }

    fn calc_all_antinodes(&mut self) {
        let freqs = self.get_all_freqs();
        for freq in freqs {
            self.calculate_antinodes(freq);
        }
    }

    fn get_all_freqs(&self) -> HashSet<char> {
        self.nodes
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|node| {
                if node.frequency != '.' {
                    Some(node.frequency)
                } else {
                    None
                }
            })
            .collect()
    }

    fn count_antinodes(&self) -> usize {
        self.nodes
            .iter()
            .flat_map(|row| row.iter())
            .filter(|node| node.antinode)
            .count()
    }
}

fn get_antinodes(pair: Vec<&&Node>) -> [Option<[usize; 2]>; 2] {
    let first = pair[0];
    let second = pair[1];
    let coldiff = second.col as i32 - first.col as i32;
    let rowdiff = second.row as i32 - first.row as i32;

    let new_first = {
        let row = (first.row as i32 - rowdiff) as isize;
        let col = (first.col as i32 - coldiff) as isize;
        if row < 0 || col < 0 {
            None
        } else {
            Some([row as usize, col as usize])
        }
    };
    let new_second = {
        let row = (first.row as i32 + 2 * rowdiff) as isize;
        let col = (first.col as i32 + 2 * coldiff) as isize;
        if row < 0 || col < 0 {
            None
        } else {
            Some([row as usize, col as usize])
        }
    };

    [new_first, new_second]
}

fn get_antinode_n(pair: &[&&Node], n: i32, height: usize, width: usize) -> Option<[usize; 2]> {
    let first = pair[0];
    let second = pair[1];
    let coldiff = second.col as i32 - first.col as i32;
    let rowdiff = second.row as i32 - first.row as i32;

    let row = (first.row as i32 + n * rowdiff) as isize;
    let col = (first.col as i32 + n * coldiff) as isize;
    if row < 0 || col < 0 || row > height as isize || col > width as isize {
        None
    } else {
        Some([row as usize, col as usize])
    }
}

fn get_antinode_all(pair: Vec<&&Node>, height: usize, width: usize) -> Vec<[usize; 2]> {
    log::trace!("get_antinode_all: {:?}", pair);
    let mut n = 0;
    let mut results: Vec<[usize; 2]> = Vec::new();
    loop {
        let mut found_one = false;
        if let Some(coords) = get_antinode_n(&pair, n, height, width) {
            results.push(coords);
            found_one = true;
        }
        if let Some(coords) = get_antinode_n(&pair, -n, height, width) {
            results.push(coords);
            found_one = true;
        }
        n += 1;
        log::trace!("results: {:?} n {:?}", results, n);
        if !found_one {
            break;
        }
    }
    results
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Grid: {}x{}", self.height, self.width)?;
        for row in &self.nodes {
            for node in row {
                if node.frequency == '.' && node.antinode {
                    write!(f, "\x1b[31m#\x1b[0m")?;
                } else if node.antinode {
                    write!(f, "\x1b[31m{}\x1b[0m", node.frequency)?;
                } else {
                    write!(f, "{}", node.frequency)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
    if args.len() != 2 {
        log::error!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mut g = Grid::read_from_file(&args[1]);
    log::debug!("{}", g);
    // g.calc_all_antinodes();
    g.calc_all_antinodes_n();
    log::debug!("{}", g);
    log::info!("Antinodes: {}", g.count_antinodes());

    log::info!("time elapsed: {:?}", start.elapsed());
}
