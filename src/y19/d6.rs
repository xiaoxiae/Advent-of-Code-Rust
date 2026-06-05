//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/06
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D6;

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // create a one-way tree
        let mut tree: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
        for orbit in input.trim().lines() {
            let (v1, v2) = orbit.split_once(')').unwrap();
            tree.entry(v1).or_default().push(v2);
        }

        // iterative DFS for the sum of depths from the root
        let mut total: i64 = 0;
        let mut stack: Vec<(&str, i64)> = vec![("COM", 0)];
        while let Some((v1, depth)) = stack.pop() {
            total += depth;
            if let Some(children) = tree.get(v1) {
                for &v in children {
                    stack.push((v, depth + 1));
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // create a two-way tree
        let mut tree: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
        for orbit in input.trim().lines() {
            let (v1, v2) = orbit.split_once(')').unwrap();
            tree.entry(v1).or_default().push(v2);
            tree.entry(v2).or_default().push(v1);
        }

        // bfs/dfs variables
        let mut stack: Vec<(&str, i64)> = vec![("YOU", 0)];
        let mut visited: FxHashSet<&str> = FxHashSet::default();

        while let Some((current, value)) = stack.pop() {
            visited.insert(current);

            if current == "SAN" {
                return Some((value - 2).to_string());
            }

            if let Some(neighbors) = tree.get(current) {
                for &v in neighbors {
                    if !visited.contains(v) {
                        stack.push((v, value + 1));
                    }
                }
            }
        }

        None
    }
}
