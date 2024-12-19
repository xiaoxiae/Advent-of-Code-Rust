use crate::util::Day;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (stripes, patterns) = input.split_once("\n\n").unwrap();

    (
        stripes.split(", ").collect::<Vec<_>>(),
        patterns.split_whitespace().collect::<Vec<_>>(),
    )
}

fn count_recursive(current: &str, patterns: &Vec<&str>, cache: &mut Vec<usize>) -> usize {
    if current.len() == 0 {
        return 1;
    }

    if cache[current.len()] != usize::MAX {
        return cache[current.len()];
    }

    let mut total = 0;
    for &p in patterns {
        if current.starts_with(p) {
            total += count_recursive(&current[p.len()..], patterns, cache);
        }
    }

    cache[current.len()] = total;

    total
}

pub struct Y24D19;

impl Day for Y24D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (stripes, patterns) = parse_input(input);

        let r = format!("^({})+$", stripes.join("|"));
        let regex = Regex::new(r.as_str()).unwrap();

        let count = patterns
            .into_iter()
            .filter(|l| regex.is_match(l))
            .count();

        Option::from(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (stripes, patterns) = parse_input(input);

        let count: usize = patterns
            .par_iter()
            .map(|&l| {
                let mut cache = vec![usize::MAX; l.len() + 1];

                count_recursive(l, &stripes, &mut cache)
            })
            .sum();

        Option::from(count.to_string())
    }
}
