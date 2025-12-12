use std::fmt;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

// #[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Vec<char>>,
    pointer: (usize, usize),
    beam_encounters: Vec<usize>,
    beam_count: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid {{")?;
        writeln!(f, "  rows: {}", self.rows)?;
        writeln!(f, "  cols: {}", self.cols)?;
        writeln!(f, "  beam_count: {}", self.beam_count)?;
        writeln!(f, "  pointer: {:?}", self.pointer)?;
        writeln!(f, "  beam_encounters: {:?}", self.beam_encounters)?;
        writeln!(f, "  data:")?;

        for row in &self.data {
            let line: String = row.iter().collect();
            writeln!(f, "    {}", line)?;
        }

        writeln!(f, "}}")
    }
}

impl Grid {
    fn find_start(&self) -> (usize, usize) {
        let (idx, _) = self.data[0]
            .iter()
            .enumerate()
            .find(|(_, ch)| **ch == 'S')
            .unwrap();
        (0, idx)
    }

    fn next_down_iter(
        &self,
        (curr_row, curr_col): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        (curr_row..self.rows).map(move |row| (row, curr_col))
    }

    fn get_in_row(&mut self, (curr_row, _curr_col): (usize, usize)) -> Vec<(usize, usize)> {
        self.data[curr_row]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '^')
            .map(|(col, _)| (curr_row, col))
            .collect()
    }

    fn set_bream(&mut self, curr_pos: (usize, usize)) {
        let rows: Vec<(usize, usize)> = self.next_down_iter(curr_pos).collect();

        for (r, c) in rows {
            if let Some((a, b)) = (self.data[r][c] == '.').then(|| (r, c)) {
                self.data[a][b] = '|';

                if a < self.rows - 1 {
                    if self.data[a + 1][b] == '^' {
                        self.beam_count += 1;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn transmit_bream(&mut self, pos: Vec<(usize, usize)>) {
        for (r, c) in pos {
            if let Some((a, b)) = (self.data[r][c - 1] == '.').then(|| (r, c - 1)) {
                self.set_bream((a, b));
            }
            if let Some((a, b)) = (self.data[r][c + 1] == '.').then(|| (r, c + 1)) {
                self.set_bream((a, b));
            }
        }
    }

    fn set_timelines(&mut self, pos: Vec<(usize, usize)>) {
        for (_, c) in pos {
            let timelines = &mut self.beam_encounters;
            timelines[c - 1] += timelines[c];
            timelines[c + 1] += timelines[c];
            timelines[c] = 0;
        }
    }
}

fn parse_input(buffer: &str) -> Grid {
    let mut rows = Vec::new();
    for line in buffer.lines() {
        let row: Vec<char> = line.chars().collect();
        rows.push(row);
    }

    let cols = rows[0].len();
    Grid {
        rows: rows.len(),
        cols: cols,
        data: rows,
        pointer: (0, 0),
        beam_encounters: vec![0; cols],
        beam_count: 0,
    }
}

fn part_one(grid: &mut Grid) -> usize {
    let (r, c) = grid.find_start();
    grid.data[r + 1][c] = '|';
    grid.beam_count += 1;

    let rows: Vec<(usize, usize)> = grid.next_down_iter((r, c)).collect();

    for pos in rows {
        let idx = grid.get_in_row(pos);
        grid.transmit_bream(idx);
    }

    grid.beam_count
}

fn part_two(grid: &mut Grid) -> usize {
    let (r, c) = grid.find_start();
    grid.beam_encounters[c] += 1;

    let rows: Vec<(usize, usize)> = grid.next_down_iter((r, c)).collect();
    for pos in rows {
        let idx = grid.get_in_row(pos);
        grid.set_timelines(idx);
    }

    grid.beam_count = grid.beam_encounters.iter().sum();
    grid.beam_count
}

fn main() -> Result<()> {
    let mut file = File::open("./day-7/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut grid = parse_input(&buffer);
    let result_1 = part_one(&mut grid);
    let result_2 = part_two(&mut grid);

    println!("Result part 1: {}", result_1);
    println!("Result part 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
    }

    #[test]
    fn test_part_one() {
        let mut grid = parse_input(input());
        let result = part_one(&mut grid);
        assert_eq!(21, result);
    }

    #[test]
    fn test_part_two() {
        let mut grid = parse_input(input());
        let result = part_two(&mut grid);
        assert_eq!(40, result);
    }
}
