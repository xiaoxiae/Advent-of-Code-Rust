use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D4;

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;
        for room in input.lines() {
            let parts: Vec<&str> = room.split('-').collect();
            let last_part = parts.last().unwrap();
            let sector_id: i32 = last_part[..last_part.find('[').unwrap()].parse().unwrap();
            let checksum = &last_part[last_part.find('[').unwrap() + 1..last_part.len() - 1];

            let mut freq: HashMap<char, usize> = HashMap::new();
            for c in parts[..parts.len() - 1].join("").chars() {
                *freq.entry(c).or_insert(0) += 1;
            }

            let encrypted_letters: Vec<char> = freq.iter()
                .sorted_by(|&(a_key, a_val), &(b_key, b_val)| {
                    b_val.cmp(a_val).then_with(|| a_key.cmp(b_key))
                })
                .map(|(k, _)| *k)
                .take(5)
                .collect();

            let checksum_sorted: Vec<char> = checksum.chars().collect();
            if encrypted_letters == checksum_sorted {
                total += sector_id;
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        for room in input.lines() {
            let parts: Vec<&str> = room.split('-').collect();
            let last_part = parts.last().unwrap();
            let sector_id: i32 = last_part[..last_part.find('[').unwrap()].parse().unwrap();

            let name = parts[..parts.len() - 1].join(" ");
            let real_name: String = name.chars()
                .map(|c| {
                    if c == ' ' {
                        ' '
                    } else {
                        (((c as u8 - b'a') as i32 + sector_id) % 26 + b'a' as i32) as u8 as char
                    }
                })
                .collect();

            if real_name.contains("north") {
                return Some(sector_id.to_string());
            }
        }
        None
    }
}
