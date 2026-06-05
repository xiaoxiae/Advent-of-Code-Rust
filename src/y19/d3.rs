//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/03
use crate::util::Day;

pub struct D3;

/// Return a list of (x, y) points representing the path of the wire.
fn trace_wire(instructions: &str) -> Vec<(i64, i64)> {
    let (mut x, mut y) = (0i64, 0i64);

    let mut path = vec![(x, y)];
    for instruction in instructions.split(',') {
        let dir = instruction.as_bytes()[0];
        let amount: i64 = instruction[1..].parse().unwrap();

        match dir {
            b'R' => x += amount,
            b'L' => x -= amount,
            b'U' => y += amount,
            b'D' => y -= amount,
            _ => {}
        }

        path.push((x, y));
    }

    path
}

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim().lines().collect();

        let p1 = trace_wire(lines[0]);
        let p2 = trace_wire(lines[1]);

        let mut min_d = i64::MAX;

        for i in 0..p1.len() - 1 {
            // optimization
            let a = p1[i].0.abs() + p1[i].1.abs();
            let b = p1[i + 1].0.abs() + p1[i + 1].1.abs();
            if a.max(b) > min_d {
                continue;
            }

            for j in i..p2.len() - 1 {
                // optimization
                let c = p2[j].0.abs() + p2[j].1.abs();
                let d_ = p2[j + 1].0.abs() + p2[j + 1].1.abs();
                if c.max(d_) > min_d {
                    continue;
                }

                // check for intersections
                let (p1s, p1e) = (p1[i], p1[i + 1]);
                let (p2s, p2e) = (p2[j], p2[j + 1]);

                let (mut x, mut y) = (p1s.0, p1s.1);
                while (x, y) != p1e {
                    let on_seg = (x == p2s.0
                        && x == p2e.0
                        && p2s.1.min(p2e.1) <= y
                        && y <= p2s.1.max(p2e.1))
                        || (y == p2s.1
                            && y == p2e.1
                            && p2s.0.min(p2e.0) <= x
                            && x <= p2s.0.max(p2e.0));

                    if on_seg && (x, y) != (0, 0) {
                        let d = x.abs() + y.abs();
                        if d < min_d {
                            min_d = d;
                        }
                    }

                    x += (p1e.0 - x).signum();
                    y += (p1e.1 - y).signum();
                }
            }
        }

        Some(min_d.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim().lines().collect();

        let p1 = trace_wire(lines[0]);
        let p2 = trace_wire(lines[1]);

        let mut p1_sum = 0i64;
        let mut min_d = i64::MAX;

        for i in 0..p1.len() - 1 {
            // sum the wire along the way
            if i > 0 {
                p1_sum += (p1[i].0 - p1[i - 1].0).abs() + (p1[i].1 - p1[i - 1].1).abs();
            }

            let mut p2_sum = 0i64;
            for j in 0..p2.len() - 1 {
                if j > 0 {
                    p2_sum += (p2[j].0 - p2[j - 1].0).abs() + (p2[j].1 - p2[j - 1].1).abs();
                }

                // optimization
                if p1_sum + p2_sum > min_d {
                    break;
                }

                // check for intersections
                let (p1s, p1e) = (p1[i], p1[i + 1]);
                let (p2s, p2e) = (p2[j], p2[j + 1]);

                let (mut x, mut y) = (p1s.0, p1s.1);
                while (x, y) != p1e {
                    let on_seg = (x == p2s.0
                        && x == p2e.0
                        && p2s.1.min(p2e.1) <= y
                        && y <= p2s.1.max(p2e.1))
                        || (y == p2s.1
                            && y == p2e.1
                            && p2s.0.min(p2e.0) <= x
                            && x <= p2s.0.max(p2e.0));

                    if on_seg && (x, y) != (0, 0) {
                        let d1 = (p1s.0 - x).abs() + (p1s.1 - y).abs();
                        let d2 = (p2s.0 - x).abs() + (p2s.1 - y).abs();
                        if d1 + p1_sum + d2 + p2_sum < min_d {
                            min_d = d1 + p1_sum + d2 + p2_sum;
                        }
                    }

                    x += (p1e.0 - x).signum();
                    y += (p1e.1 - y).signum();
                }
            }
        }

        Some(min_d.to_string())
    }
}
