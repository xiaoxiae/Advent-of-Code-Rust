use crate::util::Day;
use rayon::prelude::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet };

pub struct Day5;

/// Construct a graph, given rules (=edges)
fn get_graph(rules: &str) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::default();
    for rule in rules.split("\n") {
        let parts = rule.split("|").collect::<Vec<&str>>();

        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();

        graph.entry(x).or_insert_with(HashSet::default).insert(y);
    }

    graph
}

/// Return updates, one by one, from the string input
fn yield_updates(updates: &str) -> impl ParallelIterator<Item = Vec<i32>> + '_ {
    updates.trim().par_split('\n').map(|update| {
        update
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    })
}

fn check_update(graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            // If the inverse edge exists, update is incorrect
            match graph.get(&update[j]) {
                Some(v) if v.contains(&update[i]) => return false,
                _ => continue,
            }
        }
    }

    true
}

fn sort_update(graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut in_degrees: HashMap<i32, i32> = update.into_iter().map(|key| (*key, 0)).collect();

    for i in 0..update.len() {
        for j in i + 1..update.len() {
            // Check both edge directions
            for (u, v) in [(i, j), (j, i)] {
                match graph.get(&update[u]) {
                    Some(k) if k.contains(&update[v]) => {
                        *in_degrees.entry(update[v]).or_insert(0) += 1
                    }
                    _ => continue,
                }
            }
        }
    }

    // Sort by vertex in-degrees, ascending
    let mut vec = in_degrees.into_iter().collect::<Vec<(i32, i32)>>();
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec.iter().map(|&(key, _)| key).collect()
}

impl Day for Day5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let parts = input.trim().splitn(2, "\n\n").collect::<Vec<&str>>();
        let (rules, updates) = (parts[0], parts[1]);

        let graph = get_graph(rules);

        let sum: i32 = yield_updates(updates)
            .filter_map(|update| {
                if check_update(&graph, &update) {
                    Some(update[update.len() / 2])
                } else {
                    None
                }
            })
            .sum();

        Option::from(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let parts = input.trim().splitn(2, "\n\n").collect::<Vec<&str>>();
        let (rules, updates) = (parts[0], parts[1]);

        let graph = get_graph(rules);

        let sum: i32 = yield_updates(updates)
            .filter_map(|update| {
                if check_update(&graph, &update) {
                    None
                } else {
                    let sorted_update = sort_update(&graph, &update);
                    Some(sorted_update[sorted_update.len() / 2])
                }
            })
            .sum();

        Option::from(sum.to_string())
    }
}
