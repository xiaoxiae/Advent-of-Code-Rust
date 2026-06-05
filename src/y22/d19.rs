//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/19
//!
//! ⚠️ SLOW (~27s part 1, ~32s part 2): faithful port of the heap-based state search
//! (the original Python took ~3 min). Correct (988 / 8580) but flagged for manual
//! optimization (better pruning / DFS).
use crate::util::Day;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct D19;

type Ores = [i64; 4];
type Robots = [i64; 4];
type Blueprint = [[i64; 4]; 4];

fn can_build_robot(ores: &Ores, cost: &[i64; 4]) -> bool {
    ores.iter().zip(cost).all(|(o, c)| o >= c)
}

/// Return the next states, given the current ores and robots.
fn next_states(ores: &Ores, robots: &Robots, blueprint: &Blueprint) -> Vec<(Ores, Robots)> {
    let mut states: Vec<(Ores, Robots)> = vec![(*ores, *robots)];

    // attempt to build more robots
    // we're assuming that we can build at most one each turn
    let mut can_build = 0;
    for i in 0..4 {
        let cost = &blueprint[i];
        if can_build_robot(ores, cost) {
            can_build += 1;
            // NOTE: python does `o - c - (0 if i != j else 1)` for ores and
            // `r + (0 if i != j else 1)` for robots — replicated exactly.
            let mut new_ores = *ores;
            let mut new_robots = *robots;
            for j in 0..4 {
                new_ores[j] -= cost[j] + if i == j { 1 } else { 0 };
                new_robots[j] += if i == j { 1 } else { 0 };
            }
            states.push((new_ores, new_robots));
        }
    }

    // if we can build any robot, don't store resources
    if can_build == 4 {
        states.remove(0);
    }

    for (o, r) in states.iter_mut() {
        for i in 0..4 {
            o[i] += r[i];
        }
    }

    states
}

/// Return True if the state can beat max_geodes score.
fn can_be_best(remaining: i64, ores: &Ores, robots: &Robots, max_geodes: i64) -> bool {
    // shit estimation: assume we can build a geode robot every turn
    (remaining * (remaining - 1)) / 2 + robots[3] * remaining + ores[3] > max_geodes
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p = |i: usize| -> i64 { parts[i].parse().unwrap() };
        let blueprint: Blueprint = [
            [p(6), 0, 0, 0],            // ore
            [p(12), 0, 0, 0],           // clay
            [p(18), p(21), 0, 0],       // obsidian
            [p(27), 0, p(30), 0],       // geode
        ];
        blueprints.push(blueprint);
    }
    blueprints
}

fn priority(ores: &Ores) -> i64 {
    ores[0] * 1000 + ores[1] * 100 + ores[2] * 10 + ores[3]
}

/// Run the search for a single blueprint with the given time budget.
fn search(blueprint: &Blueprint, time: i64) -> i64 {
    // heap items: (priority, remaining, (ores, robots))
    // Python heapq is a min-heap; we wrap with Reverse.
    type Item = (i64, i64, (Ores, Robots));
    let start: (Ores, Robots) = ([0, 0, 0, 0], [1, 0, 0, 0]);

    let mut queue: BinaryHeap<Reverse<Item>> = BinaryHeap::new();
    queue.push(Reverse((0, time, start)));

    let mut visited: HashMap<(Ores, Robots), i64> = HashMap::new();
    visited.insert(start, time);

    let mut max_geodes = 0i64;

    while let Some(Reverse((_s, remaining, (ores, robots)))) = queue.pop() {
        max_geodes = max_geodes.max(ores[3]);

        if remaining == 0 {
            continue;
        }

        if *visited.get(&(ores, robots)).unwrap_or(&-1) != remaining {
            continue;
        }

        if !can_be_best(remaining, &ores, &robots, max_geodes) {
            continue;
        }

        for (n_ores, n_robots) in next_states(&ores, &robots, blueprint) {
            let key = (n_ores, n_robots);
            let better = match visited.get(&key) {
                None => true,
                Some(&v) => v < remaining - 1,
            };
            if better {
                queue.push(Reverse((priority(&n_ores), remaining - 1, key)));
                visited.insert(key, remaining - 1);
            }
        }
    }

    max_geodes
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let blueprints = parse(input);
        let mut total = 0i64;
        for (bpid, blueprint) in blueprints.iter().enumerate() {
            let max_geodes = search(blueprint, 24);
            total += max_geodes * (bpid as i64 + 1);
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let blueprints = parse(input);
        let mut total = 1i64;
        for blueprint in blueprints.iter().take(3) {
            let max_geodes = search(blueprint, 32);
            total *= max_geodes;
        }
        Some(total.to_string())
    }
}
