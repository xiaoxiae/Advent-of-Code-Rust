use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use itertools::Itertools;
use regex::Regex;
use crate::util::Day;

type Position = (usize, usize);
type Distance = usize;

type Bytes = Vec<Position>;

fn parse_input(input: &str) -> Bytes {
    let re = Regex::new(r"\d+").unwrap();

    let numbers = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    numbers
        .chunks(2)
        .map(|parts| (parts[0], parts[1]))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    position: Position,
    distance: Distance,
}

impl State {
    fn next_states(&self, bytes: &[Position], width: usize, height: usize) -> Vec<State> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().map(|(dx, dy)| {
            let (x, y) = self.position;

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !(0 <= nx && nx < width as isize) || !(0 <= ny && ny < height as isize) {
                None
            } else {
                let p: Position = (nx as usize, ny as usize);

                if bytes.contains(&p) {
                    None
                } else {
                    Option::from(State {
                        position: p,
                        distance: self.distance + 1,
                    })
                }
            }
        }
        ).filter_map(|v| v).collect::<Vec<_>>()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // reverse the order to make it a min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(
    bytes: &Bytes,
    start: Position,
    end: Position,
    fallen_bytes: usize,
) -> Option<Distance> {
    let mut distances: HashMap<Position, Distance> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let width = end.0 + 1;
    let height = end.1 + 1;

    heap.push(State {
        position: start,
        distance: 0,
    });

    distances.insert(start, 0);

    while let Some(state) = heap.pop() {
        if state.position == end {
            return Option::from(state.distance);
        }

        for next_state in state.next_states(&bytes[..fallen_bytes], width, height) {
            // Skip processing if we have already found a shorter distance
            let best_distance = *distances
                .get(&next_state.position)
                .unwrap_or(&usize::MAX);

            // New/improved
            if next_state.distance < best_distance {
                distances.insert(next_state.position, next_state.distance);
                heap.push(next_state);
            }
        }
    }

    None
}

pub struct Y24D18;

impl Day for Y24D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bytes = parse_input(input);

        let width: usize;
        let height: usize;
        let fallen_bytes: usize;
        if bytes.len() <= 128 {  // for test inputs;
            width = 7;
            height = 7;
            fallen_bytes = 12;
        } else {
            width = 71;
            height = 71;
            fallen_bytes = 1_024;
        }

        let distance = solve(&bytes,
                             (0, 0),
                             (width - 1, height - 1),
                             fallen_bytes,
        );

        Option::from(distance.expect("No path found!").to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bytes = parse_input(input);

        let width: usize;
        let height: usize;
        if bytes.len() <= 128 {  // for test inputs;
            width = 7;
            height = 7;
        } else {
            width = 71;
            height = 71;
        }

        println!("{} {}");

        for fallen_bytes in 1..bytes.len() {
            if let None = solve(&bytes, (0, 0), (width - 1, height - 1), fallen_bytes) {

                return Option::from(format!("{},{}", bytes[fallen_bytes - 1].0, bytes[fallen_bytes - 1].1).to_string());
            }
        }

        panic!("Path is never fully blocked!");
    }
}
