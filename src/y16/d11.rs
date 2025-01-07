use crate::util::Day;
use regex::Regex;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap};

const MAX_FLOOR: usize = 3;

pub struct D11;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Configuration {
    floor: usize,

    // the integer is separated into sections of 4 bits, alternating chip/generator
    positions: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    configuration: Configuration,

    steps: usize,
    metric: isize,
}

impl Configuration {
    fn is_end(&self, floor_pattern: usize) -> bool {
        self.positions == (floor_pattern << MAX_FLOOR)
    }

    fn is_valid(&self, generator_pattern: usize) -> bool {
        let mut microchip_pattern = generator_pattern >> 4;

        // check all microchips
        while microchip_pattern != 0 {
            let element = microchip_pattern.trailing_zeros() as usize;

            microchip_pattern &= !(1 << element);

            let floor = (self.positions >> (element)) % 16;
            let floor_number = floor.trailing_zeros() as usize;

            // has a generator
            if self.positions & (floor << (4 + element)) != 0 {
                continue;
            }

            // otherwise if there is a different generator, return false
            if self.positions & (generator_pattern << floor_number) != 0 {
                return false;
            }
        }

        true
    }

    fn next_configurations(&self, floor_pattern: usize, generator_pattern: usize) -> Vec<Configuration> {
        let mut configurations = vec![];

        let valid_positions = self.positions & (floor_pattern << self.floor);

        for d in [-1isize, 1isize] {
            // don't go over
            if (d == -1 && self.floor == 0) || (d == 1 && self.floor == MAX_FLOOR) {
                continue;
            }

            let mut i_positions = valid_positions;
            while i_positions != 0 {
                let mut i_position = i_positions.trailing_zeros() as usize;

                let mut j_positions = i_positions;
                i_positions &= !(1 << i_position);

                while j_positions != 0 {
                    let mut j_position = j_positions.trailing_zeros() as usize;
                    j_positions &= !(1 << j_position);

                    let mut new_positions = self.positions;
                    new_positions &= !(1 << i_position);
                    new_positions &= !(1 << j_position);

                    new_positions |= 1 << ((i_position as isize + d) as usize);
                    new_positions |= 1 << ((j_position as isize + d) as usize);

                    let configuration = Configuration {
                        floor: (self.floor as isize + d) as usize,
                        positions: new_positions,
                    };

                    if configuration.is_valid(generator_pattern) {
                        configurations.push(configuration);
                    }
                }
            }
        }

        configurations
    }
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

fn get_floor_pattern(elements: usize) -> usize {
    let mut pattern = 0;

    for i in 0..(elements * 2) {
        pattern |= 1 << (i * 4);
    }

    pattern
}

fn get_generator_pattern(elements: usize) -> usize {
    let mut pattern = 0;

    for i in 0..elements {
        pattern |= 1 << (i * 8 + 4);
    }

    pattern
}


fn solve(start: Configuration, elements: usize) -> usize {
    let mut distances = HashSet::default();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    distances.insert(start.clone());

    heap.push(State {
        configuration: start.clone(),
        steps: 0,
        metric: 0,
    });

    let floor_pattern = get_floor_pattern(elements);
    let generator_pattern = get_generator_pattern(elements);

    while let Some(state) = heap.pop() {
        // state.configuration.pprint();

        if state.configuration.is_end(floor_pattern) {
            return state.steps;
        }

        for next_configuration in state.configuration.next_configurations(floor_pattern, generator_pattern) {
            if distances.contains(&next_configuration) {
                continue;
            }

            let next_distance = state.steps as isize + 1;

            let next_state = State {
                configuration: next_configuration.clone(),
                steps: state.steps + 1,
                metric: next_distance,
            };

            distances.insert(next_configuration);
            heap.push(next_state);
        }
    }

    unreachable!()
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"(\b\w+\b)(?:-compatible)? (generator|microchip)").unwrap();

        let mut element_count = 0;
        let mut element_map: HashMap<String, usize> = HashMap::default();

        let mut configuration: usize = 0;

        for (floor, line) in input.trim().lines().enumerate() {
            for cap in re.captures_iter(line) {
                let material_id = element_map.entry(cap[1].to_string()).or_insert_with(|| {
                    element_count += 1;

                    element_count - 1
                });

                match &cap[2] {
                    "microchip" => configuration |= 1 << (*material_id * 8 + floor),
                    "generator" => configuration |= 1 << (*material_id * 8 + 4 + floor),
                    _ => unreachable!(),
                }
            }
        }

        let state = Configuration {
            floor: 0,
            positions: configuration,
        };

        let steps = solve(state, element_count);

        Option::from(steps.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"(\b\w+\b)(?:-compatible)? (generator|microchip)").unwrap();

        let mut element_count = 0;
        let mut element_map: HashMap<String, usize> = HashMap::default();

        let mut configuration: usize = 0;

        for (floor, line) in input.trim().lines().enumerate() {
            let mut line = line.to_string();
            if floor == 0 {
                line.push_str("An elerium generator. An elerium-compatible microchip. A dilithium generator. A dilithium-compatible microchip.")
            }

            for cap in re.captures_iter(&*line) {
                let material_id = element_map.entry(cap[1].to_string()).or_insert_with(|| {
                    element_count += 1;

                    element_count - 1
                });

                match &cap[2] {
                    "microchip" => configuration |= 1 << (*material_id * 8 + floor),
                    "generator" => configuration |= 1 << (*material_id * 8 + 4 + floor),
                    _ => unreachable!(),
                }
            }
        }

        let state = Configuration {
            floor: 0,
            positions: configuration,
        };

        let steps = solve(state, element_count);

        Option::from(steps.to_string())
    }
}
