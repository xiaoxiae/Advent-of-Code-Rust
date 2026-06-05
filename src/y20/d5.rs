//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/05
use crate::util::Day;

pub struct D5;

fn seat_id(line: &str) -> i64 {
    let chars: Vec<char> = line.chars().collect();

    let mut lo = 0;
    let mut hi = 128;
    for &c in &chars[..7] {
        let avg = (lo + hi) / 2;
        if c == 'F' {
            hi = avg;
        } else {
            lo = avg;
        }
    }
    let row = lo;

    let mut lo = 0;
    let mut hi = 8;
    for &c in &chars[7..] {
        let avg = (lo + hi) / 2;
        if c == 'L' {
            hi = avg;
        } else {
            lo = avg;
        }
    }
    let col = lo;

    (row * 8 + col) as i64
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let m = input.trim().lines().map(seat_id).max().unwrap();
        Some(m.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut ids: Vec<i64> = input.trim().lines().map(seat_id).collect();
        ids.sort();
        ids.windows(2)
            .find(|w| w[0] != w[1] - 1)
            .map(|w| (w[0] + 1).to_string())
    }
}
