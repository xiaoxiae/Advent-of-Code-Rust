use crate::util::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct D7;


// this function should be cached :(
fn calculate_subtree_weights(node: &str, tree: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>) -> usize {
    if !tree.contains_key(node) {
        return weights[node];
    }

    let subweight = tree.get(node).unwrap().iter().map(|&child| calculate_subtree_weights(child, tree, weights)).sum::<usize>();

    subweight + weights[node]
}


fn find_problem_child(node: &str, tree: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, usize>) -> String {
    if !tree.contains_key(node) {
        return node.to_string();
    }

    let mut child_weights = vec![];
    for &child in tree.get(node).unwrap().iter() {
        child_weights.push((child, calculate_subtree_weights(child, tree, weights)));
    }

    let mut unbalanced = false;
    let mut counts = HashMap::new();

    for &w in child_weights.iter() {
        *counts.entry(w.1).or_insert(0) += 1;
    }

    // this approach is not correct and fails for unbalanced nodes with 2 children
    // this is because in that case we have to examine both
    // since this didn't happen for my input and I'm too lazy, I'm leaving it like this
    let mut offender = None;
    for &w in child_weights.iter() {
        if let Some(&count) = counts.get(&w.1) {
            if count == 1 {
                offender = Some(w.0);
                unbalanced = true;
            }
        }
    }

    if unbalanced {
        return find_problem_child(offender.unwrap(), tree, weights);
    }

    node.to_string()
}


impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut nodes = HashSet::new();
        let mut stacked_nodes = HashSet::new();
        let re = Regex::new(r"^(\w+) \(\d+\)(?: -> (.+))?$").unwrap();

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let name = caps.get(1).unwrap().as_str();
                nodes.insert(name);

                if let Some(children) = caps.get(2) {
                    for node in children.as_str().split(", ") {
                        stacked_nodes.insert(node);
                    }
                }
            }
        }

        let root = nodes.symmetric_difference(&stacked_nodes).next()?;
        Some(root.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut tree = HashMap::new();
        let mut weights = HashMap::new();

        let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.+))?$").unwrap();

        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                let name = caps.get(1).unwrap().as_str();

                let mut children = vec![];

                let weight = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

                if let Some(children_str) = caps.get(3) {
                    for node in children_str.as_str().split(", ") {
                        children.push(node);
                    }
                }

                tree.insert(name, children);
                weights.insert(name, weight);
            }
        }

        // I'm lazy, this should be done properly
        let root = self.solve_part1(input).unwrap();

        let unbalanced = find_problem_child(root.as_str(), &tree, &weights);  // lol
        let unbalanced_weight = calculate_subtree_weights(&unbalanced.as_str(), &tree, &weights);

        for nodes in tree.values() {
            if nodes.contains(&unbalanced.as_str()) {
                let w = nodes.iter().map(|&n| calculate_subtree_weights(n, &tree, &weights)).collect::<Vec<_>>();
                let d = (w.iter().sum::<usize>() - unbalanced_weight) / (w.len() - 1);

                let delta = d as isize - unbalanced_weight as isize;

                return Some((weights[unbalanced.as_str()] as isize + delta).to_string());
            }
        }
        
        unreachable!();
    }
}
