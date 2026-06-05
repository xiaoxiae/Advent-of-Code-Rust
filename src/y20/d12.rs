//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/12
use crate::util::Day;

pub struct D12;

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let delta: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut d: i64 = 0;
        let mut x: i64 = 0;
        let mut y: i64 = 0;

        for inst in input.trim().lines() {
            let c = inst.chars().next().unwrap();
            let by: i64 = inst[1..].trim().parse().unwrap();

            match c {
                'E' => x += by,
                'N' => y += by,
                'S' => y -= by,
                'W' => x -= by,
                'R' => {
                    let by = by / 90;
                    d = (d - by).rem_euclid(4);
                }
                'L' => {
                    let by = by / 90;
                    d = (d + by).rem_euclid(4);
                }
                'F' => {
                    let di = d as usize;
                    x += delta[di].0 * by;
                    y += delta[di].1 * by;
                }
                _ => {}
            }
        }

        Some((x.abs() + y.abs()).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut xw: i64 = 10;
        let mut yw: i64 = 1;
        let mut xs: i64 = 0;
        let mut ys: i64 = 0;

        for inst in input.trim().lines() {
            let c = inst.chars().next().unwrap();
            let by: i64 = inst[1..].trim().parse().unwrap();

            match c {
                'E' => xw += by,
                'N' => yw += by,
                'S' => yw -= by,
                'W' => xw -= by,
                'R' => {
                    for _ in 0..(by / 90) {
                        (xw, yw) = (yw, -xw);
                    }
                }
                'L' => {
                    for _ in 0..(by / 90) {
                        (xw, yw) = (-yw, xw);
                    }
                }
                'F' => {
                    xs += xw * by;
                    ys += yw * by;
                }
                _ => {}
            }
        }

        Some((xs.abs() + ys.abs()).to_string())
    }
}
