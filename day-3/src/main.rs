use std::collections::HashSet;
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;

struct MonotonicStack {
    data: Vec<u32>,
    stack: Vec<usize>,
}

impl MonotonicStack {
    pub fn new(data: Vec<u32>) -> Self {
        // let stack_len = data.len();
        MonotonicStack {
            data,
            stack: Vec::new(),
        }
    }

    pub fn decreasing(&mut self) -> Vec<u32> {
        // println!("data: {:?}, stack: {:?}", self.data, self.stack);

        let mut result = vec![0; self.data.len()];

        for (i, value) in self.data.iter().enumerate() {
            while let Some(&top_idx) = self.stack.last() {
                let top = self.data[top_idx];
                let curr = self.data[i];
                println!("{:?}  {:?}", top, curr);

                if top < curr {
                    self.stack.pop();
                    self.stack.push(i);
                } else {
                    self.stack.push(i);
                }

                if self.stack.len() >= 12 {
                    break;
                }
                
                // if curr <= top {
                //     break;
                // }
                // println!("last: {}", last);
                // panic!("!!!!!!!");

                // let index = self.stack.pop().unwrap();
                // result[index] = *value;
                println!("stack: {:?}", self.stack);

            }

            if self.stack.len() < 12 {
                self.stack.push(i);
            }
        }
        //[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        //[3, 4, 7, 3, 4, 7, 3, 4, 7, 3, 4, 7, 7, 8, 0]
        //    4     3  4     3  4     3  4     7  8
        // 
        // 4  3  4  2  3  4  2  3  4  2  7  8
        // [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        // [9, 8, 9, 8, 9, 9, 0, 2, 2, 2, 2, 0, 0, 0, 0]
        //  888911112111          

        // println!("data: {:?}", self.data);
        // for (idx, value) in self.data.iter().enumerate() {
        //     let last = self.stack.last().copied().unwrap();
        //     while !self.stack.is_empty() && *value > self.data[last] {
        //         let index = self.stack.pop().unwrap();
        //         result[index] = self.data[idx];
        //     }
        //
        //     self.stack.push(idx);

        // while let Some(&top) = self.stack.last() {
        //     if self.data[idx] > self.data[top] {
        //         let poped = self.stack.pop().unwrap();
        //         println!(
        //             "result[{}]: {}, self.data[{}]: {:?}",
        //             poped, result[poped], idx, self.data[idx]
        //         );
        //         result[poped] = self.data[idx];
        //     } else {
        //         break;
        //     }
        // }
        //
        // self.stack.push(idx);
        // println!("result: {:?}", result);
        // }
        // for (idx, _) in self.data.iter().enumerate() {
        //     if let Some(top) = self.stack.last().copied() {
        //         // println!("top: {}", top);
        //         // println!("stack ?: {} - {} > {} = {}", !self.stack.is_empty(), self.data[idx], self.data[top], !self.stack.is_empty() && self.data[idx] > self.data[top]);
        //         while !self.stack.is_empty() && self.data[idx] > self.data[top] {
        //             result[self.stack.pop().unwrap()] = self.data[idx];
        //             // println!("result : {:?}", result);
        //         }
        //     }
        //
        //     // println!("stack: {:?}", self.stack);
        //     self.stack.push(idx);
        // }
        println!("stack: {:?}", self.stack);

        result
    }

    fn push(&mut self, value: usize) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> usize {
        let index = self.stack.pop().unwrap();
        index
    }
}

fn parse_input(buffer: &str) -> Vec<Vec<u32>> {
    let mut banks = Vec::new();
    for line in buffer.lines() {
        banks.push(line);
    }

    let mut result = Vec::new();
    for bank in banks {
        let bank_num = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        result.push(bank_num);
    }

    result
}

fn part_one(banks: &Vec<Vec<u32>>) -> u64 {
    let mut banks_set = Vec::new();

    for bank in banks {
        let mut combination = HashSet::new();

        for (ref mut i, b) in bank.iter().enumerate() {
            *i += 1;
            for (_, x) in bank[*i..bank.len()].iter().enumerate() {
                combination.insert((b, x));
            }
        }

        banks_set.push(combination);
    }

    let mut values = Vec::new();
    for s in &banks_set {
        let res = s.iter().max_by(|a, b| {
            let a_1_str = a.0.to_string();
            let a_2_str = a.1.to_string();
            let a_str = a_1_str + &a_2_str;

            let b_1_str = b.0.to_string();
            let b_2_str = b.1.to_string();
            let b_str = b_1_str + &b_2_str;

            let a_num = a_str.parse::<u32>().unwrap();
            let b_num = b_str.parse::<u32>().unwrap();

            a_num.cmp(&b_num)
        });

        values.push(res.unwrap());
    }

    let mut acc = 0;
    for v in &values {
        let a = format!("{}{}", v.0, v.1).parse::<u32>().unwrap();
        acc += a;
    }

    println!("values {:?}", values);

    acc.into()
}

