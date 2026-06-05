//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/08
use crate::util::Day;

pub struct D8;

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

/// Recursively sum the metadata of the tree, returning (total, next position).
fn sum_tree_metadata(data: &[i64], pos: usize) -> (i64, usize) {
    let n_children = data[pos];
    let n_metadata = data[pos + 1];
    let mut p = pos + 2;
    let mut total = 0;

    for _ in 0..n_children {
        let (child_total, next) = sum_tree_metadata(data, p);
        total += child_total;
        p = next;
    }

    total += data[p..p + n_metadata as usize].iter().sum::<i64>();
    p += n_metadata as usize;

    (total, p)
}

/// Construct the node's value per part 2 rules, returning (value, next position).
fn find_root_value(data: &[i64], pos: usize) -> (i64, usize) {
    let n_children = data[pos];
    let n_metadata = data[pos + 1];
    let mut p = pos + 2;

    let mut children: Vec<i64> = Vec::new();
    for _ in 0..n_children {
        let (child_value, next) = find_root_value(data, p);
        children.push(child_value);
        p = next;
    }

    let metadata = &data[p..p + n_metadata as usize];
    p += n_metadata as usize;

    let total = if children.is_empty() {
        metadata.iter().sum()
    } else {
        metadata
            .iter()
            // -1 because indexes start at 1 in the problem
            .filter(|&&index| index >= 1 && (index - 1) < children.len() as i64)
            .map(|&index| children[(index - 1) as usize])
            .sum()
    };

    (total, p)
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let data = parse(input);
        let (total, _) = sum_tree_metadata(&data, 0);
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let data = parse(input);
        let (value, _) = find_root_value(&data, 0);
        Some(value.to_string())
    }
}
