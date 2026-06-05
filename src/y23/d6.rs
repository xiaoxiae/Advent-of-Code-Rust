//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/06
use crate::util::Day;

pub struct D6;

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();

        let times: Vec<i64> = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();
        let distances: Vec<i64> = lines[1]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect();

        let product: i64 = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| (0..time).filter(|i| i * (time - i) > distance).count() as i64)
            .product();

        Some(product.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();

        let time: i64 = lines[0]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();
        let distance: i64 = lines[1]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse()
            .unwrap();

        let mut start: i64 = 0;
        for i in 0..time {
            if i * (time - i) > distance {
                start = i;
                break;
            }
        }

        let mut end: i64 = 0;
        for i in (0..time).rev() {
            if i * (time - i) > distance {
                end = i;
                break;
            }
        }

        Some((end - start + 1).to_string())
    }
}
