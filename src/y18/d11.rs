//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/11
use crate::util::Day;

pub struct D11;

/// Hundreds digit of the power computation, minus 5.
///
/// Mirrors the Python `int(str(((x + 10) * y + serial) * (x + 10))[-3]) - 5`.
/// For all coordinates here the product is always at least five digits, so the
/// `[-3]` index is the hundreds digit.
fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let n = (rack_id * y + serial) * rack_id;
    (n / 100) % 10 - 5
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let serial: i64 = input.lines().next().unwrap().trim().parse().unwrap();

        let mut board = vec![vec![0i64; 300]; 300];

        // calculate all of the power levels
        for y in 0..300 {
            for x in 0..300 {
                board[y as usize][x as usize] = power_level(x, y, serial);
            }
        }

        // calculate sums of all 3x3 parts of the grid
        let mut max_sum: i64 = 0;
        let mut max_coords = (0i64, 0i64);
        for y in 0..298 {
            for x in 0..298 {
                let mut sum = 0i64;

                // find the sum of the 3x3
                for i in 0..3 {
                    for j in 0..3 {
                        sum += board[(y + i) as usize][(x + j) as usize];
                    }
                }

                // if sum is larger than the current largest
                if max_sum < sum {
                    max_sum = sum;
                    max_coords = (x, y);
                }
            }
        }

        Some(format!("{},{}", max_coords.0, max_coords.1))
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let serial: i64 = input.lines().next().unwrap().trim().parse().unwrap();

        let size: i64 = 300;

        let bound = |x: i64, y: i64| -> bool { x >= 0 && y >= 0 && x < size && y < size };

        let mut board = vec![vec![0i64; size as usize]; size as usize];
        let mut area_table = vec![vec![0i64; size as usize]; size as usize];

        // calculate all of the power levels and the summed area table
        for y in 0..size {
            for x in 0..size {
                let (xi, yi) = (x as usize, y as usize);
                board[yi][xi] = power_level(x, y, serial);

                // the value of the summed area table
                let up = if bound(x, y - 1) {
                    area_table[(y - 1) as usize][xi]
                } else {
                    0
                };
                let left = if bound(x - 1, y) {
                    area_table[yi][(x - 1) as usize]
                } else {
                    0
                };
                let diag = if bound(x - 1, y - 1) {
                    area_table[(y - 1) as usize][(x - 1) as usize]
                } else {
                    0
                };
                area_table[yi][xi] = board[yi][xi] + up + left - diag;
            }
        }

        // go through all k x k grids
        let mut max_sum: i64 = 0;
        let mut result = String::new();
        for k in 3..300 {
            // calculate all k x k areas in the grid
            for y in 1..(size - k) {
                for x in 1..(size - k) {
                    let (xu, yu) = (x as usize, y as usize);
                    let sum = area_table[(y + k) as usize][(x + k) as usize]
                        + area_table[yu][xu]
                        - area_table[(y + k) as usize][xu]
                        - area_table[yu][(x + k) as usize];

                    // if the sum is bigger than the previous max sum
                    if sum > max_sum {
                        max_sum = sum;
                        result = format!("{},{},{}", x + 1, y + 1, k);
                    }
                }
            }
        }

        Some(result)
    }
}
