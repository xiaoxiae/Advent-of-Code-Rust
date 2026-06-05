//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/06
use crate::util::Day;

pub struct D6;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

fn simulate(input: &str, days: usize) -> u64 {
    let state = parse(input);

    let mut state_counts = [0u64; 9];
    for s in state {
        state_counts[s] += 1;
    }

    for _ in 0..days {
        let new_fish = state_counts[0];

        for i in 0..8 {
            state_counts[i] = state_counts[i + 1];
        }
        state_counts[8] = new_fish;
        state_counts[6] += new_fish;
    }

    state_counts.iter().sum()
}

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(simulate(input, 80).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(simulate(input, 256).to_string())
    }
}
