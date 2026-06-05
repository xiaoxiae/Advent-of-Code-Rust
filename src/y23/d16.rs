//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/16
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D16;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_matches('\n')
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect()
}

fn energized(grid: &[Vec<u8>], start: (i64, i64), direction: (i64, i64)) -> usize {
    let h = grid.len() as i64;
    let w = if h > 0 { grid[0].len() as i64 } else { 0 };

    // state: ((x, y), (dx, dy))
    let mut visited: FxHashSet<((i64, i64), (i64, i64))> = FxHashSet::default();
    let mut queue: Vec<((i64, i64), (i64, i64))> = vec![(start, direction)];

    while let Some(s) = queue.pop() {
        if !visited.insert(s) {
            continue;
        }

        let ((x, y), (dx, dy)) = s;
        let nx = x + dx;
        let ny = y + dy;

        // skip out of bounds
        if !(0 <= nx && nx < w && 0 <= ny && ny < h) {
            continue;
        }

        let c = grid[ny as usize][nx as usize];

        // split |
        if c == b'|' && dx != 0 {
            queue.push(((nx, ny), (0, 1)));
            queue.push(((nx, ny), (0, -1)));
            continue;
        }

        // split -
        if c == b'-' && dy != 0 {
            queue.push(((nx, ny), (1, 0)));
            queue.push(((nx, ny), (-1, 0)));
            continue;
        }

        // reflect /
        if c == b'/' {
            queue.push(((nx, ny), (-dy, -dx)));
            continue;
        }

        // reflect \
        if c == b'\\' {
            queue.push(((nx, ny), (dy, dx)));
            continue;
        }

        // continue
        queue.push(((nx, ny), (dx, dy)));
    }

    let positions: FxHashSet<(i64, i64)> = visited.iter().map(|(v, _)| *v).collect();
    positions.len()
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let answer = energized(&grid, (-1, 0), (1, 0)) - 1;
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let h = grid.len() as i64;
        let w = if h > 0 { grid[0].len() as i64 } else { 0 };

        let mut max_energy = 0usize;

        for x in 0..w {
            max_energy = max_energy.max(energized(&grid, (x, -1), (0, 1)) - 1);
            max_energy = max_energy.max(energized(&grid, (x, h), (0, -1)) - 1);
        }

        for y in 0..h {
            max_energy = max_energy.max(energized(&grid, (-1, y), (1, 0)) - 1);
            max_energy = max_energy.max(energized(&grid, (w, y), (-1, 0)) - 1);
        }

        Some(max_energy.to_string())
    }
}
