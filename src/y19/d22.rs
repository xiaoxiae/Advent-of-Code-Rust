//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/22
//!
//! ⚠️ part 2 UNSOLVED: the original 22-2.py was incomplete (never solved). Left as
//! None — not translating an agent-invented solution. Part 1 (2514) matched Python.
use crate::util::Day;

pub struct D22;

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let techniques: Vec<&str> = input.trim().lines().collect();

        let n = 10_007usize;
        let mut deck: Vec<usize> = (0..n).collect();

        for technique in &techniques {
            if technique.starts_with("deal into") {
                deck.reverse();
            } else {
                let value: i64 = technique[technique.rfind(' ').unwrap() + 1..]
                    .trim()
                    .parse()
                    .unwrap();

                if technique.starts_with("cut") {
                    // deck = deck[value:] + deck[:value], supporting negatives
                    let len = deck.len() as i64;
                    let v = ((value % len) + len) % len;
                    let v = v as usize;
                    deck.rotate_left(v);
                } else {
                    // deal with increment
                    let len = deck.len();
                    let value = value as usize;
                    let mut new_deck = vec![0usize; len];
                    let mut i = 0usize;
                    for src in 0..len {
                        new_deck[i % len] = deck[src];
                        i += value;
                    }
                    deck = new_deck;
                }
            }
        }

        let idx = deck.iter().position(|&x| x == 2019).unwrap();
        Some(idx.to_string())
    }

    fn solve_part2(&self, _input: &str) -> Option<String> {
        // The original 22-2.py was incomplete (never solved). Not translating an
        // agent-invented solution here — left unsolved pending a manual
        // implementation.
        None
    }
}
