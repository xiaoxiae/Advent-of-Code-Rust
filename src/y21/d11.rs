//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/11
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D11;

const NEIGHBOURS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn do_step(grid: &mut [Vec<i32>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut flashed: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut to_flash: Vec<(usize, usize)> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            grid[y][x] += 1;

            if grid[y][x] >= 10 {
                to_flash.push((y, x));
                flashed.insert((y, x));
            }
        }
    }

    while let Some((y, x)) = to_flash.pop() {
        for (dx, dy) in NEIGHBOURS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if !(nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            grid[ny][nx] += 1;

            if grid[ny][nx] >= 10 && !flashed.contains(&(ny, nx)) {
                to_flash.push((ny, nx));
                flashed.insert((ny, nx));
            }
        }
    }

    for &(y, x) in &flashed {
        grid[y][x] = 0;
    }

    flashed.len()
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut grid = parse(input);

        let mut total = 0;
        for _ in 0..100 {
            total += do_step(&mut grid);
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut grid = parse(input);
        let target = grid.len() * grid[0].len();

        let mut i = 1;
        loop {
            if do_step(&mut grid) == target {
                break;
            }
            i += 1;
        }

        Some(i.to_string())
    }
}
