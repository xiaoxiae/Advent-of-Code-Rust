use crate::util::Day;
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct Day5;

fn get_graph(edges: &Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::default();

    for (u, v) in edges {
        graph.entry(*u).or_insert_with(HashSet::default).insert(*v);
    }

    graph
}

fn get_edges(rules: &str) -> Vec<(i32, i32)> {
    rules
        .split("\n")
        .map(|rule| {
            rule.split("|")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>()
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

fn get_in_degrees(graph: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> HashMap<i32, i32> {
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

    in_degrees
}

// fn is_total_ordering(graph: &HashMap<i32, HashSet<i32>>) -> bool {
//     let in_degrees = get_in_degrees(&graph, &graph.keys().cloned().collect::<Vec<i32>>());
//
//     // TODO: ended here, it's a fucked graph, would have to remove rules that cause a cycle
//
//
//     println!("{:?}", in_degrees.values().cloned());
//
//     false
// }

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
    let in_degrees = get_in_degrees(&graph, &update);

    // Sort by vertex in-degrees, ascending
    let mut vec = in_degrees.into_iter().collect::<Vec<(i32, i32)>>();
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    vec.iter().map(|&(key, _)| key).collect()
}

impl Day for Day5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let parts = input.trim().splitn(2, "\n\n").collect::<Vec<&str>>();
        let (rules, updates) = (parts[0], parts[1]);

        let graph = get_graph(&get_edges(rules));

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

        let graph = get_graph(&get_edges(rules));

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

    // /// --- Tom's Part 3 ---
    // /// We are interested to see how many rules we need to create total ordering.
    // /// First, use the numbers from the second part to shuffle the rules in the following way:
    // /// - for each (idx, number), swap [idx % rules.len()] and [(idx + number) % rules.len()]
    // fn solve_part3(&self, input: &str) -> Option<String> {
    //     let parts = input.trim().splitn(2, "\n\n").collect::<Vec<&str>>();
    //     let (rules, updates) = (parts[0], parts[1]);

    //     let mut edges = get_edges(rules);

    //     for (index, rule) in yield_updates(updates)
    //         .collect::<Vec<Vec<i32>>>()
    //         .iter()
    //         .flatten()
    //         .enumerate()
    //     {
    //         let len = edges.len();
    //         edges.swap(index % len, (index + *rule as usize) % len);
    //     }

    //     let mut graph = get_graph(&Vec::new());

    //     for i in 1..edges.len() {
    //         let (u, v) = &edges[i];

    //         graph.entry(*u).or_insert_with(HashSet::default).insert(*v);

    //         if is_total_ordering(&graph) {
    //             return Option::from(i.to_string());
    //         }
    //     }

    //     panic!("Not totally orderable?")
    // }
}
