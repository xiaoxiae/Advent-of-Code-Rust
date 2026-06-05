//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/08
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D8;

fn parse(input: &str) -> Vec<(&str, i64)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (opt, arg) = line.split_once(' ').unwrap();
            (opt, arg.parse::<i64>().unwrap())
        })
        .collect()
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let instructions = parse(input);

        let mut acc: i64 = 0;
        let mut ptr: i64 = 0;
        let mut visited: FxHashSet<i64> = FxHashSet::default();

        loop {
            if visited.contains(&ptr) {
                return Some(acc.to_string());
            }
            visited.insert(ptr);

            let (opt, arg) = instructions[ptr as usize];

            if opt == "acc" {
                acc += arg;
            } else if opt == "jmp" {
                ptr += arg;
                continue;
            }

            ptr += 1;
        }
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let instructions = parse(input);
        let n = instructions.len();

        for i in 0..n {
            let mut acc: i64 = 0;
            let mut ptr: i64 = 0;
            let mut visited: FxHashSet<i64> = FxHashSet::default();

            loop {
                if ptr == n as i64 {
                    return Some(acc.to_string());
                }

                if visited.contains(&ptr) {
                    break;
                }

                visited.insert(ptr);

                // A wrong flip can send ptr out of bounds (Python would wrap on a
                // negative index); the correct flip terminates at ptr == n exactly,
                // so any out-of-range ptr means this flip is invalid.
                if ptr < 0 || ptr as usize >= n {
                    break;
                }

                let (mut opt, arg) = instructions[ptr as usize];

                if i as i64 == ptr {
                    if opt == "jmp" {
                        opt = "nop";
                    } else {
                        opt = "jmp";
                    }
                }

                if opt == "acc" {
                    acc += arg;
                } else if opt == "jmp" {
                    ptr += arg;
                    continue;
                }

                ptr += 1;
            }
        }

        None
    }
}
