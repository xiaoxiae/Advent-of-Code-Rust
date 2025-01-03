use crate::util::Day;
use std::collections::HashSet;

pub struct D7;

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        'outer: for ip in input.trim().lines() {
            let chars = ip.chars().collect::<Vec<_>>();

            let mut i = 0;

            let mut in_square = false;
            let mut abba = false;

            loop {
                match chars[i..] {
                    ['[', ..] => in_square = true,
                    [']', ..] => in_square = false,
                    [a, b, c, d, ..] if a == d && a != b && b == c => {
                        if in_square {
                            continue 'outer;
                        } else {
                            abba = true;
                        }
                    }
                    [_, ..] => {}
                    [] => break,
                }

                i += 1;
            }

            if abba {
                total += 1;
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for ip in input.trim().lines() {
            let chars = ip.chars().collect::<Vec<_>>();

            let mut abas = HashSet::new();
            let mut babs = HashSet::new();

            let mut i = 0;
            let mut in_square = false;

            loop {
                match chars[i..] {
                    ['[', ..] => in_square = true,
                    [']', ..] => in_square = false,
                    [a, b, c, ..] if a == c && a != b => {
                        if in_square {
                            babs.insert((a, b));
                        } else {
                            abas.insert((b, a));
                        }
                    }
                    [_, ..] => {}
                    [] => break,
                }

                i += 1;
            }

            for a in abas {
                if babs.contains(&a) {
                    total += 1;
                    break;
                }
            }
        }

        Option::from(total.to_string())
    }
}
