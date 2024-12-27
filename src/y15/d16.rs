use crate::util::Day;
use std::collections::HashMap;

pub struct D16;

fn parse(input: &str) -> Vec<HashMap<String, usize>> {
    input
        .lines()
        .map(|line| {
            let (_, values) = line.split_once(": ").unwrap();
            let mut hashmap: HashMap<String, usize> = HashMap::new();

            for part in values.split(", ") {
                let (key, value) = part.split_once(": ").unwrap();

                hashmap.insert(key.to_string(), value.trim().parse::<usize>().unwrap());
            }

            hashmap
        })
        .collect()
}

fn solve(aunts: Vec<HashMap<String, usize>>, outdated: bool) -> usize {
    let requirements: HashMap<String, usize> = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    let (i, _) = aunts.iter().enumerate().find(|(_, aunt)| {
        for (key, value) in *aunt {
            if match key.as_str() {
                "cats" | "trees" if outdated => requirements[key] >= *value,
                "pomeranians" | "goldfish" if outdated => requirements[key] <= *value,
                _ => requirements[key] != *value,
            } {
                return false;
            }
        }

        true
    }).unwrap();

    i + 1
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let aunts = parse(input);

        Option::from(solve(aunts, false).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let aunts = parse(input);

        Option::from(solve(aunts, true).to_string())
    }
}
