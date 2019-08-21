#![allow(dead_code)]

use rand::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug)]
struct Cell {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn new() -> Self {
        Maze {
            grid: vec![vec![Cell::new(); 10]; 10],
        }
    }

    fn with_dimensions(row: usize, col: usize) -> Self {
        Maze {
            grid: vec![vec![Cell::new(); row]; col],
        }
    }

    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col].up = rng.next_u32() % 2 == 0;
                self.grid[row][col].down = rng.next_u32() % 2 == 0;
                self.grid[row][col].left = rng.next_u32() % 2 == 0;
                self.grid[row][col].right = rng.next_u32() % 2 == 0;
            }
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x_idx, row) in self.grid.iter().enumerate() {
            write!(f, "+")?;
            for cell in row.iter() {
                if cell.up || x_idx == 0 {
                    write!(f, " -- ")?;
                } else {
                    write!(f, "    ")?;
                }

                if x_idx < self.grid.len() {
                    write!(f, "+")?;
                }
            }

            write!(f, "\n")?;
            for (idx, cell) in row.iter().enumerate() {
                if cell.left || idx == 0 {
                    write!(f, "|    ")?;
                } else {
                    write!(f, "     ")?;
                }

                if idx + 1 == row.len() {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
        }
        for _ in 0..self.grid[0].len() {
            write!(f, "+ -- ",)?;
        }
        write!(f, "+")
    }
}

fn main() {
    let mut maze = Maze::with_dimensions(2, 2);
    maze.randomize();
    println!("{}", maze);
}
