//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/03
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D3;

/// Pads the input the same way the Python does: surrounds every line with "."
/// and adds a row of "." above and below.
fn build_grid(input: &str) -> Vec<Vec<u8>> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let mut padded: Vec<Vec<u8>> = Vec::with_capacity(lines.len() + 2);

    let width = lines.first().map_or(0, |l| l.len());

    // top border row: "." * (len + 2)
    padded.push(vec![b'.'; width + 2]);

    for line in &lines {
        let mut row = Vec::with_capacity(line.len() + 2);
        row.push(b'.');
        row.extend_from_slice(line.as_bytes());
        row.push(b'.');
        padded.push(row);
    }

    // bottom border row
    padded.push(vec![b'.'; width + 2]);

    padded
}

fn is_numeric(b: u8) -> bool {
    b.is_ascii_digit()
}

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = build_grid(input);

        let is_near_symbol = |x: usize, y: usize| -> bool {
            const DELTAS: [(isize, isize); 8] = [
                (0, 1), (1, 0), (-1, 0), (0, -1),
                (-1, 1), (1, -1), (-1, -1), (1, 1),
            ];
            for (dx, dy) in DELTAS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if ny < 0 || nx < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }
                let c = grid[ny][nx];
                if !is_numeric(c) && c != b'.' {
                    return true;
                }
            }
            false
        };

        let mut total: i64 = 0;

        for y in 0..grid.len() {
            let mut start: Option<usize> = None;
            let mut is_valid = false;

            for x in 0..grid[y].len() {
                if is_numeric(grid[y][x]) {
                    if start.is_none() {
                        start = Some(x);
                    }
                    is_valid = is_valid || is_near_symbol(x, y);
                } else {
                    if let Some(s) = start {
                        if is_valid {
                            let num: i64 = std::str::from_utf8(&grid[y][s..x])
                                .unwrap()
                                .parse()
                                .unwrap();
                            total += num;
                        }
                    }
                    start = None;
                    is_valid = false;
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = build_grid(input);

        let get_nearby_gears = |x: usize, y: usize| -> Vec<(usize, usize)> {
            const DELTAS: [(isize, isize); 8] = [
                (0, 1), (1, 0), (-1, 0), (0, -1),
                (-1, 1), (1, -1), (-1, -1), (1, 1),
            ];
            let mut gears = Vec::new();
            for (dx, dy) in DELTAS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if ny < 0 || nx < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }
                if grid[ny][nx] == b'*' {
                    gears.push((nx, ny));
                }
            }
            gears
        };

        let mut gear_dict: FxHashMap<(usize, usize), Vec<i64>> = FxHashMap::default();

        for y in 0..grid.len() {
            let mut start: Option<usize> = None;
            let mut nearby_gears: Vec<(usize, usize)> = Vec::new();

            for x in 0..grid[y].len() {
                if is_numeric(grid[y][x]) {
                    if start.is_none() {
                        start = Some(x);
                    }
                    for g in get_nearby_gears(x, y) {
                        if !nearby_gears.contains(&g) {
                            nearby_gears.push(g);
                        }
                    }
                } else {
                    if let Some(s) = start {
                        if !nearby_gears.is_empty() {
                            let num: i64 = std::str::from_utf8(&grid[y][s..x])
                                .unwrap()
                                .parse()
                                .unwrap();
                            for gear in &nearby_gears {
                                gear_dict.entry(*gear).or_default().push(num);
                            }
                        }
                    }
                    start = None;
                    nearby_gears = Vec::new();
                }
            }
        }

        let total: i64 = gear_dict
            .values()
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums[0] * nums[1])
            .sum();

        Some(total.to_string())
    }
}
