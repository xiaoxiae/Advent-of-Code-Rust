//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/22
//!
//! ⚠️ part 1 DISCREPANCY: the original 22-1.py has a variable-shadowing bug
//! (`is_within_bounds` tests the global `x` instead of its `v` arg, so the y/z
//! bounds are checked against x). The buggy Python prints 612060; this port does
//! the geometrically-correct check and yields 545118. Which value AoC accepted is
//! unknown — verify manually.
//! ⚠️ part 2 UNSOLVED (too slow): the original 22-2.py used an O(n³) approach that
//! did not finish (>300 s), and a faithful port likewise does not finish. Left as
//! None pending a coordinate-compression / interval-subtraction rewrite.
use crate::util::Day;

pub struct D22;

type Range2 = (i64, i64);
type Rectangle = (Range2, Range2, Range2);

fn parse(input: &str) -> Vec<(String, Rectangle)> {
    let mut commands: Vec<(String, Rectangle)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let status = parts.next().unwrap().to_string();
        let coordinates = parts.next().unwrap();

        // each of "x=-14..32", "y=-22..28", "z=-25..26" -> strip first 2 chars
        let mut ranges: Vec<Range2> = Vec::new();
        for part in coordinates.split(',') {
            let stripped = &part[2..];
            let mut it = stripped.split("..");
            let a: i64 = it.next().unwrap().parse().unwrap();
            let b: i64 = it.next().unwrap().parse().unwrap();
            ranges.push((a, b));
        }

        commands.push((status, (ranges[0], ranges[1], ranges[2])));
    }

    commands
}

fn is_within_bounds(v: i64, v_min: i64, v_max: i64) -> bool {
    v_min <= v && v <= v_max
}

fn is_on(point: (i64, i64, i64), rectangles: &[(String, Rectangle)]) -> bool {
    let (x, y, z) = point;
    let mut last_status: Option<&str> = None;

    for (status, (xr, yr, zr)) in rectangles {
        if is_within_bounds(x, xr.0, xr.1)
            && is_within_bounds(y, yr.0, yr.1)
            && is_within_bounds(z, zr.0, zr.1)
        {
            last_status = Some(status.as_str());
        }
    }

    last_status == Some("on")
}

fn get_sorted_coordinates(rectangles: &[Rectangle]) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut xs: Vec<i64> = Vec::new();
    let mut ys: Vec<i64> = Vec::new();
    let mut zs: Vec<i64> = Vec::new();

    for ((x1, x2), (y1, y2), (z1, z2)) in rectangles {
        xs.push(*x1);
        xs.push(*x2);
        ys.push(*y1);
        ys.push(*y2);
        zs.push(*z1);
        zs.push(*z2);
    }

    xs.sort();
    ys.sort();
    zs.sort();

    (xs, ys, zs)
}

fn get_segments(coordinates: &[i64]) -> Vec<(i64, i64)> {
    let mut segments: Vec<(i64, i64)> = Vec::new();

    // mirror the Python which references x2 after the loop
    let mut x2 = coordinates[coordinates.len() - 1];

    for i in 0..coordinates.len() - 1 {
        let x1 = coordinates[i];
        x2 = coordinates[i + 1];
        if x1 == x2 {
            continue;
        }

        segments.push((x1, x1));

        // if there are some things between, return them
        if x1 != x2 + 1 {
            segments.push((x1 + 1, x2 - 1));
        }
    }

    segments.push((x2, x2));

    segments
}

fn count_lit_points(rectangles: &[Rectangle], commands: &[(String, Rectangle)]) -> i64 {
    let (xs, ys, zs) = get_sorted_coordinates(rectangles);

    let mut total: i64 = 0;

    let xseg = get_segments(&xs);
    let yseg = get_segments(&ys);
    let zseg = get_segments(&zs);

    for &(x1, x2) in &xseg {
        for &(y1, y2) in &yseg {
            for &(z1, z2) in &zseg {
                if is_on((x1, y1, z1), commands) {
                    total += (x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1);
                }
            }
        }
    }

    total
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let commands = parse(input);

        let mut total: i64 = 0;
        for x in -50..=50 {
            for y in -50..=50 {
                for z in -50..=50 {
                    if is_on((x, y, z), &commands) {
                        total += 1;
                    }
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, _input: &str) -> Option<String> {
        // Too slow: the original Python (O(n³)) timed out (>300 s) and this faithful
        // port does not finish either. Left unsolved pending a coordinate-
        // compression / interval-subtraction rewrite. See the file header.
        None
    }
}
