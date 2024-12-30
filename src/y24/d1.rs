use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D1;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut l, mut r) = parse_input(input);

        l.sort_unstable();
        r.sort_unstable();

        let differences: usize = l.iter().zip(r.iter()).map(|(x, y)| x.abs_diff(*y)).sum();

        Option::from(differences.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (l, r) = parse_input(input);

        let mut occurrences: HashMap<usize, usize> = HashMap::new();

        for &item in &r {
            *occurrences.entry(item).or_insert(0) += 1;
        }

        Option::from(
            l.iter()
                .map(|x| x * *occurrences.get(x).unwrap_or(&0))
                .sum::<usize>()
                .to_string(),
        )
    }

    /// --- Tom's Part 3 ---
    /// For occurrences of number i in list 1 and 2, calculate the distances between
    /// the respective pairings such that it is minimized. Return their sum.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let lists = parse_input(input);

        let mut occurrences: Vec<HashMap<usize, Vec<usize>>> = Vec::new();

        for list in [lists.0, lists.1] {
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

            let mut min_distance = usize::MAX;
            for offset in 0..(index_lists[1].len() - index_lists[0].len() + 1) {
                let distance: usize = index_lists[0]
                    .iter()
                    .zip(&index_lists[1][offset..index_lists[0].len() + offset])
                    .map(|(&a, &b)| a.abs_diff(b))
                    .sum();

                min_distance = usize::min(distance, min_distance);
            }

            distances += min_distance;
        }

        Option::from(distances.to_string())
    }
}
