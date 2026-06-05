//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/06
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D6;

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let input = input.trim();
        let total: usize = input
            .split("\n\n")
            .map(|group| {
                group
                    .chars()
                    .filter(|&c| c != ' ' && c != '\n')
                    .collect::<FxHashSet<char>>()
                    .len()
            })
            .sum();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let input = input.trim();
        let total: usize = input
            .split("\n\n")
            .map(|group| {
                let mut all_yes: FxHashSet<char> = ('a'..='z').collect();
                for line in group.split('\n') {
                    let line_set: FxHashSet<char> =
                        line.chars().filter(|&c| c != ' ').collect();
                    all_yes = all_yes.intersection(&line_set).copied().collect();
                }
                all_yes.len()
            })
            .sum();
        Some(total.to_string())
    }
}
