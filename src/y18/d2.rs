use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut two_times = 0;
        let mut three_times = 0;

        for line in input.lines() {
            let mut counts = HashMap::new();
            for c in line.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }

            if counts.values().any(|&v| v == 2) {
                two_times += 1;
            }
            if counts.values().any(|&v| v == 3) {
                three_times += 1;
            }
        }

        Some((two_times * three_times).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();

        for (i, line1) in lines.iter().enumerate() {
            for line2 in lines.iter().skip(i + 1) {
                let mut diff_count = 0;
                let mut common_chars = String::new();

                for (c1, c2) in line1.chars().zip(line2.chars()) {
                    if c1 != c2 {
                        diff_count += 1;
                        if diff_count > 1 {
                            break;
                        }
                    } else {
                        common_chars.push(c1);
                    }
                }

                if diff_count == 1 {
                    return Some(common_chars);
                }
            }
        }

        None
    }
}
