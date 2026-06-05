//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/18
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D18;

const DELTAS: [(i64, i64, i64); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

fn parse(input: &str) -> Vec<(i64, i64, i64)> {
    let mut cubes = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split(',').map(|p| p.trim().parse::<i64>().unwrap());
        let x = it.next().unwrap();
        let y = it.next().unwrap();
        let z = it.next().unwrap();
        cubes.push((x, y, z));
    }
    cubes
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let cubes_vec = parse(input);
        let cubes: FxHashSet<(i64, i64, i64)> = cubes_vec.iter().copied().collect();

        let mut total: i64 = 0;
        for &(x, y, z) in &cubes_vec {
            for &(dx, dy, dz) in &DELTAS {
                let n = (x + dx, y + dy, z + dz);
                if !cubes.contains(&n) {
                    total += 1;
                }
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let cubes_vec = parse(input);
        let cubes: FxHashSet<(i64, i64, i64)> = cubes_vec.iter().copied().collect();

        let mut min_bounds = [i64::MAX; 3];
        let mut max_bounds = [i64::MIN; 3];
        for &(x, y, z) in &cubes_vec {
            let c = [x, y, z];
            for i in 0..3 {
                if c[i] < min_bounds[i] {
                    min_bounds[i] = c[i];
                }
                if c[i] > max_bounds[i] {
                    max_bounds[i] = c[i];
                }
            }
        }
        for i in 0..3 {
            min_bounds[i] -= 1;
            max_bounds[i] += 1;
        }

        let in_bounds = |pos: (i64, i64, i64)| -> bool {
            let p = [pos.0, pos.1, pos.2];
            for i in 0..3 {
                if !(min_bounds[i] <= p[i] && p[i] <= max_bounds[i]) {
                    return false;
                }
            }
            true
        };

        let start = (min_bounds[0], min_bounds[1], min_bounds[2]);
        let mut queue: Vec<(i64, i64, i64)> = vec![start];
        let mut visited: FxHashSet<(i64, i64, i64)> = FxHashSet::default();
        visited.insert(start);
        let mut total: i64 = 0;

        while let Some(current) = queue.pop() {
            let (x, y, z) = current;
            for &(dx, dy, dz) in &DELTAS {
                let n = (x + dx, y + dy, z + dz);
                if !in_bounds(n) {
                    continue;
                }
                if cubes.contains(&n) {
                    total += 1;
                    continue;
                }
                if visited.insert(n) {
                    queue.push(n);
                }
            }
        }

        Some(total.to_string())
    }
}
