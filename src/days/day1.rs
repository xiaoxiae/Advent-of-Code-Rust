use crate::util::Day;
use std::collections::HashMap;

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

        Option::from(
            lists[0]
                .iter()
                .map(|x| x * *occurrences.get(x).unwrap_or(&0))
                .sum::<i32>()
                .to_string(),
        )
    }

    /// --- Tom's Part 3 ---
    /// For occurrences of number i in list 1 and 2, calculate the distances between
    /// the respective pairings such that it is minimized. Return their sum.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let lists = parse_input(input);

        let mut occurrences: Vec<HashMap<i32, Vec<usize>>> = Vec::new();

        for list in lists.iter() {
            let mut o = HashMap::new();

            for (i, &item) in list.iter().enumerate() {
                o.entry(item).or_insert(Vec::new()).push(i);
            }

            occurrences.push(o);
        }

        let mut distances = 0;
        for (key, i1) in &occurrences[0] {
            let i2;
            match occurrences[1].get(key) {
                None => continue,
                Some(i) => i2 = i,
            }

            let mut index_lists = vec![i1, i2];
            index_lists.sort_by(|a, b| a.len().cmp(&b.len()));

            let mut min_distance = i32::MAX;
            for offset in 0..(index_lists[1].len() - index_lists[0].len() + 1) {
                let distance: i32 = index_lists[0]
                    .iter()
                    .zip(&index_lists[1][offset..index_lists[0].len() + offset])
                    .map(|(&a, &b)| (a as i32 - b as i32).abs())
                    .sum();

                min_distance = i32::min(distance, min_distance);
            }

            distances += min_distance;
        }

        Option::from(distances.to_string())
    }
}
