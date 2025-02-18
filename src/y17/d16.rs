use crate::util::Day;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub struct D16;

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut programs = (0..16).map(|v| (v + b'a') as char).collect::<VecDeque<_>>();

        for command in input.trim().split(',') {
            match command.as_bytes()[0] {
                b's' => {
                    let n = command[1..].parse::<usize>().unwrap();

                    for _ in 0..n {
                        let p = programs.pop_back().unwrap();
                        programs.push_front(p);
                    }
                }
                b'x' => {
                    let (i, j) = command[1..]
                        .split("/")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap();

                    programs.swap(i, j);
                }
                b'p' => {
                    let (c, d) = command[1..]
                        .split("/")
                        .map(|v| v.chars().next().unwrap())
                        .collect_tuple::<(char, char)>()
                        .unwrap();

                    let i = programs.iter().position(|&x| x == c).unwrap();
                    let j = programs.iter().position(|&x| x == d).unwrap();

                    programs.swap(i, j);
                }
                _ => unreachable!(),
            }
        }

        Option::from(programs.iter().collect::<String>())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut programs = (0..16).map(|v| (v + b'a') as char).collect::<VecDeque<_>>();

        let mut seen = HashMap::new();
        let mut i = 0;
        let mut remaining = 1_000_000_000;

        while remaining > 0 {
            for command in input.trim().split(',') {
                match command.as_bytes()[0] {
                    b's' => {
                        let n = command[1..].parse::<usize>().unwrap();

                        for _ in 0..n {
                            let p = programs.pop_back().unwrap();
                            programs.push_front(p);
                        }
                    }
                    b'x' => {
                        let (i, j) = command[1..]
                            .split("/")
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect_tuple::<(usize, usize)>()
                            .unwrap();

                        programs.swap(i, j);
                    }
                    b'p' => {
                        let (c, d) = command[1..]
                            .split("/")
                            .map(|v| v.chars().next().unwrap())
                            .collect_tuple::<(char, char)>()
                            .unwrap();

                        let i = programs.iter().position(|&x| x == c).unwrap();
                        let j = programs.iter().position(|&x| x == d).unwrap();

                        programs.swap(i, j);
                    }
                    _ => unreachable!(),
                }
            }

            i += 1;
            remaining -= 1;

            let key = programs.iter().collect::<String>();

            if let Some(&first) = seen.get(&key) {
                let period = i - first;
                remaining = remaining - (remaining / period) * period;
            } else {
                seen.insert(key.clone(), i);
            }
        }

        Option::from(programs.iter().collect::<String>())
    }
}
