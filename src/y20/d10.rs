//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/10
use crate::util::Day;

pub struct D10;

fn parse(input: &str) -> Vec<i64> {
    let mut inst: Vec<i64> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();
    inst.sort();
    inst
}

fn fill_ones(ones: i64) -> i64 {
    match ones {
        0 | 1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => 0,
    }
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let inst = parse(input);

        let mut one_jolt = 1;
        let mut three_jolt = 1;

        for w in inst.windows(2) {
            if w[1] - w[0] == 1 {
                one_jolt += 1;
            } else {
                three_jolt += 1;
            }
        }

        Some((one_jolt * three_jolt).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let inst = parse(input);

        let mut deltas = vec![1i64];

        for w in inst.windows(2) {
            deltas.push(w[1] - w[0]);
        }

        deltas.push(3);

        let mut total: i64 = 1;
        let mut ones: i64 = 0;
        for &d in &deltas {
            if d == 1 {
                ones += 1;
            } else {
                total *= fill_ones(ones);
                ones = 0;
            }
        }

        Some(total.to_string())
    }
}
