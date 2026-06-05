//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/07
use crate::util::Day;

pub struct D7;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let nums = parse(input);
        let lo = *nums.iter().min().unwrap();
        let hi = *nums.iter().max().unwrap();

        let min_total = (lo..hi)
            .map(|level| nums.iter().map(|&n| (n - level).abs()).sum::<i64>())
            .min()
            .unwrap_or(i64::MAX);

        Some(min_total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let nums = parse(input);
        let lo = *nums.iter().min().unwrap();
        let hi = *nums.iter().max().unwrap();

        let min_total = (lo..hi)
            .map(|level| {
                nums.iter()
                    .map(|&n| {
                        let x = (n - level).abs();
                        x * (x + 1) / 2
                    })
                    .sum::<i64>()
            })
            .min()
            .unwrap_or(i64::MAX);

        Some(min_total.to_string())
    }
}
