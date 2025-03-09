use crate::util::Day;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

pub struct D7;

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut predecessors: HashMap<char, usize> = HashMap::new();
        let mut successors: HashMap<char, Vec<char>> = HashMap::new();

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let start = chars[5];
            let end = chars[36];

            successors.entry(start).or_default().push(end);
            successors.entry(end).or_default();
            *predecessors.entry(start).or_insert(0) += 0;
            *predecessors.entry(end).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<Reverse<char>> = predecessors
            .iter()
            .filter(|&(_, &v)| v == 0)
            .map(|(&k, _)| Reverse(k))
            .collect();

        let mut order = String::new();

        while let Some(Reverse(element)) = heap.pop() {
            order.push(element);
            if let Some(next_steps) = successors.get(&element) {
                for &successor in next_steps {
                    let count = predecessors.get_mut(&successor).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        heap.push(Reverse(successor));
                    }
                }
            }
        }

        Some(order)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut predecessors: HashMap<char, usize> = HashMap::new();
        let mut successors: HashMap<char, Vec<char>> = HashMap::new();
        let worker_count = 5;
        let base_duration = 60;

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let start = chars[5];
            let end = chars[36];

            successors.entry(start).or_default().push(end);
            successors.entry(end).or_default();
            *predecessors.entry(start).or_insert(0) += 0;
            *predecessors.entry(end).or_insert(0) += 1;
        }

        let mut available: Vec<(usize, char)> = predecessors
            .iter()
            .filter(|&(_, &v)| v == 0)
            .map(|(&k, _)| (base_duration + (k as u8 - b'A' + 1) as usize, k))
            .collect();
        available.sort();

        let mut time_elapsed = 0;
        while !available.is_empty() {
            let min_time = available.iter().take(worker_count).map(|&(t, _)| t).min().unwrap();
            time_elapsed += min_time;

            let mut completed = vec![];
            for i in 0..available.len().min(worker_count) {
                available[i].0 -= min_time;
                if available[i].0 == 0 {
                    completed.push(i);
                }
            }

            completed.sort_unstable_by(|a, b| b.cmp(a));
            for idx in completed {
                let (_, task) = available.remove(idx);
                if let Some(next_steps) = successors.get(&task) {
                    for &successor in next_steps {
                        let count = predecessors.get_mut(&successor).unwrap();
                        *count -= 1;
                        if *count == 0 {
                            available.push((base_duration + (successor as u8 - b'A' + 1) as usize, successor));
                        }
                    }
                }
            }
            available.sort();
        }

        Some(time_elapsed.to_string())
    }
}
