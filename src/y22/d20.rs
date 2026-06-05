//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/20
use crate::util::Day;

pub struct D20;

fn parse(input: &str, key: i64) -> Vec<(usize, i64)> {
    input
        .split_whitespace()
        .enumerate()
        .map(|(i, v)| (i, v.parse::<i64>().unwrap() * key))
        .collect()
}

fn mix(numbers: &mut Vec<(usize, i64)>, orig_index: usize) {
    // Find the position of the element whose original index == orig_index.
    let j = numbers.iter().position(|&(k, _)| k == orig_index).unwrap();

    let n = numbers.remove(j);

    let len = numbers.len() as i64;
    let mut new_i = (j as i64 + n.1).rem_euclid(len);

    if new_i == 0 {
        new_i = len;
    } else if new_i == len {
        new_i = 0;
    }

    numbers.insert(new_i as usize, n);
}

fn coordinates(numbers: &[(usize, i64)]) -> i64 {
    let len = numbers.len();
    let zero_pos = numbers.iter().position(|&(_, n)| n == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|&i| numbers[(i + zero_pos) % len].1)
        .sum()
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut numbers = parse(input, 1);
        for i in 0..numbers.len() {
            mix(&mut numbers, i);
        }
        Some(coordinates(&numbers).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut numbers = parse(input, 811589153);
        for _ in 0..10 {
            for i in 0..numbers.len() {
                mix(&mut numbers, i);
            }
        }
        Some(coordinates(&numbers).to_string())
    }
}
