use array2d::Array2D;
use log::{log_enabled, Level};
/// typ="/", ".", "-", "|", "\"
#[derive(Clone)]
pub struct GridElement {
    pub col: usize,
    pub row: usize,
    pub typ: char,
    pub energized: bool,
}
impl GridElement {
    pub fn new(col: usize, row: usize, typ: char) -> GridElement {
        GridElement {
            col,
            row,
            typ,
            energized: false,
        }
    }
}
pub fn printgrid(grid: &Array2D<GridElement>) {
    if !log_enabled!(Level::Debug) {
        return;
    }
    print!("  ");
    for i in 0..grid.column_len() {
        print!("{}", i);
    }
    println!();
    for (n, row) in grid.rows_iter().enumerate() {
        print!("{} ", n);
        for element in row {
            print!("{}", element.typ);
        }
        println!();
    }
    println!("---------------------------------");
    print!("  ");
    for i in 0..grid.column_len() {
        print!("{}", i);
    }
    println!();
    for (n, row) in grid.rows_iter().enumerate() {
        print!("{} ", n);
        for element in row {
            if element.energized {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
