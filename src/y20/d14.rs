//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/14
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D14;

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
        let ones: u64 = (1u64 << 36) - 1;
        let mut mask: Vec<char> = vec!['X'; 36];

        for inst in input.trim().lines() {
            let (opt, val) = inst.split_once(" = ").unwrap();

            if opt == "mask" {
                mask = val.chars().collect();
            } else {
                let index: u64 = opt.split('[').nth(1).unwrap().trim_end_matches(']').parse().unwrap();
                let mut number: u64 = val.parse().unwrap();

                for (i, c) in mask.iter().rev().enumerate() {
                    if *c != 'X' {
                        number = number & (ones ^ (1u64 << i)) | ((c.to_digit(10).unwrap() as u64) << i);
                    }
                }

                memory.insert(index, number);
            }
        }

        let total: u64 = memory.values().sum();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut memory: FxHashMap<u64, u64> = FxHashMap::default();
        let ones: u64 = (1u64 << 36) - 1;
        let mut mask: Vec<char> = vec!['X'; 36];

        for inst in input.trim().lines() {
            let (opt, val) = inst.split_once(" = ").unwrap();

            if opt == "mask" {
                mask = val.chars().collect();
            } else {
                let mut index: u64 = opt.split('[').nth(1).unwrap().trim_end_matches(']').parse().unwrap();
                let number: u64 = val.parse().unwrap();
                let mut floats: Vec<usize> = Vec::new();

                for (i, c) in mask.iter().rev().enumerate() {
                    if *c == '1' {
                        index = index & (ones ^ (1u64 << i)) | (1u64 << i);
                    } else if *c == 'X' {
                        floats.push(i);
                    }
                }

                for p in 0..(1u64 << floats.len()) {
                    let mut v = index;
                    for (bit_idx, &f) in floats.iter().enumerate() {
                        let b = (p >> bit_idx) & 1;
                        v = v & (ones ^ (1u64 << f)) | (b << f);
                    }
                    memory.insert(v, number);
                }
            }
        }

        let total: u64 = memory.values().sum();
        Some(total.to_string())
    }
}
