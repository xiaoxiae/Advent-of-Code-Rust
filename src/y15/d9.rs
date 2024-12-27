use crate::util::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct D9;

fn get_path_sizes(input: &str) -> Vec<usize> {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    let mut destinations = HashSet::new();
    let mut distances: HashMap<(String, String), usize> = HashMap::new();

    for cap in re.captures_iter(input) {
        let from = &cap[1];
        let to = &cap[2];
        let distance: usize = cap[3].parse().unwrap();

        distances.insert((from.to_string(), to.to_string()), distance);
        distances.insert((to.to_string(), from.to_string()), distance);

        destinations.insert(from.to_string());
        destinations.insert(to.to_string());
    }

    destinations
        .iter()
        .permutations(destinations.len())
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(&from, &to)| distances.get(&(from.to_string(), to.to_string())).unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>()
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let min_dist = *get_path_sizes(input).iter().min().unwrap();
        Option::from(min_dist.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let max_dist = *get_path_sizes(input).iter().max().unwrap();
        Option::from(max_dist.to_string())
    }
}
