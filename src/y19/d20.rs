//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/20
//!
//! ⚠️ SLOW / UNVERIFIED: this port does not finish within 60s, and the original
//! Python produced no answer either. Flagged for manual review (Donut Maze).
use crate::util::Day;
use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

/// A cell of the maze. Mirrors the original Python where `area[y][x]` is either a
/// character or, once two same-labelled portals have been paired up, a tuple
/// describing the teleport destination.
#[derive(Clone)]
enum Cell {
    /// A raw character from the input (`'.'`, `' '`, a letter, `'#'`, ...).
    Char(char),
    /// Part 1 teleport target: `(x, y)`.
    Portal(i32, i32),
    /// Part 2 teleport target: `(x, y, inner)`.
    PortalDepth(i32, i32, bool),
}

impl Cell {
    fn ch(&self) -> Option<char> {
        match self {
            Cell::Char(c) => Some(*c),
            _ => None,
        }
    }
}

/// Parse the input into a grid of characters, preserving the exact (possibly
/// ragged) row lengths produced by Python's `read().splitlines()`.
fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .map(|l| l.chars().map(Cell::Char).collect())
        .collect()
}

/// Equivalent of the Python `get_portal`: looks at the four neighbours of the dot
/// `(x, y)`; if one is an uppercase letter it reads the two-letter label (sorted)
/// and returns `(label, (letter_x, letter_y))`.
fn get_portal(area: &[Vec<Cell>], x: i32, y: i32) -> Option<(String, (i32, i32))> {
    for (i, d) in DIRECTIONS.iter().enumerate() {
        let (x_n, y_n) = (x + d.0, y + d.1);

        if y_n < 0 || y_n as usize >= area.len() || x_n < 0 || x_n as usize >= area[y_n as usize].len()
        {
            continue;
        }

        if let Some(c1) = area[y_n as usize][x_n as usize].ch() {
            if ('A'..='Z').contains(&c1) {
                // second letter is one further in the same direction
                let x2 = x_n + DIRECTIONS[i].0;
                let y2 = y_n + DIRECTIONS[i].1;
                let c2 = area[y2 as usize][x2 as usize].ch().unwrap();

                let mut chars = [c1, c2];
                chars.sort_unstable();
                let label: String = chars.iter().collect();
                return Some((label, (x_n, y_n)));
            }
        }
    }
    None
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut area = parse(input);

        let mut portals: FxHashMap<String, ((i32, i32), (i32, i32))> = FxHashMap::default();
        let mut start: Option<(i32, i32)> = None;
        let mut end: Option<(i32, i32)> = None;

        for y in 0..area.len() {
            for x in 0..area[y].len() {
                if matches!(area[y][x], Cell::Char('.')) {
                    if let Some((label, pos)) = get_portal(&area, x as i32, y as i32) {
                        if label == "AA" {
                            start = Some((x as i32, y as i32));
                        } else if label == "ZZ" {
                            end = Some((x as i32, y as i32));
                        } else if let Some(&(p1, p2)) = portals.get(&label) {
                            area[p1.1 as usize][p1.0 as usize] = Cell::Portal(x as i32, y as i32);
                            area[pos.1 as usize][pos.0 as usize] = Cell::Portal(p2.0, p2.1);
                        } else {
                            portals.insert(label, (pos, (x as i32, y as i32)));
                        }
                    }
                }
            }
        }

        let start = start?;
        let end = end?;

        let width = area.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut explored = vec![vec![false; width]; area.len()];

        let mut stack: VecDeque<(i32, i32, i64)> = VecDeque::new();
        stack.push_back((start.0, start.1, 0));

        while let Some((x, y, steps)) = stack.pop_front() {
            explored[y as usize][x as usize] = true;

            if (x, y) == end {
                return Some(steps.to_string());
            }

            for d in DIRECTIONS {
                let (mut x_n, mut y_n) = (x + d.0, y + d.1);

                if y_n < 0
                    || y_n as usize >= area.len()
                    || x_n < 0
                    || x_n as usize >= area[y_n as usize].len()
                {
                    continue;
                }

                let cell = &area[y_n as usize][x_n as usize];
                let is_portal = matches!(cell, Cell::Portal(..));
                if !matches!(cell, Cell::Char('.')) && !is_portal {
                    continue;
                }

                if let Cell::Portal(px, py) = area[y_n as usize][x_n as usize] {
                    x_n = px;
                    y_n = py;
                }

                if !explored[y_n as usize][x_n as usize] {
                    stack.push_back((x_n, y_n, steps + 1));
                }
            }
        }

        // The original Python prints nothing if the end is never reached.
        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut area = parse(input);

        change_inner_portals(&mut area);

        let mut portals: FxHashMap<String, ((i32, i32), (i32, i32), bool)> = FxHashMap::default();
        let mut start: Option<(i32, i32)> = None;
        let mut end: Option<(i32, i32)> = None;

        for y in 0..area.len() {
            for x in 0..area[y].len() {
                if matches!(area[y][x], Cell::Char('.')) {
                    if let Some((label, pos, inner)) = get_portal_2(&area, x as i32, y as i32) {
                        if label == "AA" {
                            start = Some((x as i32, y as i32));
                        } else if label == "ZZ" {
                            end = Some((x as i32, y as i32));
                        } else if let Some(&(p1, p2, i)) = portals.get(&label) {
                            area[p1.1 as usize][p1.0 as usize] =
                                Cell::PortalDepth(x as i32, y as i32, i);
                            area[pos.1 as usize][pos.0 as usize] =
                                Cell::PortalDepth(p2.0, p2.1, inner);
                        } else {
                            portals.insert(label, (pos, (x as i32, y as i32), inner));
                        }
                    }
                }
            }
        }

        let start = start?;
        let end = end?;

        // explored[y][x] = set of depths
        let width = area.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut explored: Vec<Vec<FxHashSet<i64>>> =
            vec![vec![FxHashSet::default(); width]; area.len()];

        let mut stack: VecDeque<(i32, i32, i64, i64)> = VecDeque::new();
        stack.push_back((start.0, start.1, 0, 0));

        while let Some((x, y, steps, depth)) = stack.pop_front() {
            if (x, y) == end && depth == 0 {
                return Some(steps.to_string());
            }

            for d in DIRECTIONS {
                let (mut x_n, mut y_n) = (x + d.0, y + d.1);

                if y_n < 0
                    || y_n as usize >= area.len()
                    || x_n < 0
                    || x_n as usize >= area[y_n as usize].len()
                {
                    continue;
                }

                let cell = &area[y_n as usize][x_n as usize];
                let is_portal = matches!(cell, Cell::PortalDepth(..));
                if !matches!(cell, Cell::Char('.')) && !is_portal {
                    continue;
                }

                if !explored[y_n as usize][x_n as usize].contains(&depth) {
                    let mut new_depth = depth;
                    if let Cell::PortalDepth(px, py, inner) = area[y_n as usize][x_n as usize] {
                        x_n = px;
                        y_n = py;
                        new_depth += if inner { 1 } else { -1 };
                    }

                    if new_depth < 0 {
                        continue;
                    }

                    stack.push_back((x_n, y_n, steps + 1, new_depth));
                    explored[y_n as usize][x_n as usize].insert(new_depth);
                }
            }
        }

        // The original Python prints nothing if the end is never reached.
        None
    }
}

