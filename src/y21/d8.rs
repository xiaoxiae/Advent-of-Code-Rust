//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/08
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D8;

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn sorted_str(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (_pre, post) = line.split_once(" | ").unwrap();
            total += post
                .split_whitespace()
                .filter(|part| matches!(part.len(), 2 | 4 | 3 | 7))
                .count();
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total: u64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (pre_raw, post_raw) = line.split_once(" | ").unwrap();

            let mut pre: Vec<&str> = pre_raw.split_whitespace().collect();
            pre.sort_by_key(|x| x.len());
            let post: Vec<&str> = post_raw.split_whitespace().collect();

            let mut mapping: FxHashMap<char, char> = FxHashMap::default();

            let cf: Vec<char> = pre[0].chars().collect();
            let cfa: Vec<char> = pre[1].chars().collect();

            // a
            for &a in &cfa {
                if !cf.contains(&a) {
                    mapping.insert('a', a);
                }
            }

            // c f
            for part in &pre {
                if part.len() != 6 {
                    continue;
                }

                let c = cf[0];
                let f = cf[1];

                let c_in = part.contains(c);
                let f_in = part.contains(f);

                if c_in && !f_in {
                    mapping.insert('f', c);
                    mapping.insert('c', f);
                } else if !c_in && f_in {
                    mapping.insert('f', f);
                    mapping.insert('c', c);
                }
            }

            let mut dg: Vec<char> = Vec::new();
            for part in &pre {
                if part.len() != 5 {
                    continue;
                }

                if part.contains(mapping[&'c'])
                    && part.contains(mapping[&'f'])
                    && part.contains(mapping[&'a'])
                {
                    dg = part.chars().filter(|p| !cfa.contains(p)).collect();
                }
            }

            // d g       in 4
            if pre[2].contains(dg[0]) {
                mapping.insert('d', dg[0]);
                mapping.insert('g', dg[1]);
            } else {
                mapping.insert('d', dg[1]);
                mapping.insert('g', dg[0]);
            }

            // b
            for p in pre[2].chars() {
                if !mapping.values().any(|&v| v == p) {
                    mapping.insert('b', p);
                }
            }

            // e
            for p in pre[pre.len() - 1].chars() {
                if !mapping.values().any(|&v| v == p) {
                    mapping.insert('e', p);
                }
            }

            let mut inverse_mapping: FxHashMap<char, char> = FxHashMap::default();
            for (&k, &v) in &mapping {
                inverse_mapping.insert(v, k);
            }

            let mut number: u64 = 0;
            for p in &post {
                let p_mapped: String = p.chars().map(|c| inverse_mapping[&c]).collect();
                let key = sorted_str(&p_mapped);
                let idx = DIGITS.iter().position(|&d| d == key).unwrap() as u64;
                number = number * 10 + idx;
            }

            total += number;
        }

        Some(total.to_string())
    }
}
