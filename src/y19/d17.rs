//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/17
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

pub struct D17;

/// Build the area map by running the program and collecting its ASCII output.
/// Mirrors the Python loop that parses outputs into a `(x, y) -> char` grid.
fn build_area(input: &str) -> FxHashMap<(i64, i64), char> {
    let mut c = Intcode::from_input(input);
    let mut area: FxHashMap<(i64, i64), char> = FxHashMap::default();

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    loop {
        match c.run() {
            Step::Output(ch) => {
                let ch = ch as u8 as char;
                if ch == '\n' {
                    x = 0;
                    y += 1;
                } else {
                    area.insert((x, y), ch);
                    x += 1;
                }
            }
            // The original Python loops until `run` returns None (program done).
            // With mem[0]=2 it instead reaches an input prompt; either way the
            // full map has already been emitted, so we stop here.
            Step::NeedInput | Step::Halt => break,
        }
    }

    area
}

/// Look up a cell, defaulting to '.' (matches the Python `Area.__getitem__`).
fn get(area: &FxHashMap<(i64, i64), char>, p: (i64, i64)) -> char {
    *area.get(&p).unwrap_or(&'.')
}

/// Sum of alignment parameters (x*y) at scaffold intersections.
fn get_intersections(area: &FxHashMap<(i64, i64), char>) -> i64 {
    let mut total = 0;
    for (&point, &ch) in area.iter() {
        if ch == '#' {
            let dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
            let all = dirs
                .iter()
                .all(|&(dx, dy)| get(area, (point.0 + dx, point.1 + dy)) == '#');
            if all {
                total += point.0 * point.1;
            }
        }
    }
    total
}

const DIRECTIONS: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

/// Orientation index for the robot glyph (matches Python `orientation` dict).
fn orientation_of(ch: char) -> Option<i64> {
    match ch {
        '>' => Some(0),
        '^' => Some(1),
        '<' => Some(2),
        'v' => Some(3),
        _ => None,
    }
}

fn next_direction(
    area: &FxHashMap<(i64, i64), char>,
    x: i64,
    y: i64,
    visited: &FxHashSet<(i64, i64)>,
) -> Option<i64> {
    for (i, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
        let point = (x + dx, y + dy);
        if !visited.contains(&point) && get(area, point) == '#' {
            return Some(i as i64);
        }
    }
    None
}

/// A path token: either a turn/move string ("L", "RR", ...) or a numeric length.
#[derive(Clone, PartialEq, Eq)]
enum Token {
    Str(String),
    Num(i64),
}

impl Token {
    fn to_display(&self) -> String {
        match self {
            Token::Str(s) => s.clone(),
            Token::Num(n) => n.to_string(),
        }
    }
}

/// Walk the scaffold greedily, emitting the full L/R + move-length path.
fn get_path(area: &FxHashMap<(i64, i64), char>) -> Vec<Token> {
    // Find the robot position/orientation (first matching glyph).
    let mut robot_position = None;
    let mut robot_orientation = 0i64;

    // Match Python iteration: insertion order of the map. FxHashMap is not
    // ordered, but the robot glyph is unique, so the result is deterministic.
    for (&point, &ch) in area.iter() {
        if let Some(o) = orientation_of(ch) {
            robot_position = Some(point);
            robot_orientation = o;
            break;
        }
    }

    let point = robot_position.expect("no robot found");
    let mut visited: FxHashSet<(i64, i64)> = FxHashSet::default();
    visited.insert(point);

    let mut path: Vec<Token> = Vec::new();
    let (mut x, mut y) = point;

    loop {
        let o = match next_direction(area, x, y, &visited) {
            Some(o) => o,
            None => break,
        };

        // adjust the robot orientation
        let mut turn_delta = robot_orientation - o;
        if turn_delta >= 3 {
            turn_delta += -4;
        } else if turn_delta <= -3 {
            turn_delta += 4;
        }
        let token_str = if turn_delta < 0 {
            "L".repeat((-turn_delta) as usize)
        } else {
            "R".repeat(turn_delta as usize)
        };
        path.push(Token::Str(token_str));
        robot_orientation = o;

        // move as much as we can in the given direction
        let (dx, dy) = DIRECTIONS[o as usize];
        let mut move_length = 0i64;
        while get(area, (x + dx, y + dy)) != '.' {
            move_length += 1;
            x += dx;
            y += dy;
            visited.insert((x, y));
        }
        path.push(Token::Num(move_length));
    }

    path
}

/// Generate possible replacements of the path: for each prefix length 2..=10,
/// yield (remaining path with all occurrences of the prefix removed, prefix).
fn get_replacements(path: &[Token]) -> Vec<(Vec<Token>, Vec<Token>)> {
    let mut out = Vec::new();
    for i in 2..11 {
        if i > path.len() {
            break;
        }
        let r = path[..i].to_vec();
        let mut p = path.to_vec();

        let mut j = 0usize;
        while j + r.len() <= p.len() {
            if p[j..j + r.len()] == r[..] {
                let mut new_p = p[..j].to_vec();
                new_p.extend_from_slice(&p[j + r.len()..]);
                p = new_p;
            } else {
                j += 1;
            }
        }

        out.push((p, r));
    }
    out
}

/// Find the A/B/C routine decomposition and build the ASCII instruction stream.
fn get_instructions(path: &[Token]) -> Vec<i64> {
    for (p1, r1) in get_replacements(path) {
        for (p2, r2) in get_replacements(&p1) {
            for (p3, r3) in get_replacements(&p2) {
                if p3.is_empty() {
                    let registers = [r1.clone(), r2.clone(), r3.clone()];

                    // Substitute A/B/C names into a fresh copy of the full path.
                    let mut main_path: Vec<Token> = path.to_vec();
                    let mut i = 0usize;
                    while i != main_path.len() {
                        for (j, r) in registers.iter().enumerate() {
                            if i + r.len() <= main_path.len()
                                && main_path[i..i + r.len()] == r[..]
                            {
                                let name =
                                    ((b'A' + j as u8) as char).to_string();
                                let mut new_path = main_path[..i].to_vec();
                                new_path.push(Token::Str(name));
                                new_path.extend_from_slice(&main_path[i + r.len()..]);
                                main_path = new_path;
                            }
                        }
                        i += 1;
                    }

                    let mut instructions: Vec<i64> = Vec::new();
                    let pieces = [&main_path, &r1, &r2, &r3];
                    for piece in pieces {
                        let joined = piece
                            .iter()
                            .map(|t| t.to_display())
                            .collect::<Vec<_>>()
                            .join(",");
                        for ch in joined.bytes() {
                            instructions.push(ch as i64);
                        }
                        instructions.push(10);
                    }

                    instructions.push(b'n' as i64);
                    instructions.push(10);

                    return instructions;
                }
            }
        }
    }
    panic!("no decomposition found");
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let area = build_area(input);
        Some(get_intersections(&area).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let area = build_area(input);
        let path = get_path(&area);
        let instructions = get_instructions(&path);

        let mut c = Intcode::from_input(input);
        c.set(0, 2);
        c.input_all(instructions);

        let mut prev_out: i64 = 0;
        loop {
            match c.run() {
                Step::Output(v) => prev_out = v,
                Step::NeedInput => panic!("unexpected input request"),
                Step::Halt => break,
            }
        }

        Some(prev_out.to_string())
    }
}
