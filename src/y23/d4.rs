//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/04
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D4;

fn matches(line: &str) -> usize {
    // line.split(maxsplit=2)[-1] => everything after "Card   N:"
    // Easiest: take substring after the colon.
    let after_colon = line.split_once(':').map(|(_, r)| r).unwrap_or(line).trim();

    let (winning_str, have_str) = after_colon.split_once(" | ").unwrap();

    let winning: FxHashSet<i64> = winning_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let have: FxHashSet<i64> = have_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    winning.intersection(&have).count()
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total: i64 = 0;
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let n = matches(line);
            if n > 0 {
                total += 1i64 << (n - 1);
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();
        let mut cards = vec![1i64; lines.len()];

        for i in 0..lines.len() {
            let n = matches(lines[i]);
            for j in (i + 1)..=(i + n) {
                if j < cards.len() {
                    cards[j] += cards[i];
                }
            }
        }

        Some(cards.iter().sum::<i64>().to_string())
    }
}
