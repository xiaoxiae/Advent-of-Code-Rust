//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/17
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D17;

// shapes in relative coordinates
// (such that (0, 0) is two units from left wall) and three up from the highest rock
const SHAPES: [&[(i64, i64)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 1), (1, 1), (2, 1), (1, 2), (1, 0)],
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

fn shape_coordinates(shape_index: usize, shape_offset: [i64; 2]) -> impl Iterator<Item = (i64, i64)> {
    SHAPES[shape_index]
        .iter()
        .map(move |&(dx, dy)| (dx + shape_offset[0], dy + shape_offset[1]))
}

fn max_y(rocks: &FxHashSet<(i64, i64)>) -> i64 {
    rocks.iter().map(|&(_, y)| y).max().unwrap_or(0)
}

fn is_shape_valid(
    rocks: &FxHashSet<(i64, i64)>,
    shape_index: usize,
    shape_offset: [i64; 2],
) -> bool {
    for (x, y) in shape_coordinates(shape_index, shape_offset) {
        if !(0 <= x && x < 7 && 0 <= y) {
            return false;
        }
        if rocks.contains(&(x, y)) {
            return false;
        }
    }
    true
}

fn add_shape(rocks: &mut FxHashSet<(i64, i64)>, shape_index: usize, shape_offset: [i64; 2]) {
    for (x, y) in shape_coordinates(shape_index, shape_offset) {
        rocks.insert((x, y));
    }
}

/// Attempt to apply a push. Returns true if it was applied (mutating shape_offset).
fn attempt_push(
    rocks: &FxHashSet<(i64, i64)>,
    shape_index: usize,
    shape_offset: &mut [i64; 2],
    push: (i64, i64),
) -> bool {
    let (dx, dy) = push;

    shape_offset[0] += dx;
    shape_offset[1] += dy;

    if !is_shape_valid(rocks, shape_index, *shape_offset) {
        shape_offset[0] -= dx;
        shape_offset[1] -= dy;
        return false;
    }

    true
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let pushes: Vec<char> = input.trim().chars().collect();

        let mut rocks: FxHashSet<(i64, i64)> = FxHashSet::default();
        let mut shape_offset: [i64; 2] = [2, 3];
        let mut shape_index: usize = 0;
        let mut push_index: usize = 0;

        let mut rock_count: i64 = 0;
        loop {
            let push = if pushes[push_index] == '<' { -1 } else { 1 };
            attempt_push(&rocks, shape_index, &mut shape_offset, (push, 0));

            push_index = (push_index + 1) % pushes.len();

            if !attempt_push(&rocks, shape_index, &mut shape_offset, (0, -1)) {
                add_shape(&mut rocks, shape_index, shape_offset);

                rock_count += 1;
                shape_index = (shape_index + 1) % SHAPES.len();
                shape_offset = [2, max_y(&rocks) + 4];
            }

            if rock_count == 2022 {
                break;
            }
        }

        Some((shape_offset[1] - 3).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let pushes: Vec<char> = input.trim().chars().collect();

        let mut rocks: FxHashSet<(i64, i64)> = FxHashSet::default();
        let mut shape_history: Vec<i64> = vec![-1; SHAPES.len()];
        let mut shape_offset: [i64; 2] = [2, 3];
        let mut shape_index: usize = 0;
        let mut push_index: usize = 0;

        // visited maps state -> (max_y, rock_count)
        let mut visited: FxHashMap<Vec<i64>, (i64, i64)> = FxHashMap::default();
        let mut rock_count: i64 = 0;

        let mut rocks_to_fall: i64 = 1_000_000_000_000;
        let mut repetition_found = false;
        let mut y_offset: i64 = 0;

        loop {
            let push = if pushes[push_index] == '<' { -1 } else { 1 };
            attempt_push(&rocks, shape_index, &mut shape_offset, (push, 0));

            push_index = (push_index + 1) % pushes.len();

            if !attempt_push(&rocks, shape_index, &mut shape_offset, (0, -1)) {
                add_shape(&mut rocks, shape_index, shape_offset);
                let my = max_y(&rocks);
                rock_count += 1;

                if !repetition_found {
                    // state = (shape_index, push_index, *shape_history)
                    let mut state: Vec<i64> = Vec::with_capacity(2 + shape_history.len());
                    state.push(shape_index as i64);
                    state.push(push_index as i64);
                    state.extend_from_slice(&shape_history);

                    if let Some(&(prev_my, prev_rock_count)) = visited.get(&state) {
                        // how many fell by that point
                        let period = rock_count - prev_rock_count;
                        rocks_to_fall -= rock_count;

                        // the y difference in the period
                        y_offset = (rocks_to_fall / period) * (my - prev_my);

                        // how many rocks are there remaining
                        rocks_to_fall = rocks_to_fall % period + 1;
                        rock_count = 0;

                        repetition_found = true;
                    }

                    visited.insert(state, (my, rock_count));
                }

                shape_index = (shape_index + 1) % SHAPES.len();
                shape_history.remove(0);
                shape_history.push(shape_offset[0]);
                shape_offset = [2, my + 4];
            }

            if rock_count == rocks_to_fall {
                break;
            }
        }

        Some((max_y(&rocks) + y_offset).to_string())
    }
}
