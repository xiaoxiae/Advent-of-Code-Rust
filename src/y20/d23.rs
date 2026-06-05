//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/23
use crate::util::Day;

pub struct D23;

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut cups: Vec<i64> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let max_cups = *cups.iter().max().unwrap();

        let mut current: i64 = 0;
        for _ in 0..100 {
            let mut taken: Vec<i64> = Vec::new();

            let mut take_from_pos = current + 1;
            for _ in 0..3 {
                take_from_pos = take_from_pos.rem_euclid(cups.len() as i64);
                taken.push(cups.remove(take_from_pos as usize));

                if take_from_pos == 0 {
                    current -= 1;
                }
            }

            let old_label = cups[current as usize];
            let mut new_label = old_label - 1;
            while !cups.contains(&new_label) {
                new_label = (new_label - 1).rem_euclid(max_cups + 1);
            }

            let i = cups.iter().position(|&x| x == new_label).unwrap();
            let mut new_cups: Vec<i64> = Vec::with_capacity(cups.len() + taken.len());
            new_cups.extend_from_slice(&cups[..i + 1]);
            new_cups.extend_from_slice(&taken);
            new_cups.extend_from_slice(&cups[i + 1..]);
            cups = new_cups;

            let pos = cups.iter().position(|&x| x == old_label).unwrap();
            current = ((pos + 1) % cups.len()) as i64;
        }

        let start = cups.iter().position(|&x| x == 1).unwrap();
        let mut result = String::new();
        for i in 0..cups.len() - 1 {
            result.push_str(&cups[(start + i + 1) % cups.len()].to_string());
        }

        Some(result)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut cups_int: Vec<usize> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize - 1)
            .collect();

        let first = cups_int[0];

        while cups_int.len() != 1_000_000 {
            cups_int.push(cups_int.len());
        }

        let max_cups = cups_int.iter().max().unwrap() + 1;

        // next[v] = value of the cup following cup with value v
        let mut next: Vec<usize> = vec![0; max_cups];
        let len = cups_int.len();
        for (i, &cup) in cups_int.iter().enumerate() {
            next[cup] = cups_int[(i + 1) % len];
        }

        let mut current = first;
        for _ in 0..10_000_000 {
            let t0 = next[current];
            let t1 = next[t0];
            let t2 = next[t1];
            let taken_values = [t0, t1, t2];

            next[current] = next[t2];

            let mut new_label = (current + max_cups - 1) % max_cups;
            while taken_values.contains(&new_label) {
                new_label = (new_label + max_cups - 1) % max_cups;
            }

            let new_next = next[new_label];
            next[new_label] = t0;
            next[t2] = new_next;

            current = next[current];
        }

        let a = next[0];
        let b = next[a];
        Some(((a + 1) * (b + 1)).to_string())
    }
}
