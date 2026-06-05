//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/14
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D14;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect()
}

/// Slide a single 'O' rock located at (x, y) as far as possible in `direction`,
/// stopping when it goes out of bounds or hits a non-'.' cell.
fn slide_one(array: &mut [Vec<u8>], x: i64, y: i64, direction: (i64, i64)) {
    let n = array.len() as i64;

    if array[y as usize][x as usize] != b'O' {
        return;
    }

    let (mut nx, mut ny) = (x, y);
    loop {
        nx += direction.0;
        ny += direction.1;

        if !(0 <= nx && nx < n && 0 <= ny && ny < n) || array[ny as usize][nx as usize] != b'.' {
            break;
        }
    }

    nx -= direction.0;
    ny -= direction.1;

    let tmp = array[y as usize][x as usize];
    array[y as usize][x as usize] = array[ny as usize][nx as usize];
    array[ny as usize][nx as usize] = tmp;
}

fn slide(array: &mut [Vec<u8>], direction: (i64, i64)) {
    let n = array.len() as i64;

    let x = if direction.0 == 0 || direction.0 == -1 {
        0
    } else {
        n - 1
    };
    let y = if direction.1 == 0 || direction.1 == -1 {
        0
    } else {
        n - 1
    };
    let k = if direction.0 != 0 { 0 } else { 1 };

    for i in 0..n {
        let dx = -direction.0 * i;
        let dy = -direction.1 * i;

        for j in 0..n {
            let mut new = [x + dx, y + dy];
            new[1 - k] += j;

            slide_one(array, new[0], new[1], direction);
        }
    }
}

fn cycle(array: &mut [Vec<u8>]) {
    for d in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
        slide(array, d);
    }
}

fn load(array: &[Vec<u8>]) -> usize {
    array
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == b'O').count() * (row.len() - i))
        .sum()
}

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut array = parse(input);
        slide(&mut array, (0, -1));
        Some(load(&array).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut array = parse(input);

        let mut i: usize = 0;
        let mut visited: FxHashMap<Vec<u8>, usize> = FxHashMap::default();

        let (start, period) = loop {
            cycle(&mut array);

            let hash: Vec<u8> = array.iter().flatten().copied().collect();

            i += 1;

            if let Some(&prev) = visited.get(&hash) {
                break (prev, i - prev);
            }

            visited.insert(hash, i);
        };

        let n: usize = 1_000_000_000;

        for _ in 0..((n - start) % period) {
            cycle(&mut array);
        }

        Some(load(&array).to_string())
    }
}
