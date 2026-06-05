//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/08
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D8;

fn parse(input: &str) -> (Vec<u8>, FxHashMap<String, (String, String)>) {
    let lines: Vec<&str> = input.lines().collect();
    let instructions: Vec<u8> = lines[0].trim().bytes().collect();

    let mut graph: FxHashMap<String, (String, String)> = FxHashMap::default();
    for line in &lines[2..] {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        // f, _, l, r = line.split()
        let f = parts[0].to_string();
        // l = l[1:-1]  (strip leading '(' and trailing ',')
        let l = parts[2][1..parts[2].len() - 1].to_string();
        // r = r[:-1]  (strip trailing ')')
        let r = parts[3][..parts[3].len() - 1].to_string();
        graph.insert(f, (l, r));
    }

    (instructions, graph)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (instructions, graph) = parse(input);

        let mut current = "AAA".to_string();
        let mut i: u64 = 0;
        while current != "ZZZ" {
            let idx = (i as usize) % instructions.len();
            let j = if instructions[idx] == b'L' { 0 } else { 1 };
            let pair = &graph[&current];
            current = if j == 0 { pair.0.clone() } else { pair.1.clone() };
            i += 1;
        }

        Some(i.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (instructions, graph) = parse(input);

        let mut current: Vec<String> = graph
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();

        let mut i: u64 = 0;
        let mut steps: Vec<u64> = Vec::new();
        while !current.is_empty() {
            let idx = (i as usize) % instructions.len();
            let j = if instructions[idx] == b'L' { 0 } else { 1 };
            i += 1;

            let mut k = 0;
            while k < current.len() {
                let pair = &graph[&current[k]];
                current[k] = if j == 0 { pair.0.clone() } else { pair.1.clone() };

                if current[k].ends_with('Z') {
                    current.remove(k);
                    steps.push(i);
                } else {
                    k += 1;
                }
            }
        }

        let result = steps.into_iter().fold(1u64, lcm);
        Some(result.to_string())
    }
}
