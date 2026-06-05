//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/17
use crate::util::Day;

pub struct D17;

fn parse(input: &str) -> (i64, i64, i64, i64) {
    let line = input.lines().next().unwrap().trim();
    // line: "target area: x=25..67, y=-260..-200"
    let rest = &line[13..];
    let (p1, p2) = rest.split_once(", ").unwrap();

    let parse_range = |s: &str| -> (i64, i64) {
        let s = &s[2..]; // skip "x=" / "y="
        let (a, b) = s.split_once("..").unwrap();
        (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
    };

    let (x1, x2) = parse_range(p1);
    let (y1, y2) = parse_range(p2);
    (x1, x2, y1, y2)
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (_x1, _x2, y1, y2) = parse(input);

        let v = y1.min(y2).abs();
        let max_y = v * (v - 1) / 2;
        Some(max_y.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (x1, x2, y1, y2) = parse(input);

        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));

        let is_within_range = |mut xv: i64, mut yv: i64| -> bool {
            let mut x = 0i64;
            let mut y = 0i64;

            while x <= x2 && y >= y1 {
                if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                    return true;
                }

                x += xv.max(0);
                y += yv;

                yv -= 1;
                xv -= 1;
            }

            false
        };

        let mut total = 0;
        for xv in 0..=x2 {
            for yv in -y1.abs()..y1.abs() {
                if is_within_range(xv, yv) {
                    total += 1;
                }
            }
        }

        Some(total.to_string())
    }
}
