//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/24
//!
//! ⚠️ SLOW (~11s, part 2): faithful port of the blizzard-basin BFS (the original
//! Python took ~8 min). Correct (301 / 859) but flagged for manual optimization.
use crate::util::Day;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use rustc_hash::FxHashSet;

pub struct D24;

const DELTAS: [(i64, i64); 5] = [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)];
const BLIZZARD_DIRECTIONS: [char; 4] = ['v', '^', '<', '>'];

struct Map {
    raw_map: Vec<Vec<char>>,
    width: i64,
    height: i64,
    blizzards: Vec<((i64, i64), (i64, i64))>,
    start: (i64, i64),
    end: (i64, i64),
}

fn parse(input: &str) -> Map {
    let raw_map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .filter(|r| !r.is_empty())
        .collect();

    let width = raw_map[0].len() as i64;
    let height = raw_map.len() as i64;

    let mut blizzards = Vec::new();
    for (y, row) in raw_map.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if let Some(idx) = BLIZZARD_DIRECTIONS.iter().position(|&c| c == ch) {
                blizzards.push(((x as i64, y as i64), DELTAS[idx]));
            }
        }
    }

    let start = (1, 0);
    let end = (width - 2, height - 1);

    Map {
        raw_map,
        width,
        height,
        blizzards,
        start,
        end,
    }
}

impl Map {
    fn is_valid(&self, pos: (i64, i64), steps: i64) -> bool {
        let (x, y) = pos;

        if !(0 <= x && x < self.width && 0 <= y && y < self.height) {
            return false;
        }

        if self.raw_map[y as usize][x as usize] == '#' {
            return false;
        }

        for &((bx0, by0), (dx, dy)) in &self.blizzards {
            let mut bx = bx0 + dx * steps;
            let mut by = by0 + dy * steps;

            bx = (bx - 1).rem_euclid(self.width - 2) + 1;
            by = (by - 1).rem_euclid(self.height - 2) + 1;

            if (bx, by) == (x, y) {
                return false;
            }
        }

        true
    }
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let m = parse(input);

        // Faithful translation, including the original heuristic's use of end[0]
        // for the y-component (i.e. abs(y - end.0)).
        let heuristic = |pos: (i64, i64), steps: i64| -> i64 {
            (pos.0 - m.end.0).abs() + (pos.1 - m.end.0).abs() + steps
        };

        let start_state = (m.start, 0i64);

        // min-heap by (heuristic, ((x, y), steps)) matching Python's tuple ordering
        let mut queue: BinaryHeap<Reverse<(i64, i64, i64, i64)>> = BinaryHeap::new();
        let mut visited: FxHashSet<((i64, i64), i64)> = FxHashSet::default();

        let h0 = heuristic(start_state.0, start_state.1);
        queue.push(Reverse((h0, start_state.0 .0, start_state.0 .1, start_state.1)));
        visited.insert(start_state);

        while let Some(Reverse((_d, x, y, steps))) = queue.pop() {
            if (x, y) == m.end {
                return Some(steps.to_string());
            }

            for (dx, dy) in DELTAS {
                let npos = (x + dx, y + dy);
                let nsteps = steps + 1;
                let state = (npos, nsteps);

                if visited.contains(&state) {
                    continue;
                }

                if m.is_valid(npos, nsteps) {
                    let h = heuristic(npos, nsteps);
                    queue.push(Reverse((h, npos.0, npos.1, nsteps)));
                    visited.insert(state);
                }
            }
        }

        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let m = parse(input);

        let heuristic = |pos: (i64, i64), steps: i64, phase: i64| -> i64 {
            if phase == 0 {
                2 * distance(m.start, m.end) + distance(pos, m.end) + steps
            } else if phase == 1 {
                distance(m.start, m.end) + distance(pos, m.start) + steps
            } else {
                distance(pos, m.end) + steps
            }
        };

        let start_state = (m.start, 0i64, 0i64);

        // min-heap by (heuristic, ((x, y), steps, phase)) matching Python's tuple ordering
        let mut queue: BinaryHeap<Reverse<(i64, i64, i64, i64, i64)>> = BinaryHeap::new();
        let mut visited: FxHashSet<((i64, i64), i64, i64)> = FxHashSet::default();

        let h0 = heuristic(start_state.0, start_state.1, start_state.2);
        queue.push(Reverse((
            h0,
            start_state.0 .0,
            start_state.0 .1,
            start_state.1,
            start_state.2,
        )));
        visited.insert(start_state);

        while let Some(Reverse((_d, x, y, steps, mut phase))) = queue.pop() {

            if (x, y) == m.end && phase == 0 {
                phase += 1;
            } else if (x, y) == m.start && phase == 1 {
                phase += 1;
            } else if (x, y) == m.end && phase == 2 {
                return Some(steps.to_string());
            }

            for (dx, dy) in DELTAS {
                let npos = (x + dx, y + dy);
                let nsteps = steps + 1;
                let state = (npos, nsteps, phase);

                if visited.contains(&state) {
                    continue;
                }

                if m.is_valid(npos, nsteps) {
                    let h = heuristic(npos, nsteps, phase);
                    queue.push(Reverse((h, npos.0, npos.1, nsteps, phase)));
                    visited.insert(state);
                }
            }
        }

        None
    }
}
