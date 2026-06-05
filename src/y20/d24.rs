//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/24
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D24;

const DIRECTIONS: [(i32, i32); 6] = [
    (2, 0),   // e
    (1, -1),  // se
    (-1, -1), // sw
    (-2, 0),  // w
    (-1, 1),  // nw
    (1, 1),   // ne
];

fn direction(s: &str) -> (i32, i32) {
    match s {
        "e" => (2, 0),
        "se" => (1, -1),
        "sw" => (-1, -1),
        "w" => (-2, 0),
        "nw" => (-1, 1),
        "ne" => (1, 1),
        _ => unreachable!(),
    }
}

fn initial_tiles(input: &str) -> FxHashSet<(i32, i32)> {
    let mut tiles: FxHashSet<(i32, i32)> = FxHashSet::default();

    for instruction in input.trim().lines() {
        let bytes = instruction.as_bytes();
        let mut x = 0;
        let mut y = 0;

        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            let (dx, dy) = if c == 'e' || c == 'w' {
                let d = direction(&instruction[i..i + 1]);
                i += 1;
                d
            } else {
                let d = direction(&instruction[i..i + 2]);
                i += 2;
                d
            };

            x += dx;
            y += dy;
        }

        if !tiles.remove(&(x, y)) {
            tiles.insert((x, y));
        }
    }

    tiles
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let tiles = initial_tiles(input);
        Some(tiles.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut tiles = initial_tiles(input);

        for _ in 0..100 {
            let mut possible_tiles: FxHashSet<(i32, i32)> = FxHashSet::default();
            for &(x, y) in &tiles {
                possible_tiles.insert((x, y));
                for (dx, dy) in DIRECTIONS {
                    possible_tiles.insert((x + dx, y + dy));
                }
            }

            let mut new_tiles: FxHashSet<(i32, i32)> = FxHashSet::default();
            for &(x, y) in &possible_tiles {
                let mut neighbours = 0;
                for (dx, dy) in DIRECTIONS {
                    if tiles.contains(&(x + dx, y + dy)) {
                        neighbours += 1;
                    }
                }

                if tiles.contains(&(x, y)) {
                    if neighbours > 0 && neighbours <= 2 {
                        new_tiles.insert((x, y));
                    }
                } else if neighbours == 2 {
                    new_tiles.insert((x, y));
                }
            }

            tiles = new_tiles;
        }

        Some(tiles.len().to_string())
    }
}
