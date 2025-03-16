use crate::util::Day;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use rustc_hash::FxHashMap as HashMap;

use Equipment::*;
use Terrain::*;

pub struct D22;

#[derive(Clone)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Equipment {
    Neither,
    Torch,
    Gear,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Terrain {
    fn level(&self) -> usize {
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }
}

impl Equipment {
    fn accessible(&self, terrain: &Terrain) -> bool {
        match (terrain, self) {
            (Rocky, Neither) | (Wet, Torch) | (Narrow, Gear) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    position: Position,
    equipment: Equipment,
}

#[derive(Debug, Eq, PartialEq)]
struct QueueEntry {
    cost: usize,
    heuristic: usize,
    state: State,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic)) // Reverse order for min-heap
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    fn distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl State {
    fn neighbours(&self, map: &Vec<Vec<Terrain>>) -> Vec<(State, usize)> {
        let mut states = vec![];

        let current_terrain = &map[self.position.y][self.position.x];

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nx = self.position.x.wrapping_add_signed(dx);
            let ny = self.position.y.wrapping_add_signed(dy);

            if let Some(terrain) = map.get(ny).and_then(|r| r.get(nx)) {
                if self.equipment.accessible(terrain) {
                    // if we can go to the terrain without swapping, do so
                    // (it's never useful to swap when we can just go, since we can always do it later)
                    states.push((
                        State {
                            position: Position { x: nx, y: ny },
                            equipment: self.equipment,
                        },
                        1,
                    ))
                } else {
                }
            }
        }

        // if not, generate states for the equipment that we can switch to and then transfer
        for new_equipment in [Neither, Torch, Gear] {
            if new_equipment != self.equipment && new_equipment.accessible(current_terrain) {
                states.push((
                    State {
                        position: Position {
                            x: self.position.x,
                            y: self.position.y,
                        },
                        equipment: new_equipment,
                    },
                    7,
                ));
            }
        }

        states
    }
}

fn parse(input: &str) -> (usize, Position) {
    let mut lines = input.lines();

    let depth = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let target = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|part| part.parse::<usize>().unwrap())
        .collect_tuple::<(usize, usize)>()
        .unwrap();

    (
        depth,
        Position {
            x: target.0,
            y: target.1,
        },
    )
}

fn get_map(depth: usize, target: Position, offset: usize) -> Vec<Vec<Terrain>> {
    let mut erosion = vec![vec![0; target.x + 1 + offset]; target.y + 1 + offset];
    let mut map = vec![vec![Rocky; target.x + 1 + offset]; target.y + 1 + offset];

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let mut value;
            if (x == 0 && y == 0) || (x == target.x && y == target.y) {
                value = 0;
            } else if x == 0 {
                value = y * 48271;
            } else if y == 0 {
                value = x * 16807;
            } else {
                value = erosion[y - 1][x] * erosion[y][x - 1];
            }

            value = (value + depth) % 20183;

            erosion[y][x] = value;
            map[y][x] = if value % 3 == 0 {
                Rocky
            } else if value % 3 == 1 {
                Wet
            } else {
                Narrow
            };
        }
    }

    map
}

fn solve(start: State, end: State, map: &Vec<Vec<Terrain>>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::default();

    heap.push(QueueEntry {
        cost: 0,
        heuristic: start.position.distance(&end.position),
        state: start,
    });
    distances.insert(start, 0);

    while let Some(QueueEntry { cost, state, .. }) = heap.pop() {
        if state == end {
            return Some(cost);
        }

        for (neighbor, move_cost) in state.neighbours(map) {
            let new_cost = cost + move_cost;

            if !distances.contains_key(&neighbor) || new_cost < distances[&neighbor] {
                let heuristic = neighbor.position.distance(&end.position);

                distances.insert(neighbor, new_cost);
                heap.push(QueueEntry {
                    cost: new_cost,
                    heuristic,
                    state: neighbor,
                });
            }
        }
    }

    None
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (depth, target) = parse(input);
        let map = get_map(depth, target, 0);

        let mut risk = 0;

        for x in 0..=target.x {
            for y in 0..=target.y {
                risk += map[y][x].level()
            }
        }

        Option::from(risk.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (depth, target) = parse(input);
        let map = get_map(depth, target, 100); // a bit yucky

        let result = solve(
            State {
                position: Position { x: 0, y: 0 },
                equipment: Torch,
            },
            State {
                position: target,
                equipment: Torch,
            },
            &map,
        );

        Option::from(result.unwrap().to_string())
    }
}