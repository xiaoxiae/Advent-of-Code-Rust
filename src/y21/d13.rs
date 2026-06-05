//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/13
use rustc_hash::FxHashSet;

use crate::util::Day;

pub struct D13;

fn parse(input: &str) -> (Vec<[i64; 2]>, Vec<(char, i64)>) {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();

    let dots = dots_str
        .lines()
        .map(|dot| {
            let (x, y) = dot.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();

    let folds = folds_str
        .lines()
        .filter(|l| l.starts_with("fold along "))
        .map(|fold| {
            // strip "fold along " prefix (11 chars), then split on '='
            let rest = &fold[11..];
            let (coord, val_str) = rest.split_once('=').unwrap();
            (coord.chars().next().unwrap(), val_str.parse().unwrap())
        })
        .collect();

    (dots, folds)
}

fn apply_fold(dots: &mut [[i64; 2]], coord: char, val: i64) {
    for dot in dots.iter_mut() {
        let (x, y) = (dot[0], dot[1]);
        if coord == 'x' && x > val {
            dot[0] -= (x - val) * 2;
        }
        if coord == 'y' && y > val {
            dot[1] -= (y - val) * 2;
        }
    }
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut dots, folds) = parse(input);

        let (coord, val) = folds[0];
        apply_fold(&mut dots, coord, val);

        let set: FxHashSet<(i64, i64)> = dots.iter().map(|d| (d[0], d[1])).collect();

        Some(set.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut dots, folds) = parse(input);

        for (coord, val) in folds {
            apply_fold(&mut dots, coord, val);
        }

        let minx = dots.iter().map(|d| d[0]).min().unwrap();
        let miny = dots.iter().map(|d| d[1]).min().unwrap();
        let maxx = dots.iter().map(|d| d[0]).max().unwrap();
        let maxy = dots.iter().map(|d| d[1]).max().unwrap();

        let set: FxHashSet<(i64, i64)> = dots.iter().map(|d| (d[0], d[1])).collect();

        let mut output = String::new();
        for y in miny..=maxy {
            for x in minx..=maxx {
                output.push(if set.contains(&(x, y)) { 'O' } else { ' ' });
            }
            output.push('\n');
        }
        // Trailing newline is removed to match printed output trimming.
        while output.ends_with('\n') {
            output.pop();
        }

        Some(output)
    }
}
