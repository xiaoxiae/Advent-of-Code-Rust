use crate::util::Day;
use crate::y17::d22::NodeState::{Clean, Flagged, Infected, Weakened};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::io::BufRead;

pub struct D22;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug)]
struct Virus {
    direction: usize,
    position: (isize, isize),
}

enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn parse(input: &str) -> HashSet<(isize, isize)> {
    let mut result = HashSet::default();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                result.insert((
                    x as isize - line.len() as isize / 2,
                    y as isize - line.len() as isize / 2,
                ));
            }
        }
    }

    result
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut map = parse(input);
        let mut virus = Virus {
            direction: 0,
            position: (0, 0),
        };

        let mut infection_count = 0;

        for _ in 0..10_000 {
            let infected = map.contains(&virus.position);

            if infected {
                virus.direction = (virus.direction + 1) % DIRECTIONS.len();
                map.remove(&virus.position);
            } else {
                virus.direction = (virus.direction + DIRECTIONS.len() - 1) % DIRECTIONS.len();
                map.insert(virus.position);
                infection_count += 1;
            }

            virus.position = (
                virus.position.0 + DIRECTIONS[virus.direction].0,
                virus.position.1 + DIRECTIONS[virus.direction].1,
            );
        }

        Option::from(infection_count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut map: HashMap<(isize, isize), NodeState> =
            parse(input).into_iter().map(|k| (k, Infected)).collect();

        let mut virus = Virus {
            direction: 0,
            position: (0, 0),
        };

        let mut infection_count = 0;

        for _ in 0..10_000_000 {
            let state = map.entry(virus.position).or_insert(Clean);

            match state {
                Clean => {
                    virus.direction = (virus.direction + DIRECTIONS.len() - 1) % DIRECTIONS.len();
                    *state = Weakened;
                }
                Weakened => {
                    infection_count += 1;
                    *state = Infected;
                }
                Infected => {
                    virus.direction = (virus.direction + 1) % DIRECTIONS.len();
                    *state = Flagged;
                }
                Flagged => {
                    virus.direction = (virus.direction + 2) % DIRECTIONS.len();
                    *state = Clean;
                }
            }

            virus.position = (
                virus.position.0 + DIRECTIONS[virus.direction].0,
                virus.position.1 + DIRECTIONS[virus.direction].1,
            );
        }

        Option::from(infection_count.to_string())
    }
}
