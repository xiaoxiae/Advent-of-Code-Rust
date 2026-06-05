//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/01
use crate::util::Day;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total: i64 = input
            .trim()
            .lines()
            .map(|x| x.trim().parse::<i64>().unwrap() / 3 - 2)
            .sum();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total: i64 = 0;

        for line in input.trim().lines() {
            let mut subtotal: i64 = 0;
            let mut instruction = line.trim().parse::<i64>().unwrap() / 3 - 2;

            while instruction > 0 {
                subtotal += instruction;
                instruction = instruction / 3 - 2;
            }

            total += subtotal;
        }

        Some(total.to_string())
    }
}
