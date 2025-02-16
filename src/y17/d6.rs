use crate::util::Day;
use std::collections::HashSet;

pub struct D6;

fn redistribute(blocks: &mut Vec<u32>) {
    let max_idx = blocks.iter().enumerate().min_by_key(|&(i, &v)| (!v, i)).unwrap().0;
    let max_val = blocks[max_idx];
    blocks[max_idx] = 0;

    for i in 0..max_val {
        let n = blocks.len();
        blocks[(max_idx + 1 + i as usize) % n] += 1;
    }
}

fn find_cycle_length(blocks: &mut Vec<u32>) -> usize {
    let mut seen = HashSet::new();
    let mut cycles = 0;

    while seen.insert(blocks.clone()) {
        redistribute(blocks);
        cycles += 1;
    }
    cycles
}

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut blocks: Vec<u32> = input.trim().split("\t").filter_map(|s| s.parse().ok()).collect();
        Some(find_cycle_length(&mut blocks).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut blocks: Vec<u32> = input.trim().split("\t").filter_map(|s| s.parse().ok()).collect();
        find_cycle_length(&mut blocks); // Run once to reach loop start
        Some(find_cycle_length(&mut blocks).to_string()) // Run again to measure loop size
    }
}