fn find_cfg(bank: &Vec<u32>) -> u64 {
    let mut result = Vec::new();

    let max_size = bank.len();
    let mut windows_size: usize = 12;
    let mut window_start: usize = 0;
    let mut window_end: usize = windows_size;

    println!("bank:   {:?}", bank);
    while windows_size != 0 {
        if result.len() >= windows_size {
            break;
        }

        let idx = &bank[window_start..window_end]
            .iter()
            .enumerate()
            .fold(None, |acc, curr| match acc {
                None => Some(curr),
                Some(prev) => {
                    if curr.1 > prev.1 {
                        Some(curr)
                    } else {
                        Some(prev)
                    }
                }
            });

        let first_idx = idx.unwrap().0;
        println!("first idx {:?}", first_idx);

        if (first_idx + window_end) > max_size {
            let window = &bank[max_size - first_idx..];
            println!("WINDOW {:?} - {:?} {:?}", max_size, first_idx, window);
            result.extend(window);
            break;
        }

        result.push(idx.unwrap().1);
        window_start = idx.unwrap().0 + 1;
        window_end = window_start + windows_size;
        println!("result: {:?}", result);
    }
    println!("RESULT: {:?}", result);

    let num: String = result.iter().map(|n| n.to_string()).collect();

    num.parse().unwrap()
}

fn part_two(banks: &mut Vec<Vec<u32>>) -> u64 {
    for bank in banks.iter().skip(2) {
        let res = find_cfg(&bank);
        println!("{}", res);
        let mut m_stack = MonotonicStack::new(bank.to_vec());
        let result = m_stack.decreasing();
        println!("\nReSult: {:?}", result);
        // println!("Data:   {:?}", m_stack.data);
        // println!("Stack:  {:?}", m_stack.stack);
        // break;
        // return 0;
    }
    0
}

// fn part_two(banks: &mut Vec<Vec<u32>>) -> u64 {
//     // println!("banks {:?}", banks);
//
//     for bank in banks.iter_mut() {
//         println!("bank {:?}", bank);
//
//         for count in 1..12 {
//             if bank[0] < bank[1] {
//                 bank.remove(0);
//             }
//
//             let mut matches: Vec<usize> = bank
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(i, n)| (*n == count).then_some(i))
//                 .collect();
//
//             if matches.len() > 3 {
//                 println!("matches: {:?}", matches);
//             }
//
//             matches.sort_unstable_by(|a, b| b.cmp(a));
//             for idx in matches {
//                 if bank.len() <= 12 {
//                     break;
//                 }
//
//                 println!("remove: {}", idx);
//                 bank.remove(idx);
//             }
//
//             // println!("count {:?} - {:?}", count, matches);
//             // if matches.len() >= 3 {
//             //     println!("1bank {:?}", bank.clone());
//             //     println!("matches: {:?}", matches.clone());
//             //     matches.truncate(3);
//             //     for idx in matches {
//             //         bank[idx] = 0;
//             //     }
//             //
//             //     bank.retain(|b| *b != 0);
//             //     break;
//             // }
//         }
//     }
//
//     let mut values = Vec::new();
//     for bank in banks.clone() {
//         let str = bank.iter().fold(String::new(), |acc, n| {
//             let s = format!("{}", *n);
//             acc + &s
//         });
//
//         values.push(str);
//     }
//
//     let mut acc = 0;
//     for v in values {
//         println!("{:?}", v);
//         acc += v.parse::<u64>().unwrap();
//     }
//
//     acc
// }

fn main() -> Result<()> {
    let mut file = File::open("./day-3/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let ids = parse_input(&buffer);
    let result_1 = part_one(&ids);

    println!("Result part 1: {}", result_1);
    // println!("Result part 2: {}", result_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "987654321111111
811111111111119
234234234234278
818181911112111"
    }

    #[test]
    fn test_part_one() {
        let banks = parse_input(input());
        let result = part_one(&banks);
        assert_eq!(357, result);
    }

    #[test]
    fn test_part_two() {
        let mut banks = parse_input(input());
        let result = part_two(&mut banks);
        assert_eq!(3121910778619, result);
    }
}
