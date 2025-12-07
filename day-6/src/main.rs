use std::collections::HashSet;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

#[derive(Debug)]
struct Grid<'a> {
    rows: usize,
    col: usize,
    data: Vec<Vec<&'a str>>,
    operation: Vec<&'a str>,
    operation2: Vec<Vec<String>>,
}

impl Grid<'_> {
    fn get_problems(&self) -> Vec<Problem> {
        let mut problems = Vec::new();
        let mut numbers_str = vec![Vec::new(); self.col];

        for row in self.data.clone() {
            let mut idx_col = 0;
            for value in row.clone() {
                numbers_str[idx_col].push(value);
                idx_col += 1;
            }
        }

        let numbers = numbers_str
            .iter()
            .map(|n| {
                n.iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        for (idx, number) in numbers.iter().enumerate() {
            problems.push(Problem {
                numbers: number.to_vec(),
                operator: self.operation[idx].to_string(),
            });
        }

        problems
    }

    fn get_problems2(&self) -> Vec<Problem2> {
        let mut problems = Vec::new();

        let start = self.col;

        let mut numbers = Vec::new();
        for (idx, i) in (0..start).rev().enumerate() {
            numbers.push(Vec::new());
            for d in self.data.clone() {
                numbers[idx].push(d[i]);
            }
        }

        for (idx, n) in numbers.iter().enumerate() {
            let rev_idx = numbers.len() - 1 - idx;

            problems.push(Problem2 {
                numbers: n.to_vec(),
                operator: self.operation[rev_idx],
            });
        }

        problems
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: String,
}

#[derive(Debug, Clone)]
struct Problem2<'a> {
    numbers: Vec<&'a str>,
    operator: &'a str,
}

fn parse_input(buffer: &str) -> Grid {
    let mut rows = Vec::new();
    for line in buffer.lines() {
        let row = line.split_whitespace().collect::<Vec<&str>>();
        rows.push(row.iter().map(|r| r.trim()).collect::<Vec<_>>());
    }

    let operator = buffer.lines().last().unwrap().to_string();

    let mut result: Vec<Vec<String>> = Vec::new();
    let mut idx: usize = 0;
    for ch in operator.chars() {
        if !ch.is_whitespace() {
            result.push(Vec::new());
            result[idx].push(ch.to_string());
            idx += 1;
        } else {
            if idx == 0 {
                result.push(Vec::new());
                idx += 1;
            }
            result[idx - 1].push(ch.to_string());
        }
    }

    let result: Vec<Vec<String>> = result.into_iter().map(|row| vec![row.concat()]).collect();
    Grid {
        rows: rows.len(),
        col: rows[0].len(),
        data: rows[0..rows.len() - 1].to_vec(),
        operation: rows.last().unwrap().to_vec(),
        operation2: result,
    }
}

fn parse_input_2(buffer: &str) -> Grid {
    let mut indices = Vec::new();

    for line in buffer.lines() {
        let idx: HashSet<usize> = line
            .char_indices()
            .filter(|(_, ch)| ch.is_whitespace())
            .map(|(i, _)| i)
            .collect();
        indices.push(idx);
    }

    let mut iter = indices.into_iter();
    let first = iter.next().unwrap();
    let inter = iter.fold(first, |acc, set| acc.intersection(&set).cloned().collect());

    let mut inter_vec: Vec<usize> = inter.iter().cloned().collect();
    inter_vec.sort();

    let mut result = Vec::new();

    for (idx, line) in buffer.lines().enumerate() {
        result.push(Vec::new());

        let mut start = 0;
        for i in inter_vec.iter() {
            let end = *i;
            let split = &line[start..end];
            result[idx].push(split);

            start = end + 1;
        }

        let split = &line[start..line.len()];
        result[idx].push(split);
    }

    Grid {
        rows: result.len() - 1,
        col: result[0].len(),
        data: result[0..result.len() - 1].to_vec(),
        operation: result[result.len() - 1].clone(),
        operation2: vec![],
    }
}

fn part_one(problems: &Vec<Problem>) -> i64 {
    let mut result = Vec::new();

    for p in problems {
        match p.operator.as_str() {
            "*" => {
                let sum = p.numbers.iter().fold(1, |acc, n| acc * n);
                result.push(sum);
            }
            "+" => {
                let sum = p.numbers.iter().fold(0, |acc, n| acc + n);
                result.push(sum);
            }
            _ => {}
        }
    }

    result.iter().fold(0, |acc, r| acc + r)
}

fn part_two(problems: &Vec<Problem2>) -> i64 {
    let mut result = 0;

    let mut result_new = Vec::new();
    for (_idx, p) in problems.iter().enumerate() {
        let mut new_num = Vec::new();
        for i in (0..p.operator.len()).rev() {
            for (_idx, n) in p.numbers.clone().iter().enumerate() {
                let num = n.chars().nth(i);
                new_num.push(num.unwrap());
            }
            new_num.push(p.operator.chars().nth(0).unwrap());
        }

        result_new.push(new_num);
    }

    for vec in result_new {
        let mut s: String = vec.iter().filter(|c| **c != ' ').collect();

        let op = s.pop().unwrap();
        let res: i64 = match op {
            '+' => s.split(op).map(|c| c.parse::<i64>().unwrap()).sum(),
            '*' => s.split(op).map(|c| c.parse::<i64>().unwrap()).product(),
            _ => 0,
        };

        result += res;
    }

    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-6/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let grid = parse_input(&buffer);
    let problems = grid.get_problems();

    let grid2 = parse_input_2(&buffer);
    let problems2 = grid2.get_problems2();

    let result_1 = part_one(&problems);
    let result_2 = part_two(&problems2);

    println!("Result part 1: {}", result_1);
    println!("Result part 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
    }

    #[test]
    fn test_part_one() {
        let grid = parse_input(input());
        // println!("{:?}", grid);
        let problems = grid.get_problems();
        // println!("{:?}", problems);
        let result = part_one(&problems);
        assert_eq!(4277556, result);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input_2(input());
        let problems = grid.get_problems2();
        let result = part_two(&problems);
        assert_eq!(3263827, result);
    }
}
