//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/18
use std::collections::VecDeque;

use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::util::Day;

pub struct D18;

fn parse(input: &str) -> (Vec<Vec<char>>, (i32, i32), usize) {
    let mut area: Vec<Vec<char>> = Vec::new();
    let mut pos = (0i32, 0i32);
    let mut key_count = 0;

    for (y, line) in input.trim_end_matches('\n').lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let mut c = ch;
            if c == '@' {
                pos = (x as i32, y as i32);
                c = '.';
            } else if c.is_ascii_lowercase() {
                key_count += 1;
            }
            row.push(c);
        }
        area.push(row);
    }

    (area, pos, key_count)
}

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (area, pos, key_count) = parse(input);

        let height = area.len();
        let width = area[0].len();

        // run BFS over (x, y, steps, keys)
        let mut stack: VecDeque<(i32, i32, u32, Vec<char>)> = VecDeque::new();
        stack.push_back((pos.0, pos.1, 0, Vec::new()));

        let mut explored: Vec<Vec<FxHashSet<Vec<char>>>> =
            vec![vec![FxHashSet::default(); width]; height];

        while let Some((x, y, steps, keys)) = stack.pop_front() {
            // check if we haven't already been here
            let mut key_tuple = keys.clone();
            key_tuple.sort_unstable();
            if explored[y as usize][x as usize].contains(&key_tuple) {
                continue;
            }
            explored[y as usize][x as usize].insert(key_tuple);

            for (x_d, y_d) in DIRS {
                let x_n = x + x_d;
                let y_n = y + y_d;

                if x_n < 0 || x_n >= width as i32 || y_n < 0 || y_n >= height as i32 {
                    continue;
                }

                let ch = area[y_n as usize][x_n as usize];

                if ch.is_ascii_lowercase() {
                    // walking over a key, add it to the keys set
                    let mut new_keys = keys.clone();
                    if !new_keys.contains(&ch) {
                        new_keys.push(ch);
                    }

                    // Python uses set union; emulate set membership count
                    let distinct: FxHashSet<char> = new_keys.iter().copied().collect();
                    if distinct.len() == key_count {
                        return Some((steps + 1).to_string());
                    } else {
                        stack.push_back((x_n, y_n, steps + 1, new_keys));
                    }
                } else if ch == '.'
                    || (ch.is_ascii_uppercase()
                        && keys.contains(&ch.to_ascii_lowercase()))
                {
                    stack.push_back((x_n, y_n, steps + 1, keys.clone()));
                }
            }
        }

        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut area: Vec<Vec<char>> = Vec::new();
        let mut pos = (0i32, 0i32);

        for (y, line) in input.trim_end_matches('\n').lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    pos = (x as i32, y as i32);
                }
                row.push(ch);
            }
            area.push(row);
        }

        // build the blockage
        area[pos.1 as usize][pos.0 as usize] = '#';
        for (x_d, y_d) in DIRS {
            area[(pos.1 + y_d) as usize][(pos.0 + x_d) as usize] = '#';
        }

        // get all key/door sequences in all of the 4 corners
        let corners = [(-1, 1), (1, 1), (-1, -1), (1, -1)];
        let mut sequences: Vec<Vec<(u32, Vec<char>)>> = Vec::new();
        for (x_d, y_d) in corners {
            sequences.push(yield_sequences(&area, (pos.0 + x_d, pos.1 + y_d)));
        }

        // for each sequence quadruple, figure out if it is possible
        let mut minimum_steps: Option<u32> = None;

        for combination in sequences.iter().map(|v| v.iter()).multi_cartesian_product() {
            let mut keys: FxHashSet<char> = FxHashSet::default();
            let mut positions = vec![0usize; combination.len()];

            loop {
                let mut was_changed = false;

                for (i, c) in combination.iter().enumerate() {
                    let sequence = &c.1;

                    while positions[i] < sequence.len() {
                        let ch = sequence[positions[i]];
                        if ch.is_ascii_lowercase() {
                            keys.insert(ch);
                            positions[i] += 1;
                            was_changed = true;
                        } else if ch.is_ascii_uppercase() {
                            if !keys.contains(&ch.to_ascii_lowercase()) {
                                break;
                            }
                            positions[i] += 1;
                        }
                    }
                }

                // if we're stuck at doors, go to the next combination
                if !was_changed {
                    break;
                }

                // if there are no more doors, evaluate the combination score
                let all_done = combination
                    .iter()
                    .enumerate()
                    .all(|(i, c)| positions[i] == c.1.len());
                if all_done {
                    let steps: u32 = combination.iter().map(|c| c.0).sum();
                    if minimum_steps.is_none_or(|m| m > steps) {
                        minimum_steps = Some(steps);
                    }
                    break;
                }
            }
        }

        minimum_steps.map(|m| m.to_string())
    }
}

