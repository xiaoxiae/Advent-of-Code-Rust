//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/22
//!
//! ⚠️ part 2 UNSOLVED: the original 22-2.py crashed (IndexError) on the real input
//! — the cube-folding was never completed. solve_part2 returns None. Part 1 (36518)
//! is verified.
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D22;

const DELTAS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DELTAS_DIAGONAL: [(i64, i64); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

/// Parse input into (field as padded rows of chars, moves, turns, start_x).
fn parse(input: &str) -> (Vec<Vec<char>>, Vec<i64>, Vec<char>, i64) {
    // Split on first blank line (field \n\n instructions).
    let (field_part, instr_part) = input.split_once("\n\n").unwrap_or((input, ""));
    let instructions = instr_part.trim();

    let mut field: Vec<Vec<char>> = field_part
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    // ensure that all lines have the same length
    let maze_length = field.iter().map(|l| l.len()).max().unwrap_or(0);
    for row in field.iter_mut() {
        while row.len() < maze_length {
            row.push(' ');
        }
    }

    // parse moves and turns via re.split("R|L", ...) and re.split(r"\d+", ...)[1:-1]
    let mut moves: Vec<i64> = Vec::new();
    let mut turns: Vec<char> = Vec::new();
    let mut cur = String::new();
    for ch in instructions.chars() {
        if ch == 'R' || ch == 'L' {
            moves.push(cur.parse::<i64>().unwrap_or(0));
            cur.clear();
            turns.push(ch);
        } else {
            cur.push(ch);
        }
    }
    // last number after final turn
    moves.push(cur.parse::<i64>().unwrap_or(0));

    // start x: first non-space char in row 0
    let mut x_start = 0i64;
    if let Some(row0) = field.first() {
        if let Some(i) = row0.iter().position(|&c| c != ' ') {
            x_start = i as i64;
        }
    }

    (field, moves, turns, x_start)
}

fn at(field: &[Vec<char>], x: i64, y: i64) -> char {
    field[y as usize][x as usize]
}

fn rotate(orientation: i64, where_: char) -> i64 {
    (orientation + (if where_ == 'R' { 1 } else { -1 })).rem_euclid(DELTAS.len() as i64)
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (field, moves, turns, x_start) = parse(input);
        let height = field.len() as i64;

        let move_and_wrap = |x: i64, y: i64, delta: (i64, i64)| -> (i64, i64) {
            let mut x = x;
            let mut y = y;
            loop {
                y = (y + delta.1).rem_euclid(height);
                let w = field[y as usize].len() as i64;
                x = (x + delta.0).rem_euclid(w);
                if field[y as usize][x as usize] != ' ' {
                    return (x, y);
                }
            }
        };

        let move_forward = |x: i64, y: i64, orientation: i64, distance: i64| -> (i64, i64) {
            if distance == 0 {
                return (x, y);
            }
            let (dx, dy) = DELTAS[orientation as usize];
            let mut x = x;
            let mut y = y;
            for _ in 0..distance {
                let (nx, ny) = move_and_wrap(x, y, (dx, dy));
                if at(&field, nx, ny) != '.' {
                    return (x, y);
                }
                x = nx;
                y = ny;
            }
            (x, y)
        };

        let mut orientation = 0i64;
        let mut x = x_start;
        let mut y = 0i64;

        for (i, &m) in moves.iter().enumerate() {
            let (nx, ny) = move_forward(x, y, orientation, m);
            x = nx;
            y = ny;
            if i < turns.len() {
                orientation = rotate(orientation, turns[i]);
            }
        }

        let answer = 1000 * (y + 1) + 4 * (x + 1) + orientation;
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (field, moves, turns, x_start) = parse(input);
        let width = field.iter().map(|l| l.len()).max().unwrap_or(0) as i64;
        let height = field.len() as i64;

        let side_length = ((width * height / 12) as f64).sqrt() as i64;

        let y_start = 0i64;

        let in_bounds = |nx: i64, ny: i64| -> bool { 0 <= nx && nx < width && 0 <= ny && ny < height };

        // find the points on the border of the field (flood fill from start)
        let mut queue: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
        queue.push_back((x_start, y_start));
        let mut field_points: FxHashSet<(i64, i64)> = FxHashSet::default();
        field_points.insert((x_start, y_start));
        while let Some((cx, cy)) = queue.pop_front() {
            for &(dx, dy) in DELTAS.iter() {
                let (nx, ny) = (cx + dx, cy + dy);
                if !in_bounds(nx, ny) {
                    continue;
                }
                if at(&field, nx, ny) == ' ' {
                    continue;
                }
                if !field_points.contains(&(nx, ny)) {
                    field_points.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }

        // filter inner points
        let mut border_points: FxHashSet<(i64, i64)> = FxHashSet::default();
        for &(cx, cy) in field_points.iter() {
            let mut neighbouring = 0;
            for &(dx, dy) in DELTAS.iter() {
                if field_points.contains(&(cx + dx, cy + dy)) {
                    neighbouring += 1;
                }
            }
            if neighbouring != 4 {
                border_points.insert((cx, cy));
            }
        }

        // order them by their adjacency (start from (cx, cy) = last popped in flood fill;
        // matches Python which uses the leftover loop variable cx, cy).
        // In Python, cx, cy carry the last values from the flood-fill loop.
        // Reconstruct: the last popped node. We track it during flood fill above is gone;
        // re-run minimal: but Python's queue empties so cx,cy = last popped element.
        // We replicate by repeating the flood fill to capture the final (cx, cy).
        let mut last_cx = x_start;
        let mut last_cy = y_start;
        {
            let mut q2: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
            q2.push_back((x_start, y_start));
            let mut seen: FxHashSet<(i64, i64)> = FxHashSet::default();
            seen.insert((x_start, y_start));
            while let Some((cx, cy)) = q2.pop_front() {
                last_cx = cx;
                last_cy = cy;
                for &(dx, dy) in DELTAS.iter() {
                    let (nx, ny) = (cx + dx, cy + dy);
                    if !in_bounds(nx, ny) {
                        continue;
                    }
                    if at(&field, nx, ny) == ' ' {
                        continue;
                    }
                    if !seen.contains(&(nx, ny)) {
                        seen.insert((nx, ny));
                        q2.push_back((nx, ny));
                    }
                }
            }
        }

        let deltas_plus_diag: Vec<(i64, i64)> = [DELTAS, DELTAS_DIAGONAL].concat();

        let mut bqueue: std::collections::VecDeque<(i64, i64)> = std::collections::VecDeque::new();
        bqueue.push_back((last_cx, last_cy));
        let mut sorted_border_points: Vec<(i64, i64)> = Vec::new();
        let mut sorted_set: FxHashSet<(i64, i64)> = FxHashSet::default();
        while let Some((cx, cy)) = bqueue.pop_front() {
            sorted_border_points.push((cx, cy));
            sorted_set.insert((cx, cy));
            for &(dx, dy) in deltas_plus_diag.iter() {
                let (nx, ny) = (cx + dx, cy + dy);
                if border_points.contains(&(nx, ny)) && !sorted_set.contains(&(nx, ny)) {
                    bqueue.push_back((nx, ny));
                    break;
                }
            }
        }

        // build the border list of (x, y, orientation)
        let mut border: Vec<(i64, i64, i64)> = Vec::new();
        for &(cx, cy) in sorted_border_points.iter() {
            for (o, &(dx, dy)) in DELTAS.iter().enumerate() {
                let (nx, ny) = (cx + dx, cy + dy);
                if !in_bounds(nx, ny) || at(&field, nx, ny) == ' ' {
                    border.push((cx, cy, o as i64));
                    continue;
                }
            }
        }

        let blen = border.len() as i64;
        // Guard: original Python crashes (IndexError) on inputs where this logic
        // goes out of range. Replicate negative-index semantics where possible; if
        // an access would be out of bounds, mirror the Python crash by bailing.
        let bidx = |i: i64| -> Option<usize> {
            if blen == 0 {
                return None;
            }
            let m = i.rem_euclid(blen);
            Some(m as usize)
        };

        // fix incorrectly oriented corners
        // for i in range(-3, len(border)): uses border[i+j] for j in -3..=0
        {
            let mut i = -3i64;
            while i < blen {
                // Python indexes border[i+j]; negative indices wrap, positive >= len crash.
                let mut vals = [0i64; 4];
                let mut ok = true;
                for (k, j) in (-3..=0).enumerate() {
                    let idx = i + j;
                    if idx >= blen {
                        ok = false;
                        break;
                    }
                    match bidx(idx) {
                        Some(u) => vals[k] = border[u].2,
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }
                if !ok {
                    // Python would raise IndexError here; abort part 2 like the original.
                    return None;
                }
                let (o1, o2, o3, o4) = (vals[0], vals[1], vals[2], vals[3]);
                if o1 == o3 && o2 == o4 && o1 != o2 && o2 != o3 && o3 != o4 {
                    // swap border[i-2] and border[i-1]
                    let ia = bidx(i - 2);
                    let ib = bidx(i - 1);
                    if let (Some(a), Some(b)) = (ia, ib) {
                        border.swap(a, b);
                    }
                }
                i += 1;
            }
        }

        let is_edge_point = |x: i64, y: i64| -> Option<(i64, i64)> {
            let mut air_count = 0;
            let mut wall_count = 0;
            let mut d: Option<(i64, i64)> = None;
            for &(dx, dy) in DELTAS_DIAGONAL.iter() {
                if at(&field, x + dx, y + dy) == ' ' {
                    air_count += 1;
                    d = Some((dx, dy));
                }
            }
            for &(dx, dy) in DELTAS.iter() {
                if at(&field, x + dx, y + dy) != ' ' {
                    wall_count += 1;
                }
            }
            if air_count == 1 && wall_count == 4 {
                d
            } else {
                None
            }
        };

        let deltas_index = |d: (i64, i64)| -> i64 {
            DELTAS.iter().position(|&v| v == d).unwrap() as i64
        };

        // portals: (x, y, orientation) -> (x, y, orientation)
        let mut portals: FxHashMap<(i64, i64, i64), (i64, i64, i64)> = FxHashMap::default();

        // create edge point portals
        let mut corner: Vec<(Vec<(i64, i64, i64)>, Vec<(i64, i64, i64)>)> = Vec::new();
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                if let Some(d) = is_edge_point(x, y) {
                    let mut a_side = Vec::new();
                    let mut b_side = Vec::new();
                    for i in 1..=side_length {
                        let a_in = (x + d.0 * i, y, deltas_index((0, d.1)));
                        let b_in = (x, y + d.1 * i, deltas_index((d.0, 0)));
                        let a_out = (b_in.0, b_in.1, (b_in.2 + 2).rem_euclid(DELTAS.len() as i64));
                        let b_out = (a_in.0, a_in.1, (a_in.2 + 2).rem_euclid(DELTAS.len() as i64));
                        portals.insert(a_in, a_out);
                        portals.insert(b_in, b_out);
                        a_side.push(a_in);
                        b_side.push(b_in);
                    }
                    corner.push((a_side, b_side));
                }
            }
        }

        let move_and_wrap = |x: i64, y: i64, orientation: i64,
                             portals: &FxHashMap<(i64, i64, i64), (i64, i64, i64)>|
         -> (i64, i64, i64) {
            if let Some(&p) = portals.get(&(x, y, orientation)) {
                return p;
            }
            let (dx, dy) = DELTAS[orientation as usize];
            let (mut nx, mut ny) = (x + dx, y + dy);
            if !field_points.contains(&(nx, ny)) {
                nx = x;
                ny = y;
            }
            (nx, ny, orientation)
        };

        // walk from each side of the corner and wherever you end up, those two wrap
        for (a_side, b_side) in corner.iter() {
            for (a0, b0) in a_side.iter().zip(b_side.iter()) {
                let mut a = *a0;
                let mut b = *b0;
                for _ in 0..(side_length * 4) {
                    a = move_and_wrap(a.0, a.1, a.2, &portals);
                    b = move_and_wrap(b.0, b.1, b.2, &portals);
                }
                portals.insert(a, (b.0, b.1, (b.2 + 2).rem_euclid(DELTAS.len() as i64)));
                portals.insert(b, (a.0, a.1, (a.2 + 2).rem_euclid(DELTAS.len() as i64)));
            }
        }

        // add the one remaining side
        let mut remaining: Vec<usize> = Vec::new();
        for (i, b) in border.iter().enumerate() {
            if !portals.contains_key(b) {
                remaining.push(i);
            }
        }

        for j in 0..2i64 {
            for ii in 0..side_length {
                let i = ii + j * side_length * 2;
                let idx1 = i as usize;
                let idx2 = (side_length * 2 - i - 1) as usize;
                if idx1 >= remaining.len() || idx2 >= remaining.len() {
                    return None;
                }
                let o1 = border[remaining[idx1]];
                let o2 = border[remaining[idx2]];
                portals.insert(o1, (o2.0, o2.1, (o2.2 + 2).rem_euclid(DELTAS.len() as i64)));
                portals.insert(o2, (o1.0, o1.1, (o1.2 + 2).rem_euclid(DELTAS.len() as i64)));
            }
        }

        let move_forward = |x: i64, y: i64, orientation: i64, distance: i64,
                            portals: &FxHashMap<(i64, i64, i64), (i64, i64, i64)>|
         -> (i64, i64, i64) {
            if distance == 0 {
                return (x, y, orientation);
            }
            let mut x = x;
            let mut y = y;
            let mut orientation = orientation;
            for _ in 0..distance {
                let (nx, ny, norientation) = move_and_wrap(x, y, orientation, portals);
                if at(&field, nx, ny) != '.' {
                    return (x, y, orientation);
                }
                x = nx;
                y = ny;
                orientation = norientation;
            }
            (x, y, orientation)
        };

        let mut x = x_start;
        let mut y = y_start;
        let mut orientation = 0i64;
        for (i, &m) in moves.iter().enumerate() {
            let (nx, ny, no) = move_forward(x, y, orientation, m, &portals);
            x = nx;
            y = ny;
            orientation = no;
            if i < turns.len() {
                orientation = rotate(orientation, turns[i]);
            }
        }

        let answer = 1000 * (y + 1) + 4 * (x + 1) + orientation;
        Some(answer.to_string())
    }
}
