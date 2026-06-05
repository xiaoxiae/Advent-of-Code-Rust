//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/15
use crate::util::Day;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use rustc_hash::FxHashSet;

pub struct D15;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect()
}

fn solve<F>(w: i64, h: i64, get: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    let start = (0i64, 0i64);
    let end = (w - 1, h - 1);

    let mut heap: BinaryHeap<Reverse<(i64, (i64, i64))>> = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    let mut explored: FxHashSet<(i64, i64)> = FxHashSet::default();
    explored.insert(start);

    while let Some(Reverse((d, (x, y)))) = heap.pop() {
        if (x, y) == end {
            return d;
        }

        for (dx, dy) in [(0i64, 1i64), (1, 0), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            if !(0 <= nx && nx < w && 0 <= ny && ny < h) {
                continue;
            }

            if explored.contains(&(nx, ny)) {
                continue;
            }

            heap.push(Reverse((d + get(nx, ny), (nx, ny))));
            explored.insert((nx, ny));
        }
    }

    unreachable!()
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let w = grid[0].len() as i64;
        let h = grid.len() as i64;

        let answer = solve(w, h, |x, y| grid[y as usize][x as usize]);
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let prev_w = grid[0].len() as i64;
        let prev_h = grid.len() as i64;
        let w = prev_w * 5;
        let h = prev_h * 5;

        let answer = solve(w, h, |x, y| {
            let nx = x % prev_w;
            let ny = y % prev_h;
            let cx = x / prev_w;
            let cy = y / prev_h;
            (grid[ny as usize][nx as usize] + cx + cy - 1) % 9 + 1
        });
        Some(answer.to_string())
    }
}
