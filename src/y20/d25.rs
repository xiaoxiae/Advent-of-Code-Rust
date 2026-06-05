//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/25
use crate::util::Day;

pub struct D25;

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut lines = input.trim().lines();
        let p1: u64 = lines.next()?.trim().parse().ok()?;
        let p2: u64 = lines.next()?.trim().parse().ok()?;

        const MODULUS: u64 = 20201227;

        let mut l1 = 0u64;
        let mut v = 1u64;
        while v != p1 {
            v *= 7;
            v %= MODULUS;
            l1 += 1;
        }

        let mut v = 1u64;
        for _ in 0..l1 {
            v *= p2;
            v %= MODULUS;
        }

        Some(v.to_string())
    }

    fn solve_part2(&self, _input: &str) -> Option<String> {
        None
    }
}
