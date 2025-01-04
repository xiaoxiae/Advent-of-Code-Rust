use crate::util::Day;
use regex::Regex;
use std::collections::HashMap;

pub struct D10;

type Destination = (String, usize);

fn solve(input: &str) -> (usize, usize) {
    let value_regex = Regex::new(r"\d+").unwrap();
    let bot_regex = Regex::new(r".*?(?P<bot>\d+).*?to (?P<low_to>\w+).*?(?P<low_number>\d+).*?to (?P<high_to>\w+).*?(?P<high_number>\d+)").unwrap();

    let mut commands: HashMap<Destination, (Destination, Destination)> = HashMap::new();
    let mut storage: HashMap<Destination, Vec<usize>> = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 1;

    for line in input.trim().lines() {
        if line.starts_with("value") {
            let numbers = value_regex
                .captures_iter(line)
                .map(|a| a.get(0).unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            storage
                .entry((String::from("bot"), numbers[1]))
                .or_insert(vec![])
                .push(numbers[0]);
        } else if line.starts_with("bot") {
            let parts = bot_regex.captures(line).unwrap();

            let bot_id = parts["bot"].parse::<usize>().unwrap();

            let low_destination = (
                parts["low_to"].to_string(),
                parts["low_number"].parse::<usize>().unwrap(),
            );
            let high_destination = (
                parts["high_to"].to_string(),
                parts["high_number"].parse::<usize>().unwrap(),
            );

            commands.insert(
                (String::from("bot"), bot_id),
                (low_destination, high_destination),
            );
        } else {
            panic!("Invalid command {}", line);
        }
    }

    loop {
        let mut modified = false;

        let mut min = 0;
        let mut max = 0;
        let mut destination = &Destination::default();

        for (dest, values) in storage.iter_mut() {
            if values.len() == 2 {
                modified = true;

                let a = values.pop().unwrap();
                let b = values.pop().unwrap();

                min = a.min(b);
                max = a.max(b);

                destination = dest;

                if min == 17 && max == 61 {
                    part1 = dest.1;
                }

                break;
            }
        }

        if !modified {
            break;
        }

        let (lo, hi) = commands.get(destination).unwrap();

        storage.entry(lo.clone()).or_insert(vec![]).push(min);
        storage.entry(hi.clone()).or_insert(vec![]).push(max);
    }

    part2 = 1;
    for i in 0..3 {
        part2 *= storage.get(&(String::from("output"), i)).unwrap()[0]
    }

    (part1, part2)
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input).0.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input).1.to_string())
    }
}
