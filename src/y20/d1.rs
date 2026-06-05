//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/01
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D1;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let values = parse(input);
        let target: i64 = 2020;
        let mut saw: FxHashSet<i64> = FxHashSet::default();
        for value in values {
            if saw.contains(&(target - value)) {
                return Some((value * (target - value)).to_string());
            }
            saw.insert(value);
        }
        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let values = parse(input);
        let target: i64 = 2020;
        let mut saw: FxHashSet<i64> = FxHashSet::default();
        for i in 0..values.len() {
            let v1 = values[i];
            for &v2 in &values[i + 1..] {
                if saw.contains(&(target - v1 - v2)) {
                    return Some((v1 * v2 * (target - v1 - v2)).to_string());
                }
            }
            saw.insert(v1);
        }
        None
    }
}
