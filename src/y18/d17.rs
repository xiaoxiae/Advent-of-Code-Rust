//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/17
use crate::util::Day;
use regex::Regex;

pub struct D17;

/// Build the `world` grid exactly like the Python:
/// `world[x][y]` where x spans [minX, maxX) and y spans [minY, maxY).
/// Cell values: 0 empty, 1 clay, 2 flowing water, 3 still water.
/// Returns (world, min_x) so the source position 500-minX can be computed.
fn build_world(input: &str) -> (Vec<Vec<u8>>, isize) {
    let re = Regex::new(r"\d+").unwrap();

    // rows: lines starting with 'y' -> [y, x1, x2]
    // columns: lines starting with 'x' -> [x, y1, y2]
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut columns: Vec<Vec<i64>> = Vec::new();

    for line in input.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let numbers: Vec<i64> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        if line.starts_with('x') {
            columns.push(numbers);
        } else {
            rows.push(numbers);
        }
    }

    // minX = min over (columns[*][0]) and (rows[*][1..])
    // Python: min([min(columns)[0]] + min([row[1:] for row in rows]))
    // min(columns) compares lists lexicographically; [0] takes first elem.
    // min([row[1:] ...]) returns the lexicographically smallest tail list,
    // then outer min over [that single value] + that list flattens... but
    // Python's `+` here concatenates a 1-element list with a list, and min
    // over the concatenation. To stay faithful to the *intended* result
    // (overall min / max x and y), we compute the true extrema directly,
    // which matches the original on real inputs.
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for c in &columns {
        // x = c[0], y1 = c[1], y2 = c[2]
        min_x = min_x.min(c[0]);
        max_x = max_x.max(c[0]);
        min_y = min_y.min(c[1].min(c[2]));
        max_y = max_y.max(c[1].max(c[2]));
    }
    for r in &rows {
        // y = r[0], x1 = r[1], x2 = r[2]
        min_y = min_y.min(r[0]);
        max_y = max_y.max(r[0]);
        min_x = min_x.min(r[1].min(r[2]));
        max_x = max_x.max(r[1].max(r[2]));
    }
    // Python adds +1 to maxX and maxY.
    max_x += 1;
    max_y += 1;

    let width = (max_x - min_x) as usize; // x dimension
    let depth = (max_y - min_y) as usize; // y dimension

    let mut world: Vec<Vec<u8>> = vec![vec![0u8; depth]; width];

    // add columns (vertical clay): x fixed, y from y1..=y2
    for c in &columns {
        let x = (c[0] - min_x) as usize;
        for y in c[1]..=c[2] {
            world[x][(y - min_y) as usize] = 1;
        }
    }
    // add rows (horizontal clay): y fixed, x from x1..=x2
    for r in &rows {
        let y = (r[0] - min_y) as usize;
        for x in r[1]..=r[2] {
            world[(x - min_x) as usize][y] = 1;
        }
    }

    (world, min_x as isize)
}

/// Python-style index into the x dimension: negative indices wrap around to
/// the end (e.g. -1 -> width - 1). The original solution genuinely relies on
/// this behaviour — `minX` is allowed to go to -1, and `world[-1]` then refers
/// to the last row — so we must reproduce it faithfully (its answers depend on
/// this quirk).
#[inline]
fn px(x: isize, width: usize) -> usize {
    let w = width as isize;
    (((x % w) + w) % w) as usize
}

#[inline]
fn get(world: &[Vec<u8>], x: isize, y: usize) -> u8 {
    world[px(x, world.len())][y]
}

#[inline]
fn set(world: &mut [Vec<u8>], x: isize, y: usize, v: u8) {
    let i = px(x, world.len());
    world[i][y] = v;
}

/// Find the bounds by either overflowing or hitting a wall.
/// Returns (min_x, max_x, min_overflow, max_overflow).
fn bounds(x: isize, y: usize, world: &[Vec<u8>]) -> (isize, isize, bool, bool) {
    let mut min_x = x;
    let mut max_x = x;

    // flow left
    while get(world, min_x, y + 1) != 0
        && get(world, min_x, y + 1) != 2
        && get(world, min_x - 1, y) != 1
    {
        min_x -= 1;
    }
    let min_overflow = get(world, min_x, y + 1) == 0 || get(world, min_x, y + 1) == 2;

    // flow right
    while get(world, max_x, y + 1) != 0
        && get(world, max_x, y + 1) != 2
        && get(world, max_x + 1, y) != 1
    {
        max_x += 1;
    }
    let max_overflow = get(world, max_x, y + 1) == 0 || get(world, max_x, y + 1) == 2;

    (min_x, max_x, min_overflow, max_overflow)
}

/// Recursively fill the tank.
fn recursion(x: isize, y: usize, world: &mut Vec<Vec<u8>>) {
    let depth = world[px(x, world.len())].len();

    // if we hit the bottom or other flowing water
    if y + 1 == depth || get(world, x, y + 1) == 2 {
        set(world, x, y, 2);
        return;
    }

    // if we can flow downwards, do so
    if get(world, x, y + 1) == 0 {
        set(world, x, y, 2);
        recursion(x, y + 1, world);
    }

    // if there is still water under us or clay after flowing down, flow sideways
    if get(world, x, y + 1) == 1 || get(world, x, y + 1) == 3 {
        let (b_min, b_max, b_min_of, b_max_of) = bounds(x, y, world);
        let fill = if b_min_of || b_max_of { 2 } else { 3 };
        for flow_x in b_min..=b_max {
            set(world, flow_x, y, fill);
        }

        // if either of the sides overflowed, recursively fall from that side
        if b_min_of {
            recursion(b_min, y, world);
        }
        if b_max_of {
            recursion(b_max, y, world);
        }
    }
}

/// Build the world and run the (deep) recursion on a thread with a large
/// stack, returning the filled world. The recursion depth is proportional to
/// the grid depth (~1700 on real inputs), which can overflow the default
/// stack, so we mirror Python's `setrecursionlimit` with a roomy stack.
fn filled_world(input: &str) -> Vec<Vec<u8>> {
    let input = input.to_owned();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let (mut world, min_x) = build_world(&input);
            recursion(500 - min_x, 0, &mut world);
            world
        })
        .unwrap()
        .join()
        .unwrap()
}

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let world = filled_world(input);

        // count the total number of water (flowing 2 or still 3)
        let total: usize = world
            .iter()
            .flatten()
            .filter(|&&n| n == 2 || n == 3)
            .count();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let world = filled_world(input);

        // count the number of still water (3)
        let total: usize = world
            .iter()
            .flatten()
            .filter(|&&n| n == 3)
            .count();

        Some(total.to_string())
    }
}
