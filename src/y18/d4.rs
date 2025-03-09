use crate::util::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct D4;

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut data: Vec<&str> = input.lines().collect();
        data.sort();

        let re = Regex::new(r"#(\d+) ").unwrap();
        let mut guard_data: HashMap<u32, (u32, Vec<(u32, u32)>)> = HashMap::new();
        let mut i = 0;

        while i < data.len() {
            let guard_id: u32 = re.captures(data[i])?.get(1)?.as_str().parse().ok()?;

            let entry = guard_data.entry(guard_id).or_insert((0, Vec::new()));

            i += 1;
            while i < data.len() {
                if data[i].contains("falls asleep") {
                    let sleep_start: u32 = data[i][15..17].parse().ok()?;
                    let sleep_end: u32 = data[i + 1][15..17].parse().ok()?;

                    entry.0 += sleep_end - sleep_start;
                    entry.1.push((sleep_start, sleep_end));

                    i += 2;
                    continue;
                }
                break;
            }
        }

        let (&sleepiest_guard_id, _) = guard_data.iter().max_by_key(|(_, &(total, _))| total)?;

        let mut sleep_pattern = vec![0; 60];
        for &(start, end) in &guard_data[&sleepiest_guard_id].1 {
            for i in start..end {
                sleep_pattern[i as usize] += 1;
            }
        }

        let minute = sleep_pattern.iter().enumerate().max_by_key(|&(_, &v)| v)?.0 as u32;

        Some((minute * sleepiest_guard_id).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut data: Vec<&str> = input.lines().collect();
        data.sort();

        let re = Regex::new(r"#(\d+) ").unwrap();
        let mut guard_data: HashMap<u32, [u32; 60]> = HashMap::new();
        let mut i = 0;

        while i < data.len() {
            let guard_id: u32 = re.captures(data[i])?.get(1)?.as_str().parse().ok()?;

            let entry = guard_data.entry(guard_id).or_insert([0; 60]);

            i += 1;
            while i < data.len() {
                if data[i].contains("falls asleep") {
                    let sleep_start: u32 = data[i][15..17].parse().ok()?;
                    let sleep_end: u32 = data[i + 1][15..17].parse().ok()?;

                    for j in sleep_start..sleep_end {
                        entry[j as usize] += 1;
                    }

                    i += 2;
                    continue;
                }
                break;
            }
        }

        let (_, guard, sleepiest_minute) = guard_data
            .iter()
            .flat_map(|(&k, v)| v.iter().enumerate().map(move |(i, &val)| (val, k, i)))
            .max()?;

        Some((guard * sleepiest_minute as u32).to_string())
    }
}
