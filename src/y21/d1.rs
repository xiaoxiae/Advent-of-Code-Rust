//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/01
use crate::util::Day;

pub struct D1;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let nums = parse(input);

        let total = nums.windows(2).filter(|w| w[0] < w[1]).count();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let nums = parse(input);

        let total = nums
            .windows(4)
            .filter(|w| {
                let a: i64 = w[0..3].iter().sum();
                let b: i64 = w[1..4].iter().sum();
                a < b
            })
            .count();

        Some(total.to_string())
    }
}
