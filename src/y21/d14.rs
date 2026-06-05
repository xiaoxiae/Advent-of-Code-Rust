//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/14
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D14;

fn solve(input: &str, steps: usize) -> i64 {
    let input = input.trim();
    let (template, rules_str) = input.split_once("\n\n").unwrap();

    let mut rules: FxHashMap<(u8, u8), u8> = FxHashMap::default();
    for rule in rules_str.lines() {
        let (a, b) = rule.split_once(" -> ").unwrap();
        let ab = a.as_bytes();
        rules.insert((ab[0], ab[1]), b.as_bytes()[0]);
    }

    let tb = template.as_bytes();

    let mut rule_counts: FxHashMap<(u8, u8), i64> = FxHashMap::default();
    for w in tb.windows(2) {
        *rule_counts.entry((w[0], w[1])).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_rule_counts: FxHashMap<(u8, u8), i64> = FxHashMap::default();
        for (&(x, y), &cnt) in &rule_counts {
            let m = rules[&(x, y)];
            *new_rule_counts.entry((x, m)).or_insert(0) += cnt;
            *new_rule_counts.entry((m, y)).or_insert(0) += cnt;
        }
        rule_counts = new_rule_counts;
    }

    let mut quantity: FxHashMap<u8, i64> = FxHashMap::default();
    for (&(a, b), &c) in &rule_counts {
        *quantity.entry(a).or_insert(0) += c;
        *quantity.entry(b).or_insert(0) += c;
    }

    // all elements except the first and last are counted twice
    *quantity.entry(tb[0]).or_insert(0) += 1;
    *quantity.entry(tb[tb.len() - 1]).or_insert(0) += 1;

    for v in quantity.values_mut() {
        *v /= 2;
    }

    let min_val = *quantity.values().min().unwrap();
    let max_val = *quantity.values().max().unwrap();

    max_val - min_val
}

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(solve(input, 10).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(solve(input, 40).to_string())
    }
}
