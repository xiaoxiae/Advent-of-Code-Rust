use std::num::ParseIntError;
use crate::util::Day;
use rayon::prelude::*;

pub struct Y24D25;

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys: Vec<_> = vec![];
    let mut locks: Vec<_> = vec![];

    for key_or_lock in input.split("\n\n") {
        let lines: Vec<&str> = key_or_lock.lines().collect();

        let char = lines[0].chars().next().unwrap();

        let mut item: Vec<usize> = vec![];

        for column in 0..lines[0].len() {
            let mut height = 0;
            for row in 0..lines.len() {
                if lines[row].chars().nth(column).unwrap() != char {
                    break;
                }

                height += 1;
            }

            match char {
                '#' => {
                    height -= 1;
                }
                '.' => {
                    height = lines.len() - height - 1;
                }
                _ => panic!("Invalid character!")
            }

            item.push(height);
        }

        match char {
            '#' => keys.push(item),
            '.' => locks.push(item),
            _ => panic!("Invalid character!")
        }
    }

    (keys, locks)
}

impl Day for Y24D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (keys, locks) = parse(input);

        let mut matches = 0;
        for key in &keys {
            for lock in &locks {
                if key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5) {
                    matches += 1;
                }
            }
        }

        Option::from(matches.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        match input.parse::<usize>() {
            Ok(49) => Option::from("<3".to_string()),
            _ => None,
        }
    }
}
