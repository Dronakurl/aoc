#![allow(dead_code)]
use crate::printgrid::{printgrid, GridElement};
use crate::read_file::read_file;
use array2d::Array2D;
use log::*;
use std::env;
use std::time::Instant;

mod printgrid;
mod read_file;

#[derive(Clone, Debug, PartialEq)]
struct Beam {
    cur: (usize, usize),
    dir: (i8, i8),
}

impl Beam {
    fn new(row: usize, col: usize, dir: (i8, i8)) -> Beam {
        Beam {
            cur: (row, col),
            dir,
        }
    }
    fn is_in_hist(hist: &Vec<Beam>, row: usize, col: usize, dir: (i8, i8)) -> bool {
        hist.contains(&Beam {
            cur: (row, col),
            dir,
        })
    }
    fn walk(&mut self, grid: &mut Array2D<GridElement>, hist: &mut Vec<Beam>) {
        debug!("dir={:?} cur={:?}", self.dir, self.cur);
        let (row, col) = (self.cur.0 as usize, self.cur.1 as usize);
        let ce: &mut GridElement;
        match grid.get_mut(row, col) {
            None => {
                return;
            }
            Some(fe) => {
                ce = fe;
            }
        }

        ce.energized = true;
        match ce.typ {
            '/' => {
                self.dir = match self.dir {
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    _ => self.dir,
                }
            }
            '\\' => {
                self.dir = match self.dir {
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    _ => self.dir,
                }
            }
            '|' => {
                self.dir = match self.dir {
                    (0, 1) | (0, -1) => {
                        let mut b = Beam {
                            cur: (ce.row, ce.col),
                            dir: (-1, 0),
                        };
                        // debug!("| cur={:?} dir={:?} {:?} hit me ", self.cur, self.dir, &b);
                        b.walk(grid, hist);
                        (1, 0)
                    }
                    _ => self.dir,
                }
            }
            '-' => {
                self.dir = match self.dir {
                    (1, 0) | (-1, 0) => {
                        let mut b = Beam {
                            cur: (ce.row, ce.col),
                            dir: (0, 1),
                        };
                        // debug!("- cur={:?} dir={:?} {:?} hit me ", self.cur, self.dir, &b);
                        b.walk(grid, hist);
                        (0, -1)
                    }
                    _ => self.dir,
                }
            }
            _ => self.dir = self.dir,
        }

        if let Some(nextgridele) = match self.dir {
            (-1, 0) if row >= 1 => grid.get(row - 1, col),
            (1, 0) => grid.get(row + 1, col),
            (0, -1) if col >= 1 => grid.get(row, col - 1),
            (0, 1) => grid.get(row, col + 1),
            _ => None,
        } {
            self.cur = (nextgridele.row, nextgridele.col);
        }

        if Beam::is_in_hist(hist, self.cur.0, self.cur.1, self.dir) {
            // debug!("{:?}", self);
            // debug!("{:?}", hist);
            return;
        }
        hist.push(self.clone());
        self.walk(grid, hist);
    }
}

fn count_energized(grid: &Array2D<GridElement>) -> i32 {
    let mut total: i32 = 0;
    for ge in grid.elements_row_major_iter() {
        if ge.energized {
            total += 1;
        }
    }
    total
}
fn reset_energized(grid: &mut Array2D<GridElement>) {
    for (x, y) in grid.indices_row_major() {
        if let Some(element) = grid.get_mut(x, y) {
            if element.energized {
                element.energized = false;
            }
        }
    }
}

fn main() {
    env_logger::init();
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
    let lines: Vec<&str> = inputtxt.lines().collect();
    let hei = lines.len();
    let wid = lines.first().map_or(0, |line| line.len());
    let mut grid: Array2D<GridElement> =
        Array2D::filled_with(GridElement::new(0, 0, '.'), hei, wid);

    for (row, line) in inputtxt.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if let Some(ge) = grid.get_mut(row, col) {
                ge.row = row;
                ge.col = col;
                ge.typ = c;
            }
        }
    }

    let mut b = Beam {
        cur: (0, 0),
        dir: (0, 1),
    };
    let mut hist: Vec<Beam> = Vec::new();
    b.walk(&mut grid, &mut hist);

    printgrid(&grid);
    println!("part 1: {}", count_energized(&grid));
    println!("time elapsed: {:?}", start.elapsed());

    let mut edges: Vec<Beam> = Vec::new();

    for col in 0..wid {
        edges.push(Beam {
            cur: (0, col),
            dir: (1, 0),
        });
        edges.push(Beam {
            cur: (hei - 1, col),
            dir: (-1, 0),
        });
    }
    edges.push(Beam {
        cur: (0, 0),
        dir: (0, 1),
    });
    edges.push(Beam {
        cur: (hei - 1, 0),
        dir: (0, 1),
    });
    edges.push(Beam {
        cur: (0, wid - 1),
        dir: (0, -1),
    });
    edges.push(Beam {
        cur: (hei - 1, wid - 1),
        dir: (0, -1),
    });
    for row in 0..hei {
        edges.push(Beam {
            cur: (row, 0),
            dir: (0, 1),
        });
        edges.push(Beam {
            cur: (row, wid - 1),
            dir: (0, -1),
        });
    }
    let mut max = 0_i32;
    for edge in edges.iter_mut() {
        reset_energized(&mut grid);
        let mut hist: Vec<Beam> = Vec::new();
        edge.walk(&mut grid, &mut hist);
        let cnt = count_energized(&grid);
        info!("{:?} {}", edge, cnt);
        if cnt > max {
            max = cnt;
        }
    }
    println!("part 2 = {}", max);
    println!("time elapsed: {:?}", start.elapsed());
}

#[cfg(test)]
mod test;
