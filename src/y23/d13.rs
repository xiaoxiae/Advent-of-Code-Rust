//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/13
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D13;

/// Check whether there is a reflection between `row` and `row + 1`.
fn _reflective(array: &[Vec<u8>], row: usize) -> bool {
    let n = array.len();
    // Python: min(row + 1, len(array) - row - 1)
    let limit = (row + 1).min(n - row - 1);
    for i in 0..limit {
        if array[row - i] != array[row + i + 1] {
            return false;
        }
    }
    true
}

fn transpose(array: &[Vec<u8>]) -> Vec<Vec<u8>> {
    if array.is_empty() {
        return Vec::new();
    }
    let cols = array[0].len();
    let mut out = vec![Vec::with_capacity(array.len()); cols];
    for row in array {
        for (x, &c) in row.iter().enumerate() {
            out[x].push(c);
        }
    }
    out
}

/// Collect all reflective scores (matching Python's generator semantics).
fn reflective_scores(pattern: &[Vec<u8>]) -> Vec<i64> {
    let mut scores = Vec::new();

    for i in 0..pattern.len().saturating_sub(1) {
        if _reflective(pattern, i) {
            scores.push(((i + 1) * 100) as i64);
        }
    }

    let t = transpose(pattern);
    for i in 0..t.len().saturating_sub(1) {
        if _reflective(&t, i) {
            scores.push((i + 1) as i64);
        }
    }

    scores
}

fn reflective_score(pattern: &[Vec<u8>]) -> i64 {
    for i in 0..pattern.len().saturating_sub(1) {
        if _reflective(pattern, i) {
            return ((i + 1) * 100) as i64;
        }
    }

    let t = transpose(pattern);
    for i in 0..t.len().saturating_sub(1) {
        if _reflective(&t, i) {
            return (i + 1) as i64;
        }
    }

    0
}

fn smudged_reflective_score(pattern: &[Vec<u8>]) -> i64 {
    let mut pattern: Vec<Vec<u8>> = pattern.to_vec();

    let baseline: FxHashSet<i64> = reflective_scores(&pattern).into_iter().collect();

    let height = pattern.len();
    let width = pattern.first().map_or(0, Vec::len);

    for y in 0..height {
        for x in 0..width {
            let orig = pattern[y][x];
            pattern[y][x] = if orig == b'#' { b'.' } else { b'#' };

            let new: FxHashSet<i64> = reflective_scores(&pattern).into_iter().collect();
            if !new.is_empty() && new != baseline {
                // list(new - baseline)[0]
                return *new.difference(&baseline).next().unwrap();
            }

            pattern[y][x] = orig;
        }
    }

    0
}

fn parse(input: &str) -> Vec<Vec<Vec<u8>>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.as_bytes().to_vec())
                .collect::<Vec<Vec<u8>>>()
        })
        .filter(|p: &Vec<Vec<u8>>| !p.is_empty())
        .collect()
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let patterns = parse(input);
        let total: i64 = patterns.iter().map(|p| reflective_score(p)).sum();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let patterns = parse(input);
        let total: i64 = patterns.iter().map(|p| smudged_reflective_score(p)).sum();
        Some(total.to_string())
    }
}
