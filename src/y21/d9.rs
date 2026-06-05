//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/09
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D9;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

const NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn in_bounds(grid: &[Vec<i32>], x: i32, y: i32) -> bool {
    x >= 0 && (x as usize) < grid[0].len() && y >= 0 && (y as usize) < grid.len()
}

fn get_neighbours(grid: &[Vec<i32>], x: i32, y: i32) -> Vec<(i32, i32)> {
    NEIGHBOURS
        .iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(nx, ny)| in_bounds(grid, nx, ny))
        .collect()
}

fn is_lowpoint(grid: &[Vec<i32>], x: i32, y: i32) -> bool {
    let val = grid[y as usize][x as usize];
    get_neighbours(grid, x, y)
        .into_iter()
        .all(|(nx, ny)| grid[ny as usize][nx as usize] > val)
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);

        let mut total: i32 = 0;
        for y in 0..grid.len() as i32 {
            for x in 0..grid[0].len() as i32 {
                let val = grid[y as usize][x as usize];
                let mut is_low = true;
                for (dx, dy) in NEIGHBOURS {
                    let nx = x + dx;
                    let ny = y + dy;
                    if !in_bounds(&grid, nx, ny) {
                        continue;
                    }
                    if grid[ny as usize][nx as usize] <= val {
                        is_low = false;
                        break;
                    }
                }
                if is_low {
                    total += 1 + val;
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);

        let mut basins: FxHashMap<(i32, i32), i64> = FxHashMap::default();

        for y in 0..grid.len() as i32 {
            for x in 0..grid[0].len() as i32 {
                if grid[y as usize][x as usize] == 9 {
                    continue;
                }

                // flow_to_lowpoint
                let mut cx = x;
                let mut cy = y;
                while !is_lowpoint(&grid, cx, cy) {
                    let val = grid[cy as usize][cx as usize];
                    for (nx, ny) in get_neighbours(&grid, cx, cy) {
                        if val > grid[ny as usize][nx as usize] {
                            cx = nx;
                            cy = ny;
                            break;
                        }
                    }
                }

                *basins.entry((cx, cy)).or_insert(0) += 1;
            }
        }

        let mut v: Vec<i64> = basins.values().copied().collect();
        v.sort();
        let n = v.len();
        let answer = v[n - 1] * v[n - 2] * v[n - 3];

        Some(answer.to_string())
    }
}