/// Return the possible key/door sequences for collecting all keys in a given sector.
fn yield_sequences(area: &[Vec<char>], pos: (i32, i32)) -> Vec<(u32, Vec<char>)> {
    let height = area.len();
    let width = area[0].len();

    // count all the keys in the given sector (by running a regular BFS)
    let mut stack: Vec<(i32, i32)> = vec![pos];
    let mut explored = vec![vec![false; width]; height];
    let mut key_count = 0;

    while let Some((x, y)) = stack.pop() {
        if explored[y as usize][x as usize] {
            continue;
        }
        explored[y as usize][x as usize] = true;

        let ch = area[y as usize][x as usize];
        if ch.is_ascii_lowercase() {
            key_count += 1;
        }

        for (x_d, y_d) in DIRS {
            let x_n = x + x_d;
            let y_n = y + y_d;
            if area[y_n as usize][x_n as usize] != '#' {
                stack.push((x_n, y_n));
            }
        }
    }

    // run the weird bfs and return all possible orders of keys-doors
    let mut results: Vec<(u32, Vec<char>)> = Vec::new();
    let mut queue: VecDeque<(i32, i32, u32, Vec<char>, Vec<char>)> = VecDeque::new();
    queue.push_back((pos.0, pos.1, 0, Vec::new(), Vec::new()));

    let mut explored2: Vec<Vec<FxHashSet<Vec<char>>>> =
        vec![vec![FxHashSet::default(); width]; height];

    while let Some((x, y, steps, keys, sequence)) = queue.pop_front() {
        let key_tuple = keys.clone();
        if explored2[y as usize][x as usize].contains(&key_tuple) {
            continue;
        }
        explored2[y as usize][x as usize].insert(key_tuple);

        for (x_d, y_d) in DIRS {
            let x_n = x + x_d;
            let y_n = y + y_d;

            if x_n < 0 || x_n >= width as i32 || y_n < 0 || y_n >= height as i32 {
                continue;
            }

            let ch = area[y_n as usize][x_n as usize];

            if ch.is_ascii_lowercase() {
                // walking over a key
                let mut new_keys = keys.clone();
                if !new_keys.contains(&ch) {
                    new_keys.push(ch);
                }

                if new_keys.len() == key_count {
                    let mut new_seq = sequence.clone();
                    new_seq.push(ch);
                    results.push((steps + 1, new_seq));
                } else {
                    let mut new_seq = sequence.clone();
                    new_seq.push(ch);
                    queue.push_back((x_n, y_n, steps + 1, new_keys, new_seq));
                }
            } else if ch == '.' {
                queue.push_back((x_n, y_n, steps + 1, keys.clone(), sequence.clone()));
            } else if ch.is_ascii_uppercase() {
                let mut new_seq = sequence.clone();
                new_seq.push(ch);
                queue.push_back((x_n, y_n, steps + 1, keys.clone(), new_seq));
            }
        }
    }

    results
}
