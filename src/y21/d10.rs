//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/10
use crate::util::Day;

pub struct D10;

fn matching(p: char) -> Option<char> {
    match p {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total: u64 = 0;
        for line in input.trim().lines() {
            let mut stack: Vec<char> = Vec::new();
            for p in line.chars() {
                if let Some(open) = matching(p) {
                    if stack.pop() != Some(open) {
                        total += match p {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        break;
                    }
                } else {
                    stack.push(p);
                }
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut scores: Vec<u64> = Vec::new();
        for line in input.trim().lines() {
            let mut stack: Vec<char> = Vec::new();
            let mut corrupted = false;
            for p in line.chars() {
                if let Some(open) = matching(p) {
                    if stack.pop() != Some(open) {
                        corrupted = true;
                        break;
                    }
                } else {
                    stack.push(p);
                }
            }
            if !corrupted {
                let mut subtotal: u64 = 0;
                for &p in stack.iter().rev() {
                    let points = match p {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    };
                    subtotal = subtotal * 5 + points;
                }
                scores.push(subtotal);
            }
        }
        scores.sort_unstable();
        Some(scores[scores.len() / 2].to_string())
    }
}