/// Flood-fill from the centre of the map through `' '` cells, lowercasing every
/// uppercase letter touched. These mark the *inner* portals (the outer-ring
/// labels sit beyond the surrounding wall and are never reached).
fn change_inner_portals(area: &mut [Vec<Cell>]) {
    if area.is_empty() {
        return;
    }
    let width = area.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut explored = vec![vec![false; width]; area.len()];

    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
    stack.push_back(((width / 2) as i32, (area.len() / 2) as i32));

    while let Some((x, y)) = stack.pop_front() {
        for d in DIRECTIONS {
            let (x_n, y_n) = (x + d.0, y + d.1);

            // Mirror Python's lack of bounds checking here: it would index and
            // raise on out-of-bounds. Rows can be ragged, so we guard to avoid a
            // panic; an out-of-bounds neighbour simply has no character.
            if y_n < 0 || y_n as usize >= area.len() || x_n < 0 {
                continue;
            }
            let row = &area[y_n as usize];
            if x_n as usize >= row.len() {
                continue;
            }

            if let Some(c) = area[y_n as usize][x_n as usize].ch() {
                if ('A'..='Z').contains(&c) {
                    area[y_n as usize][x_n as usize] =
                        Cell::Char(c.to_ascii_lowercase());
                } else if c == ' ' && !explored[y_n as usize][x_n as usize] {
                    stack.push_back((x_n, y_n));
                    explored[y_n as usize][x_n as usize] = true;
                }
            }
        }
    }
}

/// Part-2 portal detection: like `get_portal`, but it uppercases when reading the
/// label (so the lowercased inner markers still match) and reports whether the
/// portal is inner (the letter was lowercase, i.e. its byte had bit 0x20 set).
fn get_portal_2(area: &[Vec<Cell>], x: i32, y: i32) -> Option<(String, (i32, i32), bool)> {
    for (i, d) in DIRECTIONS.iter().enumerate() {
        let (x_n, y_n) = (x + d.0, y + d.1);

        if y_n < 0 || y_n as usize >= area.len() || x_n < 0 || x_n as usize >= area[y_n as usize].len()
        {
            continue;
        }

        if let Some(raw) = area[y_n as usize][x_n as usize].ch() {
            let up = raw.to_ascii_uppercase();
            if ('A'..='Z').contains(&up) {
                let x2 = x_n + DIRECTIONS[i].0;
                let y2 = y_n + DIRECTIONS[i].1;
                let c2 = area[y2 as usize][x2 as usize].ch().unwrap().to_ascii_uppercase();

                let mut chars = [up, c2];
                chars.sort_unstable();
                let label: String = chars.iter().collect();

                // inner iff the original letter was lowercase (bit 0x20 set)
                let inner = (raw as u32) & 32 != 0;
                return Some((label, (x_n, y_n), inner));
            }
        }
    }
    None
}

pub struct D20;
