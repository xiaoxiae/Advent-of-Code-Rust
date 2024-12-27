use crate::util::Day;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

type Map = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (isize, isize);
type Distance = usize;

type DistanceResults = HashMap<(Position, Direction), Distance>;
type PathResults = HashMap<(Position, Direction), Vec<(Position, Direction)>>;

fn parse_input(input: &str) -> (Map, Position, Position) {
    let mut map = input
        .trim()
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect::<Map>();

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                'S' => {
                    start = (x, y);
                    map[y][x] = '.';
                }
                'E' => {
                    end = (x, y);
                    map[y][x] = '.';
                }
                _ => continue,
            }
        }
    }

    (map, start, end)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    position: Position,
    direction: Direction,
    distance: Distance,
}

impl State {
    fn is_forward_empty(&self, map: &Map) -> bool {
        let nx = self.position.0 as isize + self.direction.0;
        let ny = self.position.1 as isize + self.direction.1;

        match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some('.') => true,
            _ => false,
        }
    }

    fn turn(&self, map: &Map, sign: isize) -> Option<State> {
        let mut state = self.clone();
        state.direction = (sign * self.direction.1, -sign * self.direction.0);
        state.distance += 1000;

        if state.is_forward_empty(map) {
            Option::from(state)
        } else {
            None
        }
    }

    fn turn_left(&self, map: &Map) -> Option<State> {
        self.turn(map, -1)
    }

    fn turn_right(&self, map: &Map) -> Option<State> {
        self.turn(map, 1)
    }

    fn step(&self, map: &Map) -> Option<State> {
        if self.is_forward_empty(map) {
            Option::from(State {
                position: (
                    (self.position.0 as isize + self.direction.0) as usize,
                    (self.position.1 as isize + self.direction.1) as usize,
                ),
                direction: self.direction,
                distance: self.distance + 1,
            })
        } else {
            None
        }
    }

    fn next_states(&self, map: &Map) -> Vec<State> {
        [
            self.clone().step(map),
            self.clone().turn_left(map),
            self.clone().turn_right(map),
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect::<Vec<State>>()
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
    map: Map,
    start: Position,
    end: Position,
) -> (Option<Distance>, DistanceResults, PathResults) {
    let mut distances: DistanceResults = HashMap::default();
    let mut paths: PathResults = HashMap::default();

    let mut heap = BinaryHeap::new();

    distances.insert((start, (1, 0)), 0);
    paths.insert((start, (1, 0)), vec![]);

    heap.push(State {
        position: start,
        direction: (1, 0),
        distance: 0,
    });

    let mut end_distance = None;

    while let Some(state) = heap.pop() {
        if state.position == end {
            end_distance = Option::from(state.distance);
        }

        // Get the next possible positions and distances
        for next_state in state.next_states(&map) {
            // Skip processing if we have already found a shorter distance
            let best_distance = *distances
                .get(&(next_state.position, next_state.direction))
                .unwrap_or(&usize::MAX);

            // Skip states after finding the end distance
            if next_state.distance > end_distance.unwrap_or(usize::MAX) {
                continue;
            }

            // New/improved
            if next_state.distance < best_distance {
                distances.insert(
                    (next_state.position, next_state.direction),
                    next_state.distance,
                );
                paths.insert(
                    (next_state.position, next_state.direction),
                    Vec::from([(state.position, state.direction)]),
                );

                heap.push(next_state);
            } else if next_state.distance == best_distance {
                paths
                    .entry((next_state.position, next_state.direction))
                    .or_insert_with(Vec::new)
                    .push((state.position, state.direction));
            }
        }
    }

    (end_distance, distances, paths)
}

pub struct D16;

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (map, start, end) = parse_input(input);
        let (end_distance, _, _) = solve(map, start, end);

        Option::from(end_distance.unwrap().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (map, start, end) = parse_input(input);
        let (_, _, paths) = solve(map, start, end);

        let mut coordinates = HashSet::default();
        let mut deque =
            VecDeque::from([(end, (0, -1)), (end, (0, 1)), (end, (-1, 0)), (end, (1, 0))]);

        while let Some((pos, dir)) = deque.pop_front() {
            coordinates.insert(pos);

            for neighbor in paths.get(&(pos, dir)).unwrap_or(&vec![]) {
                deque.push_back(*neighbor);
            }
        }

        Option::from(coordinates.len().to_string())
    }
}
