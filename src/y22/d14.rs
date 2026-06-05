//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/14
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D14;

fn parse(a: &str) -> (i64, i64) {
    let mut it = a.split(',').map(|x| x.trim().parse::<i64>().unwrap());
    let x = it.next().unwrap();
    let y = it.next().unwrap();
    (x, y)
}

fn return_points_between(a: &str, b: &str) -> Vec<(i64, i64)> {
    let (mut x1, mut y1) = parse(a);
    let (x2, y2) = parse(b);

    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    let mut coords = vec![(x1, y1)];
    while (x1, y1) != (x2, y2) {
        x1 += dx;
        y1 += dy;
        coords.push((x1, y1));
    }

    coords
}

fn build_cave(input: &str) -> (FxHashMap<(i64, i64), char>, i64) {
    let mut cave: FxHashMap<(i64, i64), char> = FxHashMap::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let coords: Vec<&str> = line.split(" -> ").collect();
        for i in 0..coords.len() - 1 {
            for p in return_points_between(coords[i], coords[i + 1]) {
                cave.insert(p, '#');
            }
        }
    }

    let max_y = cave.keys().map(|&(_, y)| y).max().unwrap();
    (cave, max_y)
}

/// Part 1: returns Some(resting position) or None if it falls into the abyss.
fn simulate_fall_1(cave: &FxHashMap<(i64, i64), char>, max_y: i64) -> Option<(i64, i64)> {
    let (mut s_x, mut s_y) = (500i64, 0i64);

    loop {
        if !cave.contains_key(&(s_x, s_y + 1)) {
            s_y += 1;
        } else if !cave.contains_key(&(s_x - 1, s_y + 1)) {
            s_y += 1;
            s_x -= 1;
        } else if !cave.contains_key(&(s_x + 1, s_y + 1)) {
            s_y += 1;
            s_x += 1;
        } else {
            return Some((s_x, s_y));
        }

        if s_y > max_y {
            return None;
        }
    }
}

/// Part 2: floor at max_y + 2 (rests at max_y + 1 if nothing else stops it).
fn simulate_fall_2(cave: &FxHashMap<(i64, i64), char>, max_y: i64) -> (i64, i64) {
    let (mut s_x, mut s_y) = (500i64, 0i64);

    loop {
        if !cave.contains_key(&(s_x, s_y + 1)) {
            s_y += 1;
        } else if !cave.contains_key(&(s_x - 1, s_y + 1)) {
            s_y += 1;
            s_x -= 1;
        } else if !cave.contains_key(&(s_x + 1, s_y + 1)) {
            s_y += 1;
            s_x += 1;
        } else {
            return (s_x, s_y);
        }

        if s_y == max_y + 1 {
            return (s_x, s_y);
        }
    }
}

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut cave, max_y) = build_cave(input);

        let mut sand = 0i64;
        while let Some(coord) = simulate_fall_1(&cave, max_y) {
            cave.insert(coord, 'o');
            sand += 1;
        }

        Some(sand.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut cave, max_y) = build_cave(input);

        let mut sand = 0i64;
        loop {
            let coord = simulate_fall_2(&cave, max_y);
            cave.insert(coord, 'o');
            sand += 1;

            if coord.1 == 0 {
                break;
            }
        }

        Some(sand.to_string())
    }
}
