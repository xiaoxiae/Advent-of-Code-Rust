//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/01
use crate::util::Day;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total: i64 = 0;
        for line in input.lines() {
            let numerics: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            if numerics.is_empty() {
                continue;
            }
            let first = numerics[0];
            let last = *numerics.last().unwrap();
            total += (first * 10 + last) as i64;
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut total: i64 = 0;
        for line in input.lines() {
            let bytes = line.as_bytes();
            let mut numbers: Vec<i64> = Vec::new();
            for i in 0..bytes.len() {
                let c = bytes[i];
                if c.is_ascii_digit() {
                    numbers.push((c - b'0') as i64);
                    continue;
                }

                for (j, digit) in digits.iter().enumerate() {
                    let dlen = digit.len();
                    if i + dlen <= bytes.len() && &line[i..i + dlen] == *digit {
                        numbers.push((j + 1) as i64);
                        break;
                    }
                }
            }

            if numbers.is_empty() {
                continue;
            }
            total += 10 * numbers[0] + numbers.last().unwrap();
        }
        Some(total.to_string())
    }
}
