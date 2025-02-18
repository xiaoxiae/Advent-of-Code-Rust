use crate::util::Day;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct D12;

fn parse(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut graph = HashMap::new();

    for line in input.trim().lines() {
        let parts = line.split(" <-> ").collect::<Vec<_>>();

        let from = parts[0].parse::<usize>().unwrap();

        for to in parts[1].split(", ") {
            let to = to.parse::<usize>().unwrap();

            graph.entry(from).or_insert_with(Vec::new).push(to);
            graph.entry(to).or_insert_with(Vec::new).push(from);
        }
    }

    graph
}

fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> HashSet<usize> {
    let mut explored = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for &neighbour in graph.get(&current).unwrap() {
            if explored.contains(&neighbour) {
                continue;
            }

            explored.insert(neighbour);
            queue.push_back(neighbour);
        }
    }

    explored
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let graph = parse(input);

        let explored = bfs(&graph, 0);

        Option::from(explored.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let graph = parse(input);

        let mut explored = HashSet::new();
        let mut groups = 0;

        for &vertex in graph.keys() {
            if explored.contains(&vertex) {
                continue;
            }

            explored.extend(bfs(&graph, vertex));
            groups += 1;
        }

        Option::from(groups.to_string())
    }
}
