//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/10
use crate::util::Day;
use regex::Regex;

pub struct D10;

fn parse(input: &str) -> Vec<[i64; 4]> {
    let re = Regex::new(r"-*[0-9]+").unwrap();
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let nums: Vec<i64> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            [nums[0], nums[1], nums[2], nums[3]]
        })
        .collect()
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // faithful translation of 10-1.py
        let mut points = parse(input);

        let mut min_x_delta: i64 = i64::MAX;

        loop {
            let (mut x_min, mut x_max) = (i64::MAX, i64::MIN);
            let (mut y_min, mut y_max) = (i64::MAX, i64::MIN);

            for point in points.iter_mut() {
                point[0] += point[2];
                point[1] += point[3];

                x_min = x_min.min(point[0]);
                x_max = x_max.max(point[0]);
                y_min = y_min.min(point[1]);
                y_max = y_max.max(point[1]);
            }

            if x_max - x_min < min_x_delta {
                min_x_delta = x_max - x_min;
            } else {
                // move the points back 1 step (that was the minimal delta)
                let (mut bx_min, mut bx_max) = (i64::MAX, i64::MIN);
                let (mut by_min, mut by_max) = (i64::MAX, i64::MIN);

                for point in points.iter_mut() {
                    point[0] -= point[2];
                    point[1] -= point[3];

                    bx_min = bx_min.min(point[0]);
                    bx_max = bx_max.max(point[0]);
                    by_min = by_min.min(point[1]);
                    by_max = by_max.max(point[1]);
                }

                // build the board to display the message on
                let width = (min_x_delta + 1) as usize;
                let height = (by_max - by_min + 1) as usize;
                let mut board = vec![vec![0u8; width]; height];

                for point in &points {
                    let r = (point[1] - by_min) as usize;
                    let c = (point[0] - bx_min) as usize;
                    board[r][c] = 1;
                }

                // prettyprint the board
                let mut out = String::new();
                for row in &board {
                    for &num in row {
                        if num == 0 {
                            out.push(' ');
                        } else {
                            out.push('*');
                        }
                    }
                    out.push('\n');
                }
                // drop trailing newline to match a trimmed multi-line message
                while out.ends_with('\n') {
                    out.pop();
                }
                return Some(out);
            }
        }
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // faithful translation of 10-2.py
        let mut points = parse(input);

        let mut min_x_delta: i64 = i64::MAX;
        let mut counter: i64 = 0;

        loop {
            let (mut x_min, mut x_max) = (i64::MAX, i64::MIN);

            for point in points.iter_mut() {
                point[0] += point[2];
                point[1] += point[3];

                x_min = x_min.min(point[0]);
                x_max = x_max.max(point[0]);
            }

            counter += 1;

            if x_max - x_min < min_x_delta {
                min_x_delta = x_max - x_min;
            } else {
                return Some((counter - 1).to_string());
            }
        }
    }
}
