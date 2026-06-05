//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/15
use crate::util::Day;

pub struct D15;

fn hash(string: &str) -> usize {
    let mut current_value: usize = 0;
    for c in string.bytes() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total: usize = input.trim().split(',').map(hash).sum();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

        for part in input.trim().split(',') {
            let instruction = if part.ends_with('-') { '-' } else { '=' };
            let label: &str = if instruction == '-' {
                &part[..part.len() - 1]
            } else {
                &part[..part.len() - 2]
            };
            let box_idx = hash(label);

            if instruction == '-' {
                if let Some(i) = boxes[box_idx].iter().position(|(l, _)| l == label) {
                    boxes[box_idx].remove(i);
                }
            } else {
                let value: u32 = part[part.len() - 1..].parse().unwrap();
                if let Some(i) = boxes[box_idx].iter().position(|(l, _)| l == label) {
                    boxes[box_idx][i].1 = value;
                } else {
                    boxes[box_idx].push((label.to_string(), value));
                }
            }
        }

        let mut total: u64 = 0;
        for (i, b) in boxes.iter().enumerate() {
            for (j, (_, value)) in b.iter().enumerate() {
                total += (i as u64 + 1) * (j as u64 + 1) * (*value as u64);
            }
        }
        Some(total.to_string())
    }
}
