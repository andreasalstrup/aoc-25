use std::collections::HashSet;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(min: usize, max: usize) -> Range {
        Range { min, max }
    }

    pub fn in_range(&self, num: usize) -> bool {
        self.min <= num && num <= self.max
    }

    pub fn get_ids(&self) -> Vec<usize> {
        (self.min..self.max + 1).map(|n| n).collect()
    }
}

#[derive(Debug)]
struct DataBase {
    pub ranges: Vec<Range>,
    pub ids: Vec<usize>,
}

fn parse_input(buffer: &str) -> DataBase {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut range_mode = true;

    for line in buffer.lines() {
        if line.is_empty() {
            range_mode = false;
            continue;
        }

        if range_mode {
            let (min, max) = line.split_once('-').unwrap();
            ranges.push(Range::new(
                min.parse::<usize>().unwrap(),
                max.parse::<usize>().unwrap(),
            ));
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    DataBase { ranges, ids }
}

fn part_one(db: &DataBase) -> i64 {
    let mut count = 0;

    for id in &db.ids {
        let valid_id = &db.ranges.iter().any(|r| r.in_range(*id));
        if *valid_id {
            count += 1;
        }
    }

    count
}

fn part_two(db: &DataBase) -> usize {
    let mut ids_set: HashSet<usize> = HashSet::new();

    for r in &db.ranges {
        r.get_ids().iter().for_each(|id| {
            ids_set.insert(*id);
        });

        println!("size: {:?}", ids_set.len());
        break;
    }

    ids_set.len()
}

fn main() -> Result<()> {
    let mut file = File::open("./day-5/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let database = parse_input(&buffer);
    let result_1 = part_one(&database);
    let result_2 = part_two(&database);

    println!("Result part 1: {}", result_1);
    println!("Result part 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
    }

    #[test]
    fn test_part_one() {
        let database = parse_input(input());
        let result = part_one(&database);
        assert_eq!(3, result);
    }

    #[test]
    fn test_part_two() {
        let database = parse_input(input());
        let result = part_two(&database);
        assert_eq!(14, result);
    }
}
