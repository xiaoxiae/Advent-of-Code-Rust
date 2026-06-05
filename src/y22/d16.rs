//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/16
//!
//! ⚠️ SLOW (~13s, part 2): faithful port of the subset-enumeration BFS (32k valve
//! subsets). Correct (1850 / 2306) but flagged for manual optimization.
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub struct D16;

/// Parse the input into a map: valve name -> (flow, neighbours).
fn parse(input: &str) -> FxHashMap<String, (i64, Vec<String>)> {
    let mut valves: FxHashMap<String, (i64, Vec<String>)> = FxHashMap::default();

    for row in input.lines() {
        let row = row.trim_end_matches('\n');
        if row.trim().is_empty() {
            continue;
        }

        // Python: row.split(maxsplit=9)
        let mut parts: Vec<&str> = row.splitn(10, ' ').collect();
        // The last element may contain leading whitespace removed by python split;
        // splitn keeps the rest intact (no extra leading space here since fields are single-spaced).
        let valve = parts[1].to_string();
        // parts[4] is like "rate=22;" -> [5:-1] gives "22"
        let flow_field = parts[4];
        let flow_str = &flow_field[5..flow_field.len() - 1];
        let flow: i64 = flow_str.parse().unwrap();
        let last = parts.pop().unwrap();
        let leads_to: Vec<String> = last.split(", ").map(|s| s.to_string()).collect();

        valves.insert(valve, (flow, leads_to));
    }

    valves
}

/// BFS distances from `valve` to all other valves.
fn bfs_distances(
    valves: &FxHashMap<String, (i64, Vec<String>)>,
    valve: &str,
) -> FxHashMap<String, i64> {
    let mut visited: FxHashSet<String> = FxHashSet::default();
    let mut queue: VecDeque<(String, i64)> = VecDeque::new();
    queue.push_back((valve.to_string(), 0));
    let mut paths: FxHashMap<String, i64> = FxHashMap::default();

    while let Some((current, distance)) = queue.pop_front() {
        paths.insert(current.clone(), distance);

        for neighbour in &valves[&current].1 {
            if !visited.contains(neighbour) {
                visited.insert(neighbour.clone());
                queue.push_back((neighbour.clone(), distance + 1));
            }
        }
    }

    paths.remove(valve);
    paths
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let valves = parse(input);

        // precompute distances from every valve
        let mut valve_paths: FxHashMap<String, FxHashMap<String, i64>> = FxHashMap::default();
        for valve in valves.keys() {
            valve_paths.insert(valve.clone(), bfs_distances(&valves, valve));
        }

        // BFS over states: (remaining, current, opened (sorted tuple), pressure)
        let mut queue: VecDeque<(i64, String, Vec<String>, i64)> = VecDeque::new();
        queue.push_back((30, "AA".to_string(), Vec::new(), 0));
        let mut max_pressure: i64 = 0;

        while let Some((remaining, current, opened, pressure)) = queue.pop_front() {
            max_pressure = max_pressure.max(pressure);

            for (tunnel, &distance) in &valve_paths[&current] {
                if !opened.contains(tunnel) && valves[tunnel].0 != 0 {
                    if remaining - distance - 1 >= 0 {
                        let mut new_opened = opened.clone();
                        new_opened.push(tunnel.clone());
                        new_opened.sort();
                        queue.push_back((
                            remaining - distance - 1,
                            tunnel.clone(),
                            new_opened,
                            pressure + (remaining - distance - 1) * valves[tunnel].0,
                        ));
                    }
                }
            }
        }

        Some(max_pressure.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let valves = parse(input);

        // interesting (non-zero) valves, sorted
        let mut non_zero_valves: Vec<String> = valves
            .iter()
            .filter(|(_, (flow, _))| *flow != 0)
            .map(|(name, _)| name.clone())
            .collect();
        non_zero_valves.sort();

        // precompute distances between interesting valves (and from AA)
        let mut valve_paths: FxHashMap<String, FxHashMap<String, i64>> = FxHashMap::default();
        let non_zero_set: FxHashSet<String> = non_zero_valves.iter().cloned().collect();

        let mut sources: Vec<String> = non_zero_valves.clone();
        sources.push("AA".to_string());

        for valve in &sources {
            let mut visited: FxHashSet<String> = FxHashSet::default();
            let mut queue: VecDeque<(String, i64)> = VecDeque::new();
            queue.push_back((valve.clone(), 0));
            let mut paths: FxHashMap<String, i64> = FxHashMap::default();

            while let Some((current, distance)) = queue.pop_front() {
                if non_zero_set.contains(&current) {
                    paths.insert(current.clone(), distance);
                }
                for neighbour in &valves[&current].1 {
                    if !visited.contains(neighbour) {
                        visited.insert(neighbour.clone());
                        queue.push_back((neighbour.clone(), distance + 1));
                    }
                }
            }

            if valve != "AA" {
                paths.remove(valve);
            }
            valve_paths.insert(valve.clone(), paths);
        }

        // Map valve names to bit indices for fast subset handling.
        let index_of: FxHashMap<String, usize> = non_zero_valves
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect();
        let n = non_zero_valves.len();
        let full_mask: u32 = if n == 32 { u32::MAX } else { (1u32 << n) - 1 };

        // For each subset, compute the best pressure achievable by a single agent
        // restricted to opening only valves in that subset.
        // subset_score keyed by bitmask.
        let mut subset_score: FxHashMap<u32, i64> = FxHashMap::default();

        // subsets(s): all non-empty proper subsets (k from 1 to len-1)
        // i.e. masks from 1 to full_mask-1 that are not equal to full_mask.
        // The python generator yields combinations for k in range(1, len(s)),
        // which is exactly every mask with 1 <= popcount < n.
        for mask in 1u32..full_mask {
            // mask ranges over [1, full_mask - 1]; popcount in [1, n-1] automatically
            // since full_mask itself (popcount n) is excluded and 0 (popcount 0) skipped.
            // BFS for this subset.
            let mut max_pressure: i64 = 0;
            // state: (remaining, current, opened_mask, pressure)
            let mut queue: VecDeque<(i64, String, u32, i64)> = VecDeque::new();
            queue.push_back((26, "AA".to_string(), 0, 0));

            while let Some((remaining, current, opened, pressure)) = queue.pop_front() {
                max_pressure = max_pressure.max(pressure);

                for (tunnel, &distance) in &valve_paths[&current] {
                    let bit = 1u32 << index_of[tunnel];
                    if mask & bit == 0 {
                        continue;
                    }
                    if opened & bit != 0 {
                        continue;
                    }
                    if remaining - distance - 1 < 0 {
                        continue;
                    }
                    let new_opened = opened | bit;
                    queue.push_back((
                        remaining - distance - 1,
                        tunnel.clone(),
                        new_opened,
                        pressure + (remaining - distance - 1) * valves[tunnel].0,
                    ));
                }
            }

            subset_score.insert(mask, max_pressure);
        }

        // Combine each subset with its complement.
        let mut max_pressure: i64 = 0;
        for mask in 1u32..full_mask {
            let other = full_mask & !mask;
            let a = *subset_score.get(&mask).unwrap_or(&0);
            let b = *subset_score.get(&other).unwrap_or(&0);
            max_pressure = max_pressure.max(a + b);
        }

        Some(max_pressure.to_string())
    }
}
