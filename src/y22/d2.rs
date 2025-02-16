use crate::util::Day;
use std::collections::HashMap;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let value = HashMap::from([("R", 1), ("P", 2), ("S", 3)]);
        let mapping = HashMap::from([
            ("A", "R"), ("B", "P"), ("C", "S"),
            ("X", "R"), ("Y", "P"), ("Z", "S"),
        ]);

        fn rps(a: &str, b: &str) -> i32 {
            match (a, b) {
                _ if a == b => 0,
                ("R", "S") | ("S", "P") | ("P", "R") => -1,
                _ => 1,
            }
        }

        let mut total = 0;
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let [a, b] = parts[..] {
                let a = mapping.get(a).unwrap();
                let b = mapping.get(b).unwrap();
                total += value.get(b).unwrap() + (rps(a, b) + 1) * 3;
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let value = HashMap::from([("R", 1), ("P", 2), ("S", 3)]);
        let outcome = HashMap::from([("X", -1), ("Y", 0), ("Z", 1)]);
        let mapping = HashMap::from([("A", "R"), ("B", "P"), ("C", "S")]);

        let win = HashMap::from([("R", "P"), ("P", "S"), ("S", "R")]);
        let lose = HashMap::from([("R", "S"), ("P", "R"), ("S", "P")]);

        let mut total = 0;
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let [a, o] = parts[..] {
                let a = mapping.get(a).unwrap();
                let o = outcome.get(o).unwrap();
                let rps = match o {
                    0 => a,
                    -1 => lose.get(a).unwrap(),
                    1 => win.get(a).unwrap(),
                    _ => unreachable!(),
                };

                total += value.get(rps).unwrap() + (o + 1) * 3;
            }
        }

        Some(total.to_string())
    }
}
