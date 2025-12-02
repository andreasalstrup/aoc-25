use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

fn parse_input(buffer: &str) -> Vec<i32> {
    let mut values = Vec::new();

    for line in buffer.lines() {
        let (c, n) = line.split_at(1);

        let ch: char = c.chars().next().unwrap();
        let num = n.parse::<i32>().unwrap();

        match ch {
            'L' => values.push(-num),
            'R' => values.push(num),
            _ => {}
        }
    }

    values
}

fn part_one(values: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut dial = 50;

    for v in values.into_iter() {
        if (dial % 100) == 0 {
            result += 1;
        }

        dial += v;
    }

    result
}

fn part_two(values: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut dial: i32 = 50;

    for v in values {
        let mut num = v.abs();

        while num > 0 {
            if v.is_negative() {
                dial -= 1;
            } else {
                dial += 1;
            }

            if dial.abs() == 100 {
                dial = 0;
            }

            if dial == 0 {
                result += 1;
            }

            num -= 1;
        }
    }

    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-1/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let values = parse_input(&buffer);

    println!("Result part 1: {}", part_one(values.clone()));
    println!("Result part 2: {}", part_two(values));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
    }

    #[test]
    fn test_part_two() {
        let values = parse_input(input());
        let result = part_two(values);
        assert_eq!(6, result);
    }
}
