//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/11
use crate::util::Day;
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub struct D11;

fn solve(input: &str, expansions: i64) -> i64 {
    let rows: Vec<&str> = input.lines().collect();

    let mut universes: Vec<(usize, usize)> = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                universes.push((x, y));
            }
        }
    }

    let mut empty_rows: FxHashSet<usize> = FxHashSet::default();
    for (y, row) in rows.iter().enumerate() {
        if !row.contains('#') {
            empty_rows.insert(y);
        }
    }

    let width = rows[0].len();
    let row_bytes: Vec<&[u8]> = rows.iter().map(|r| r.as_bytes()).collect();

    let mut empty_columns: FxHashSet<usize> = FxHashSet::default();
    for x in 0..width {
        let has_galaxy = row_bytes
            .iter()
            .any(|rb| x < rb.len() && rb[x] == b'#');
        if !has_galaxy {
            empty_columns.insert(x);
        }
    }

    let mut total: i64 = 0;
    for pair in universes.iter().combinations(2) {
        let (x1, y1) = *pair[0];
        let (x2, y2) = *pair[1];

        let mut distance: i64 = 0;
        for x in x1.min(x2)..x1.max(x2) {
            if empty_columns.contains(&x) {
                distance += expansions;
            }
            distance += 1;
        }

        for y in y1.min(y2)..y1.max(y2) {
            if empty_rows.contains(&y) {
                distance += expansions;
            }
            distance += 1;
        }

        total += distance;
    }

    total
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(solve(input, 1).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(solve(input, 1000000 - 1).to_string())
    }
}
