//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/09
use crate::util::Day;

pub struct D9;

const PREAMBLE: usize = 25;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

fn adds_up_to(numbers: &[i64], target: i64) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }
    false
}

/// Returns the first invalid number (the one that is not the sum of two of the
/// previous `PREAMBLE` numbers).
fn find_invalid(instructions: &[i64]) -> i64 {
    for window in instructions.windows(PREAMBLE + 1) {
        let n = window[PREAMBLE];
        if !adds_up_to(&window[..PREAMBLE], n) {
            return n;
        }
    }
    unreachable!()
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let instructions = parse(input);
        Some(find_invalid(&instructions).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let instructions = parse(input);
        let target = find_invalid(&instructions);

        // Replicate find_continuous_sum: for i in range, for j in range(i+1, len),
        // if sum(instructions[i:j]) == target -> min(slice) + max(slice).
        // The Python slice instructions[i:j] covers indices i..j (j exclusive).
        for i in 0..instructions.len() {
            for j in (i + 1)..instructions.len() {
                let slice = &instructions[i..j];
                if slice.iter().sum::<i64>() == target {
                    let mn = *slice.iter().min().unwrap();
                    let mx = *slice.iter().max().unwrap();
                    return Some((mn + mx).to_string());
                }
            }
        }
        None
    }
}
