use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

#[derive(Debug, Default)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn parse(mut str: &str) -> Range {
        str = str.trim();
        let values: Vec<_> = str.split("-").collect();
        let min = values.get(0).unwrap().parse::<u64>().unwrap();
        let max = values.get(1).unwrap().parse::<u64>().unwrap();

        Range { min, max }
    }

    fn get_ids(&self) -> Vec<u64> {
        if self.min == 0 && self.max == 0 {
            return vec![];
        }

        let mut result: Vec<u64> = vec![];
        for i in self.min..self.max + 1 {
            result.push(i);
        }

        result
    }

    fn is_invalid_part_1(id: &u64) -> bool {
        let mut first = id.to_string();

        if first.len() % 2 != 0 {
            return false;
        }

        let last = first.split_off(first.len() / 2);
        first == last
    }

    fn is_invalid_part_2(id: &u64) -> bool {
        let str = id.to_string();

        for (idx, _c) in str.char_indices() {
            let new = str.get(idx..str.len()).unwrap();

            let mul = str.len() / new.len();
            let mul_str = new.repeat(mul);

            let res = if mul_str == str { true } else { false };

            if res && idx > 0 {
                return true;
            }
        }

        false
    }
}

fn parse_input(buffer: &str) -> Vec<u64> {
    let mut ranges = Vec::new();
    for part in buffer.split(",") {
        ranges.push(Range::parse(part));
    }

    let mut ids = Vec::new();
    for range in ranges {
        ids.push(range.get_ids());
    }

    let flatten_ids = ids.into_iter().flatten().collect::<Vec<u64>>();
    flatten_ids
}

fn part_one(ids: &Vec<u64>) -> u64 {
    let mut result = 0;

    for id in ids {
        if Range::is_invalid_part_1(id) {
            result += id
        }
    }

    result
}

fn part_two(ids: &Vec<u64>) -> u64 {
    let mut result = 0;

    for id in ids {
        if Range::is_invalid_part_2(id) {
            result += id
        }
    }

    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-2/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let ids = parse_input(&buffer);
    let result_1 = part_one(&ids);
    let result_2 = part_two(&ids);

    println!("Result part 1: {}", result_1);
    println!("Result part 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn test_part_one() {
        let ids = parse_input(input());
        let result = part_one(&ids);
        assert_eq!(1227775554, result);
    }

    #[test]
    fn test_part_two() {
        let ids = parse_input(input());
        let result = part_two(&ids);
        assert_eq!(4174379265, result);
    }
}
