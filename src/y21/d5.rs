//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/05
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D5;

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let a = parts.next().unwrap();
            let b = parts.next().unwrap();
            let parse_point = |s: &str| {
                let mut it = s.split(',');
                let x: i64 = it.next().unwrap().trim().parse().unwrap();
                let y: i64 = it.next().unwrap().trim().parse().unwrap();
                (x, y)
            };
            (parse_point(a), parse_point(b))
        })
        .collect()
}

/// Part 1 logic: only horizontal/vertical lines.
fn get_pipe_points_straight(x1: i64, y1: i64, x2: i64, y2: i64) -> Vec<(i64, i64)> {
    let (x1, x2) = (x1.min(x2), x1.max(x2));
    let (y1, y2) = (y1.min(y2), y1.max(y2));

    let mut points = Vec::new();

    if x1 == x2 {
        for i in y1..=y2 {
            points.push((x1, i));
        }
    } else if y1 == y2 {
        for i in x1..=x2 {
            points.push((i, y1));
        }
    }

    points
}

/// Part 2 logic: includes diagonals.
fn get_pipe_points(x1: i64, y1: i64, x2: i64, y2: i64) -> Vec<(i64, i64)> {
    let (mut sx1, mut sy1, mut sx2, mut sy2) = (x1, y1, x2, y2);

    if x1 == x2 || y1 == y2 {
        sx1 = x1.min(x2);
        sx2 = x1.max(x2);
        sy1 = y1.min(y2);
        sy2 = y1.max(y2);
    }

    let mut points = Vec::new();

    if sx1 == sx2 {
        for i in sy1..=sy2 {
            points.push((sx1, i));
        }
    } else if sy1 == sy2 {
        for i in sx1..=sx2 {
            points.push((i, sy1));
        }
    } else {
        let dx = (sx2 - sx1).signum();
        let dy = (sy2 - sy1).signum();

        let mut x = sx1;
        let mut i = 0;
        while x != sx2 + dx {
            points.push((x, sy1 + i * dy));
            x += dx;
            i += 1;
        }
    }

    points
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines = parse(input);

        let mut point_counts: FxHashMap<(i64, i64), i64> = FxHashMap::default();
        for ((x1, y1), (x2, y2)) in lines {
            if x1 == x2 || y1 == y2 {
                for point in get_pipe_points_straight(x1, y1, x2, y2) {
                    *point_counts.entry(point).or_insert(0) += 1;
                }
            }
        }

        let total = point_counts.values().filter(|&&c| c >= 2).count();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines = parse(input);

        let mut point_counts: FxHashMap<(i64, i64), i64> = FxHashMap::default();
        for ((x1, y1), (x2, y2)) in lines {
            for point in get_pipe_points(x1, y1, x2, y2) {
                *point_counts.entry(point).or_insert(0) += 1;
            }
        }

        let total = point_counts.values().filter(|&&c| c >= 2).count();
        Some(total.to_string())
    }
}
