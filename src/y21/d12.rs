//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/12
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D12;

fn is_small(x: &str) -> bool {
    x == x.to_lowercase()
}

fn parse(input: &str) -> FxHashMap<String, Vec<String>> {
    let mut caves: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (u, v) = line.split_once('-').unwrap();
        caves.entry(u.to_string()).or_default().push(v.to_string());
        caves.entry(v.to_string()).or_default().push(u.to_string());
    }
    caves
}

fn count_distinct_paths_1(
    caves: &FxHashMap<String, Vec<String>>,
    current: &str,
    visited: &mut FxHashSet<String>,
    total: &mut u64,
) {
    if current == "end" {
        *total += 1;
        return;
    }

    for neighbour in &caves[current] {
        if neighbour == "start" || visited.contains(neighbour) {
            continue;
        }

        let small = is_small(neighbour);
        if small {
            visited.insert(neighbour.clone());
        }

        count_distinct_paths_1(caves, neighbour, visited, total);

        if small {
            visited.remove(neighbour);
        }
    }
}

fn count_distinct_paths_2(
    caves: &FxHashMap<String, Vec<String>>,
    current: &str,
    visited: &mut FxHashSet<String>,
    repeated: bool,
    total: &mut u64,
) {
    if current == "end" {
        *total += 1;
        return;
    }

    for neighbour in &caves[current] {
        if neighbour == "start" {
            continue;
        }

        if visited.contains(neighbour) {
            if !repeated {
                count_distinct_paths_2(caves, neighbour, visited, true, total);
            }
            continue;
        }

        let small = is_small(neighbour);
        if small {
            visited.insert(neighbour.clone());
        }

        count_distinct_paths_2(caves, neighbour, visited, repeated, total);

        if small {
            visited.remove(neighbour);
        }
    }
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let caves = parse(input);
        let mut total = 0u64;
        let mut visited = FxHashSet::default();
        count_distinct_paths_1(&caves, "start", &mut visited, &mut total);
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let caves = parse(input);
        let mut total = 0u64;
        let mut visited = FxHashSet::default();
        count_distinct_paths_2(&caves, "start", &mut visited, false, &mut total);
        Some(total.to_string())
    }
}
