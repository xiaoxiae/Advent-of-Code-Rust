use crate::util::Day;
use std::collections::HashSet;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let sum: i32 = input.lines().filter_map(|line| line.parse::<i32>().ok()).sum();
        Some(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let numbers: Vec<i32> = input.lines().filter_map(|line| line.parse::<i32>().ok()).collect();
        let mut frequencies = HashSet::new();
        let mut sum = 0;

        for num in numbers.iter().cycle() {
            if !frequencies.insert(sum) {
                return Some(sum.to_string());
            }
            sum += num;
        }

        None
    }
}
