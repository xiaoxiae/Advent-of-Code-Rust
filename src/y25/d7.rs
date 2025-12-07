use rustc_hash::{FxHashMap, FxHashSet};

use crate::util::Day;

pub struct D7;

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut start = 0;
        let mut height = 0;
        let mut positions = FxHashSet::default();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = x;
                    }
                    '^' => {
                        positions.insert((x, y));
                    }
                    _ => continue,
                }
            }

            height = y;
        }

        let mut beams = FxHashSet::default();
        beams.insert(start);

        let mut split = 0;
        let mut y = 0;

        while y < height {
            let mut new_beams = FxHashSet::default();

            for x in beams {
                if positions.contains(&(x, y)) {
                    new_beams.insert((x - 1));
                    new_beams.insert((x + 1));
                    split += 1;
                } else {
                    new_beams.insert((x));
                }
            }

            beams = new_beams;
            y += 1;
        }

        Option::from(split.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut start = 0;
        let mut height = 0;
        let mut positions = FxHashSet::default();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = x;
                    }
                    '^' => {
                        positions.insert((x, y));
                    }
                    _ => continue,
                }
            }

            height = y;
        }

        let mut beams = FxHashMap::default();
        beams.insert(start, 1usize);

        let mut y = 0;

        while y < height {
            let mut new_beams = FxHashMap::default();

            for (x, count) in beams {
                if positions.contains(&(x, y)) {
                    new_beams
                        .entry(x - 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);

                    new_beams
                        .entry(x + 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                } else {
                    new_beams
                        .entry(x)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }

            beams = new_beams;
            y += 1;
        }

        // Option::from(split.to_string())
        Option::from(beams.iter().fold(0, |acc, (_, v)| acc + v).to_string())
    }
}
