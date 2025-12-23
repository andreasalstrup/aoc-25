use std::fs::File;
use std::io::{Result, prelude::*};
use std::{isize, usize};

use Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    VerticalTopLeft,
    VerticalTopRight,
    VerticalBottomLeft,
    VerticalBottomRight,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
            VerticalTopLeft => (-1, -1),
            VerticalTopRight => (-1, 1),
            VerticalBottomLeft => (1, -1),
            VerticalBottomRight => (1, 1),
        }
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(data: &str) -> Grid {
        let data: Vec<Vec<u8>> = data
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect())
            .collect();
        let rows = data.len();
        let cols = data.first().map(|v| v.len()).unwrap_or(0);
        Grid { data, rows, cols }
    }

    pub fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .data
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&b'.')
    }

    pub fn adj_count(&self, row: usize, col: usize) -> usize {
        let mut adj_count = 0;

        for direction in [
            Up,
            Down,
            Left,
            Right,
            VerticalTopLeft,
            VerticalTopRight,
            VerticalBottomLeft,
            VerticalBottomRight,
        ] {
            let (offset_row, offset_col) = direction.offset();
            let adj_row = row as isize + offset_row;
            let adj_col = col as isize + offset_col;

            if adj_row < 0 || adj_col < 0 {
                continue;
            }

            if self.get(adj_row, adj_col) == b'@' {
                adj_count += 1;
            }
        }

        if adj_count < 4 { 1 } else { 0 }
    }

    pub fn adj_count_repeating(&mut self, row: usize, col: usize) -> Option<(usize, usize)> {
        let mut adj_count = 0;

        for direction in [
            Up,
            Down,
            Left,
            Right,
            VerticalTopLeft,
            VerticalTopRight,
            VerticalBottomLeft,
            VerticalBottomRight,
        ] {
            let (offset_row, offset_col) = direction.offset();
            let adj_row = row as isize + offset_row;
            let adj_col = col as isize + offset_col;

            if adj_row < 0 || adj_col < 0 {
                continue;
            }

            if self.get(adj_row, adj_col) == b'@' {
                adj_count += 1;
            }
        }

        if adj_count < 4 {
            Some((row, col))
        } else {
            None
        }
    }
}

fn part_one(grid: &Grid) -> usize {
    (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.data[row][col] == b'@')
        .fold(0, |acc, (row, col)| acc + grid.adj_count(row, col))
}

fn part_two(grid: &mut Grid) -> usize {
    let pos: Vec<(usize, usize)> = (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.data[row][col] == b'@')
        .collect();

    let marked: Vec<(usize, usize)> = pos
        .iter()
        .filter_map(|&(r, c)| grid.adj_count_repeating(r, c))
        .collect();

    let mut acc = marked.len();

    for (row, col) in &marked {
        grid.data[*row][*col] = b'.';
    }

    if !marked.is_empty() {
        acc += part_two(grid);
    }

    acc
}

fn main() -> Result<()> {
    let mut file = File::open("./day-4/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let test = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let grid = &mut Grid::new(&buffer);
    // let grid = &mut Grid::new(test);

    println!("Result part one: {}", part_one(grid));
    println!("Result part two: {}", part_two(grid));

    Ok(())
}
