//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/11
use crate::util::Day;

pub struct D11;

const DIRS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn occupied_adjacent(lines: &[Vec<char>], x: isize, y: isize) -> usize {
    let h = lines.len() as isize;
    let w = lines[0].len() as isize;
    let mut occupied = 0;
    for (xd, yd) in DIRS.iter() {
        let xn = x + xd;
        let yn = y + yd;
        if !(0 <= xn && xn < w && 0 <= yn && yn < h) {
            continue;
        }
        if lines[yn as usize][xn as usize] == '#' {
            occupied += 1;
        }
    }
    occupied
}

fn occupied_visible(lines: &[Vec<char>], x: isize, y: isize) -> usize {
    let h = lines.len() as isize;
    let w = lines[0].len() as isize;
    let mut occupied = 0;
    for (xd, yd) in DIRS.iter() {
        let mut xn = x + xd;
        let mut yn = y + yd;
        while 0 <= xn && xn < w && 0 <= yn && yn < h {
            let c = lines[yn as usize][xn as usize];
            if c == 'L' {
                break;
            }
            if c == '#' {
                occupied += 1;
                break;
            }
            xn += xd;
            yn += yd;
        }
    }
    occupied
}

fn simulate(
    mut lines: Vec<Vec<char>>,
    count_fn: fn(&[Vec<char>], isize, isize) -> usize,
    threshold: usize,
) -> usize {
    loop {
        let mut new_lines = lines.clone();
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                let occupied = count_fn(&lines, x as isize, y as isize);
                if lines[y][x] == 'L' && occupied == 0 {
                    new_lines[y][x] = '#';
                }
                if lines[y][x] == '#' && occupied >= threshold {
                    new_lines[y][x] = 'L';
                }
            }
        }
        if lines == new_lines {
            break;
        }
        lines = new_lines;
    }

    lines
        .iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count()
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines = parse(input);
        Some(simulate(lines, occupied_adjacent, 4).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines = parse(input);
        Some(simulate(lines, occupied_visible, 5).to_string())
    }
}
