use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut position = 50;
        let mut zero_meetings = 0;

        for line in input.split_ascii_whitespace() {
            let mut chars = line.chars();

            let sign = match chars.next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => continue,
            };

            let steps = chars.collect::<String>().parse::<isize>().unwrap() * sign;

            position = (position + steps) % 100;

            if position == 0 {
                zero_meetings += 1;
            }
        }

        Option::from(zero_meetings.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut position = 50;
        let mut zero_meetings = 0;

        for line in input.split_ascii_whitespace() {
            let mut chars = line.chars();

            let sign = match chars.next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => continue,
            };

            let mut steps = chars.collect::<String>().parse::<isize>().unwrap() * sign;

            zero_meetings += steps.abs() / 100;
            steps %= 100;

            if position != 0 {
                match sign {
                    -1 => {
                        if steps.abs() >= position {
                            zero_meetings += 1;
                        }
                    }
                    1 => {
                        if steps >= 100 - position {
                            zero_meetings += 1;
                        }
                    }
                    _ => {}
                }
            }

            position = (position + steps + 100) % 100;
        }

        Option::from(zero_meetings.to_string())
    }
}
