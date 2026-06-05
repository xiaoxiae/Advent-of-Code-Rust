//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/02
use crate::util::Day;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for value in input.trim().lines() {
            let mut parts = value.split_whitespace();
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            let password = parts.next().unwrap();

            let mut bounds = a.split('-');
            let lo: usize = bounds.next().unwrap().parse().unwrap();
            let hi: usize = bounds.next().unwrap().parse().unwrap();

            let policy = b.trim_matches(':');
            let policy_char = policy.chars().next().unwrap();

            let count = password.chars().filter(|&c| c == policy_char).count();

            if lo <= count && count <= hi {
                total += 1;
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for value in input.trim().lines() {
            let mut parts = value.split_whitespace();
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            let password = parts.next().unwrap();

            let mut bounds = a.split('-');
            let i: usize = bounds.next().unwrap().parse::<usize>().unwrap() - 1;
            let j: usize = bounds.next().unwrap().parse::<usize>().unwrap() - 1;

            let policy = b.trim_matches(':');
            let policy_char = policy.chars().next().unwrap();

            let chars: Vec<char> = password.chars().collect();
            let ci = chars[i];
            let cj = chars[j];

            if (ci == policy_char || cj == policy_char) && ci != cj {
                total += 1;
            }
        }

        Some(total.to_string())
    }
}
