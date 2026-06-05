//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/02
use crate::util::Day;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut forward: i64 = 0;
        let mut depth: i64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let value: i64 = parts.next().unwrap().parse().unwrap();

            match instruction {
                "down" => depth += value,
                "up" => depth -= value,
                "forward" => forward += value,
                _ => {}
            }
        }

        Some((depth * forward).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut forward: i64 = 0;
        let mut depth: i64 = 0;
        let mut aim: i64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let value: i64 = parts.next().unwrap().parse().unwrap();

            match instruction {
                "down" => aim += value,
                "up" => aim -= value,
                "forward" => {
                    forward += value;
                    depth += value * aim;
                }
                _ => {}
            }
        }

        Some((depth * forward).to_string())
    }
}
