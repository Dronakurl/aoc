#![allow(dead_code)]
use log::{debug, error, info};
use std::env;
use std::fs::File;
use std::io::prelude::*;

// Example input
// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX

pub fn read_file(file_name: &str) -> std::io::Result<String> {
    info!("Attempting to read file: {}", file_name);
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        error!("Failed to read file: {}", e);
        return Err(e);
    }
    info!("Successfully read file: {}", file_name);
    Ok(contents)
}

struct Matrix {
    rows: Vec<Vec<char>>,
}

impl Matrix {
    fn height(&self) -> usize {
        self.rows.len()
    }
    fn width(&self) -> usize {
        self.rows[0].len()
    }
    fn assert_valid(&self) -> bool {
        let width = self.width();
        for row in self.rows.iter() {
            if row.len() != width {
                return false;
            }
        }
        true
    }
    fn from_string(contents: &str) -> Matrix {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in contents.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            matrix.push(row);
        }
        let m = Matrix { rows: matrix };
        if !m.assert_valid() {
            panic!("Invalid matrix");
        }
        m
    }
    fn get_row(&self, row: usize) -> Vec<char> {
        self.rows[row].clone()
    }
    fn get_column(&self, col: usize) -> Vec<char> {
        let mut column: Vec<char> = Vec::new();
        for row in self.rows.iter() {
            column.push(row[col]);
        }
        column
    }
    fn columns(&self) -> Vec<Vec<char>> {
        let mut columns: Vec<Vec<char>> = Vec::new();
        for i in 0..self.width() {
            columns.push(self.get_column(i));
        }
        columns
    }
    fn diagonals(&self) -> Vec<Vec<char>> {
        let mut diagonals: Vec<Vec<char>> = Vec::new();
        for i in 0..self.width() {
            let mut diagonal: Vec<char> = Vec::new();
            let mut j = 0;
            let mut k = i;
            while j < self.height() && k < self.width() {
                diagonal.push(self.rows[j][k]);
                j += 1;
                k += 1;
            }
            diagonals.push(diagonal);
        }
        for i in 1..self.height() {
            let mut diagonal: Vec<char> = Vec::new();
            let mut j = i;
            let mut k = 0;
            while j < self.height() && k < self.width() {
                diagonal.push(self.rows[j][k]);
                j += 1;
                k += 1;
            }
            diagonals.push(diagonal);
        }

        // Top-right to bottom-left diagonals
        for i in 0..self.width() {
            let mut diagonal: Vec<char> = Vec::new();
            let mut j = 0;
            let mut k = i;
            while j < self.height() && k < self.width() {
                diagonal.push(self.rows[j][self.width() - 1 - k]);
                j += 1;
                k += 1;
            }
            diagonals.push(diagonal);
        }
        for i in 1..self.height() {
            let mut diagonal: Vec<char> = Vec::new();
            let mut j = i;
            let mut k = 0;
            while j < self.height() && k < self.width() {
                diagonal.push(self.rows[j][self.width() - 1 - k]);
                j += 1;
                k += 1;
            }
            diagonals.push(diagonal);
        }
        diagonals
    }
    // Count the number of times the string "XMAS" appears in the matrix
    fn count_xmas(&self) -> u64 {
        let mut count: u64 = 0;
        for row in self.rows.iter() {
            count += row.iter().collect::<String>().matches("XMAS").count() as u64;
            count += row.iter().collect::<String>().matches("SAMX").count() as u64;
        }
        for col in self.columns().iter() {
            count += col.iter().collect::<String>().matches("XMAS").count() as u64;
            count += col.iter().collect::<String>().matches("SAMX").count() as u64;
        }
        debug!("Diagonals: {:?}", self.diagonals());
        for col in self.diagonals().iter() {
            count += col.iter().collect::<String>().matches("XMAS").count() as u64;
            count += col.iter().collect::<String>().matches("SAMX").count() as u64;
        }
        count
    }

    // Get a slide of the matrix of the shape 3x3 at the specified row and column
    // If the row or column is at the edge of the matrix, return None
    fn get_3x3_slice(&self, row: usize, col: usize) -> Option<Vec<Vec<char>>> {
        let mut slice: Vec<Vec<char>> = Vec::new();
        for i in 0..3 {
            let mut row_slice: Vec<char> = Vec::new();
            for j in 0..3 {
                if row + i >= self.height() || col + j >= self.width() {
                    return None;
                }
                row_slice.push(self.rows[row + i][col + j]);
            }
            slice.push(row_slice);
        }
        Some(slice)
    }

    fn check_x_mas(&self, mat: &[Vec<char>]) -> bool {
        if mat[1][1] != 'A' {
            return false;
        }
        ((mat[0][0] == 'M' && mat[2][2] == 'S') || (mat[0][0] == 'S' && mat[2][2] == 'M'))
            && ((mat[2][0] == 'M' && mat[0][2] == 'S') || (mat[2][0] == 'S' && mat[0][2] == 'M'))
    }

    fn count_x_mas(&self) -> u32 {
        let mut count: u32 = 0;
        for i in 0..self.height() {
            for j in 0..self.width() {
                if let Some(slice) = self.get_3x3_slice(i, j) {
                    if self.check_x_mas(&slice) {
                        debug!("{:?}x{}", i, j);
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn display(&self, replace: Vec<char>) {
        for row in self.rows.iter() {
            for c in row.iter() {
                // if character is in replace, replace it by "."
                if replace.contains(c) {
                    print!(".");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Please provide a filename as a command line argument.");
        return;
    }
    let contents = match read_file(&args[1]) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Error reading file: {}", e);
            return;
        }
    };
    debug!("File contents: {}", contents);
    let m = Matrix::from_string(&contents);
    println!("XMAS appears {} times in the matrix", m.count_xmas());
    debug!("{:?}", m.get_3x3_slice(4, 4));
    m.display(vec!['X']);
    println!(
        "XMAS appears {} times in 3x3 slices of the matrix",
        m.count_x_mas()
    );
}
