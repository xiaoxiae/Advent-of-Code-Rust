use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use itertools::Itertools;
use crate::util::Day;

pub struct D13;

fn is_wall(x: usize, y: usize, favorite_number: usize) -> bool {
    let a = x * x + 3 * x + 2 * x * y + y + y * y + favorite_number;

    a.count_ones() % 2 == 1
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbouring_positions(&self) -> Vec<Position> {
        let mut positions = vec![];

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            positions.push(Position { x: self.x  + dx, y: self.y + dy });
        }

        positions
    }

    fn distance_to(&self, other: Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    position: Position,

    steps: usize,
    metric: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.metric.cmp(&self.metric)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let favorite_number = input.parse::<usize>().unwrap();

        let mut explored: HashSet<Position> = HashSet::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let start = Position { x: 1, y: 1 };
        let end = Position { x: 31, y: 39 };

        explored.insert(start.clone());

        heap.push(State {
            position: start.clone(),
            steps: 0,
            metric: start.distance_to(end.clone()),
        });

        while let Some(state) = heap.pop() {
            if state.position == end {
                return Option::from(state.steps.to_string());
            }

            for next_position in state.position.neighbouring_positions() {
                if next_position.x < 0 || next_position.y < 0 {
                    continue;
                }

                if is_wall(next_position.x as usize, next_position.y as usize, favorite_number) {
                    continue;
                }

                if explored.contains(&next_position) {
                    continue;
                }

                let next_state = State {
                    position: next_position.clone(),
                    steps: state.steps + 1,
                    metric: state.steps + 1 + next_position.distance_to(end.clone()),
                };

                explored.insert(next_position.clone());
                heap.push(next_state);
            }
        }

        unreachable!();
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let favorite_number = input.parse::<usize>().unwrap();

        let mut explored: HashSet<Position> = HashSet::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let start = Position { x: 1, y: 1 };

        explored.insert(start.clone());

        heap.push(State {
            position: start.clone(),
            steps: 0,
            metric: 0,
        });

        while let Some(state) = heap.pop() {
            if state.steps == 50 {
                continue;
            }

            for next_position in state.position.neighbouring_positions() {
                if next_position.x < 0 || next_position.y < 0 {
                    continue;
                }

                if is_wall(next_position.x as usize, next_position.y as usize, favorite_number) {
                    continue;
                }

                if explored.contains(&next_position) {
                    continue;
                }

                let next_state = State {
                    position: next_position.clone(),
                    steps: state.steps + 1,
                    metric: state.steps + 1,
                };

                explored.insert(next_position.clone());
                heap.push(next_state);
            }
        }

        Option::from(explored.len().to_string())
    }
}
