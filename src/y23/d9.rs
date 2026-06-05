//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/09
use crate::util::Day;

pub struct D9;

fn get_diffs(numbers: &[i64]) -> Vec<i64> {
    numbers.windows(2).map(|w| w[1] - w[0]).collect()
}

fn build_pyramid(numbers: Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid: Vec<Vec<i64>> = vec![numbers];

    loop {
        let last = pyramid.last().unwrap();
        if last.len() < 2 || (last[0] == 0 && last[1] == 0) {
            break;
        }
        let diffs = get_diffs(last);
        pyramid.push(diffs);
    }

    pyramid
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total: i64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let mut pyramid = build_pyramid(numbers);

            for i in (0..pyramid.len() - 1).rev() {
                let add = pyramid[i].last().unwrap() + pyramid[i + 1].last().unwrap();
                pyramid[i].push(add);
            }

            total += *pyramid[0].last().unwrap();
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total: i64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let mut pyramid = build_pyramid(numbers);

            for i in (0..pyramid.len() - 1).rev() {
                let val = pyramid[i][0] - pyramid[i + 1][0];
                pyramid[i].insert(0, val);
            }

            total += pyramid[0][0];
        }

        Some(total.to_string())
    }
}
