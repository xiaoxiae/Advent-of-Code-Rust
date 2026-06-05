//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/18
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D18;

fn delta(dir: &str) -> (i64, i64) {
    match dir {
        "R" => (1, 0),
        "D" => (0, -1),
        "L" => (-1, 0),
        "U" => (0, 1),
        _ => unreachable!(),
    }
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut trench: FxHashSet<(i64, i64)> = FxHashSet::default();
        trench.insert((x, y));

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            let dir = parts.next().unwrap();
            let d: i64 = parts.next().unwrap().parse().unwrap();

            let (dx, dy) = delta(dir);
            for _ in 0..d {
                x += dx;
                y += dy;
                trench.insert((x, y));
            }
        }

        let deltas: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

        let mut queue: Vec<(i64, i64)> = vec![(1, -1)];
        while let Some((x, y)) = queue.pop() {
            for (dx, dy) in deltas.iter() {
                let nx = x + dx;
                let ny = y + dy;

                if trench.contains(&(nx, ny)) {
                    continue;
                }

                trench.insert((nx, ny));
                queue.push((nx, ny));
            }
        }

        Some(trench.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let delta_order = ["R", "D", "L", "U"];
        let mut prev_dir: Option<&str> = None;

        let mut points: Vec<(i64, i64)> = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // hex = line.split()[-1][2:-1]
            let last = line.split_whitespace().last().unwrap();
            let hex = &last[2..last.len() - 1];
            let last_char = hex.chars().last().unwrap();
            let dir = delta_order[last_char.to_digit(16).unwrap() as usize];
            let d = i64::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();

            match (prev_dir, dir) {
                (Some("R"), "D") => x += 1,
                (Some("D"), "L") => y -= 1,
                (Some("L"), "U") => x -= 1,
                (Some("U"), "R") => y += 1,
                _ => {}
            }

            points.push((x, y));

            let (dx, dy) = delta(dir);
            x += dx * d;
            y += dy * d;

            match (prev_dir, dir) {
                (Some("R"), "U") => y -= 1,
                (Some("U"), "L") => x += 1,
                (Some("L"), "D") => y += 1,
                (Some("D"), "R") => x -= 1,
                _ => {}
            }

            prev_dir = Some(dir);
        }

        points.push((x, y));

        let mut total: i64 = 0;
        for i in 0..points.len() {
            let (x1, y1) = if i == 0 {
                points[points.len() - 1]
            } else {
                points[i - 1]
            };
            let (x2, y2) = points[i];

            if y1 == y2 {
                total += (x1 - x2) * y1;
            }
        }

        Some(total.abs().to_string())
    }
}
