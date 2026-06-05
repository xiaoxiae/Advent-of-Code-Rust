//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/17
use crate::util::Day;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct D17;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().chars().map(|c| (c as u8 - b'0') as i64).collect())
        .collect()
}

fn heuristic(x: i64, y: i64, ex: i64, ey: i64) -> i64 {
    (x - ex).abs() + (y - ey).abs()
}

fn solve(grid: &[Vec<i64>], part2: bool) -> i64 {
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;
    let ex = width - 1;
    let ey = height - 1;

    // visited set keyed by ((nx, ny), (ndx, ndy), nc)
    let mut visited: FxHashSet<((i64, i64), (i64, i64), i64)> = FxHashSet::default();
    visited.insert(((0, 0), (0, 0), 0));

    // queue entries: (priority, d, (x, y), (dx, dy), c)
    let mut queue: BinaryHeap<Reverse<(i64, i64, (i64, i64), (i64, i64), i64)>> = BinaryHeap::new();
    queue.push(Reverse((heuristic(0, 0, ex, ey), 0, (0, 0), (0, 0), 0)));

    let dirs = [(0i64, 1i64), (1, 0), (-1, 0), (0, -1)];

    while let Some(Reverse((_, d, (x, y), (dx, dy), c))) = queue.pop() {
        for &(ndx, ndy) in &dirs {
            // don't go back
            if (dx, dy) == (-ndx, -ndy) {
                continue;
            }

            let nc;
            if (dx, dy) == (ndx, ndy) {
                // same direction
                let limit = if part2 { 10 } else { 3 };
                if c == limit {
                    continue;
                }
                nc = c + 1;
            } else {
                // can't turn in less than 4 (besides start)
                if part2 && c <= 3 && (dx, dy) != (0, 0) {
                    continue;
                }
                nc = 1;
            }

            let nx = x + ndx;
            let ny = y + ndy;

            // check out of bounds
            if !(0 <= nx && nx < width && 0 <= ny && ny < height) {
                continue;
            }

            let ns = ((nx, ny), (ndx, ndy), nc);

            if visited.contains(&ns) {
                continue;
            }

            visited.insert(ns);

            let nd = d + grid[ny as usize][nx as usize];

            if (nx, ny) == (ex, ey) && (!part2 || nc >= 4) {
                return nd;
            }

            queue.push(Reverse((
                nd + heuristic(nx, ny, ex, ey),
                nd,
                (nx, ny),
                (ndx, ndy),
                nc,
            )));
        }
    }

    -1
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        Some(solve(&grid, false).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        Some(solve(&grid, true).to_string())
    }
}
