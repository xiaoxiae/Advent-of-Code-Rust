//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/17
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D17;

fn neighbours_3d(state: &FxHashSet<(i64, i64, i64)>, cell: (i64, i64, i64)) -> usize {
    let (x, y, z) = cell;
    let mut total = 0;
    for xd in [0i64, 1, -1] {
        for yd in [0i64, 1, -1] {
            for zd in [0i64, 1, -1] {
                if xd == 0 && yd == 0 && zd == 0 {
                    continue;
                }
                if state.contains(&(x + xd, y + yd, z + zd)) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn neighbouring_cells_3d(cell: (i64, i64, i64), yield_itself: bool) -> Vec<(i64, i64, i64)> {
    let (x, y, z) = cell;
    let mut out = Vec::new();
    for xd in [0i64, 1, -1] {
        for yd in [0i64, 1, -1] {
            for zd in [0i64, 1, -1] {
                if xd == 0 && yd == 0 && zd == 0 {
                    continue;
                }
                out.push((x + xd, y + yd, z + zd));
            }
        }
    }
    if yield_itself {
        out.push(cell);
    }
    out
}

fn neighbours_4d(state: &FxHashSet<(i64, i64, i64, i64)>, cell: (i64, i64, i64, i64)) -> usize {
    let (x, y, z, a) = cell;
    let mut total = 0;
    for xd in [0i64, 1, -1] {
        for yd in [0i64, 1, -1] {
            for zd in [0i64, 1, -1] {
                for ad in [0i64, 1, -1] {
                    if xd == 0 && yd == 0 && zd == 0 && ad == 0 {
                        continue;
                    }
                    if state.contains(&(x + xd, y + yd, z + zd, a + ad)) {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

fn neighbouring_cells_4d(
    cell: (i64, i64, i64, i64),
    yield_itself: bool,
) -> Vec<(i64, i64, i64, i64)> {
    let (x, y, z, a) = cell;
    let mut out = Vec::new();
    for xd in [0i64, 1, -1] {
        for yd in [0i64, 1, -1] {
            for zd in [0i64, 1, -1] {
                for ad in [0i64, 1, -1] {
                    if xd == 0 && yd == 0 && zd == 0 && ad == 0 {
                        continue;
                    }
                    out.push((x + xd, y + yd, z + zd, a + ad));
                }
            }
        }
    }
    if yield_itself {
        out.push(cell);
    }
    out
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim_end_matches('\n').lines().collect();

        let mut state: FxHashSet<(i64, i64, i64)> = FxHashSet::default();
        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '#' {
                    state.insert((i as i64, j as i64, 0));
                }
            }
        }

        for _ in 0..6 {
            let mut new_state: FxHashSet<(i64, i64, i64)> = FxHashSet::default();
            for &cell in &state {
                for c in neighbouring_cells_3d(cell, true) {
                    if state.contains(&c) {
                        let n = neighbours_3d(&state, c);
                        if (2..=3).contains(&n) {
                            new_state.insert(c);
                        }
                    } else if neighbours_3d(&state, c) == 3 {
                        new_state.insert(c);
                    }
                }
            }
            state = new_state;
        }

        Some(state.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim_end_matches('\n').lines().collect();

        let mut state: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '#' {
                    state.insert((i as i64, j as i64, 0, 0));
                }
            }
        }

        for _ in 0..6 {
            let mut new_state: FxHashSet<(i64, i64, i64, i64)> = FxHashSet::default();
            for &cell in &state {
                for c in neighbouring_cells_4d(cell, true) {
                    if state.contains(&c) {
                        let n = neighbours_4d(&state, c);
                        if (2..=3).contains(&n) {
                            new_state.insert(c);
                        }
                    } else if neighbours_4d(&state, c) == 3 {
                        new_state.insert(c);
                    }
                }
            }
            state = new_state;
        }

        Some(state.len().to_string())
    }
}
