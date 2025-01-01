use crate::util::Day;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let groups: Vec<i32> = input
            .split("\n\n")
            .map(|group| group.lines().filter_map(|line| line.parse::<i32>().ok()).sum())
            .collect();

        groups.into_iter().max().map(|max| max.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut groups: Vec<i32> = input
            .split("\n\n")
            .map(|group| group.lines().filter_map(|line| line.parse::<i32>().ok()).sum())
            .collect();

        groups.sort_unstable();

        Some(groups.iter().rev().take(3).sum::<i32>().to_string())
    }
}
