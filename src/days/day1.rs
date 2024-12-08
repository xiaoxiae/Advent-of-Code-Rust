use std::collections::HashMap;
use crate::util::Day;

pub struct Day1;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let parts: Vec<Vec<i32>> = input
        .trim()
        .split('\n')
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    (0..parts[0].len())
        .map(|col| parts.iter().map(|row| row[col]).collect())
        .collect()
}

impl Day for Day1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut lists = parse_input(input);

        lists[0].sort();
        lists[1].sort();

        let differences: i32 = lists[0]
            .iter()
            .zip(lists[1].iter())
            .map(|(x, y)| (x - y).abs())
            .sum();

        Option::from(differences.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lists = parse_input(input);

        let mut occurrences: HashMap<i32, i32> = HashMap::new();

        for &item in &lists[1] {
            *occurrences.entry(item).or_insert(0) += 1;
        }

        Option::from(lists[0].iter()
            .map(|x| x * *occurrences.get(x).unwrap_or(&0))
            .sum::<i32>()
            .to_string())
    }
}
