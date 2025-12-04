use crate::util::Day;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for line in input.lines() {
            let mut max_left = 0;
            let mut max = 0;

            for d in line.chars().map(|c| c.to_digit(10).unwrap()) {
                max = max.max(max_left * 10 + d);
                max_left = max_left.max(d);
            }

            total += max;
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total: usize = 0;

        for line in input.lines() {
            let mut batteries = Vec::new();

            'outer: for d in line.chars().map(|c| c.to_digit(10).unwrap() as usize) {
                if (batteries.len() != 12) {
                    batteries.push(d);
                    continue;
                }

                // try to remove a battery such that it increases the number
                for i in 0..batteries.len() - 1 {
                    if batteries[i] < batteries[i + 1] {
                        batteries.remove(i);
                        batteries.push(d);
                        continue 'outer;
                    }
                }

                // if not, remember the number so we can use it to replace stuff later
                if batteries[batteries.len() - 1] < d {
                    batteries.pop();
                    batteries.push(d);
                }
            }

            let mut number = 0;
            for d in batteries {
                number = number * 10 + d;
            }

            total += number;
        }

        Option::from(total.to_string())
    }
}
