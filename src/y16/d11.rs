use crate::util::Day;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct D11;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Configuration {
    elevator: isize,

    // [0] are generators, [1] are microchips
    positions: Vec<Vec<isize>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct State {
    configuration: Configuration,
    steps: usize,
}

impl Configuration {
    fn is_end(&self) -> bool {
        self.positions.iter().all(|row| row.iter().all(|v| *v == 3))
    }

    fn is_valid(&self) -> bool {
        // if the state contains a chip and has j
        for (i, chip_floor) in self.positions[1].iter().enumerate() {
            // has generator, no need to check
            if self.positions[0][i] == *chip_floor {
                continue;
            }

            // sees the wrong generator
            if self.positions[0].contains(chip_floor) {
                return false;
            }
        }

        true
    }

    fn populate(
        &self,
        configurations: &mut Vec<Configuration>,
        current_positions: &Vec<(usize, usize)>,
        target: usize,
        position: (usize, usize),
        delta: isize,
    ) {
        if target == current_positions.len() {
            return;
        }

        let (t, e) = position;

        if t >= self.positions.len() {
            return;
        }

        let mut new_e = e + 1;
        let mut new_t = t;
        if new_e == self.positions[t].len() {
            new_t += 1;
            new_e = 0;
        }

        // move only items on the current floor
        if self.elevator == self.positions[t][e]
            && self.elevator + delta >= 0
            && self.elevator + delta <= 3
        // TODO: 3 is hard-coded
        {
            let mut new_positions = current_positions.clone();
            new_positions.push(position);

            let mut new_configuration = self.clone();
            for p in &new_positions {
                new_configuration.positions[p.0][p.1] += delta;
            }
            new_configuration.elevator += delta;

            if new_configuration.is_valid() {
                configurations.push(new_configuration);
            }

            self.populate(
                configurations,
                &new_positions,
                target,
                (new_t, new_e),
                delta,
            )
        }

        self.populate(
            configurations,
            current_positions,
            target,
            (new_t, new_e),
            delta,
        )
    }

    fn next_configurations(&self) -> Vec<Configuration> {
        let mut configurations = vec![];

        for direction in [-1, 1] {
            self.populate(&mut configurations, &vec![], 2, (0, 0), direction);
        }

        configurations
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(start: Configuration) -> usize {
    let mut steps: HashMap<Configuration, usize> = HashMap::default();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    steps.insert(start.clone(), 0);
    heap.push(State {
        configuration: start.clone(),
        steps: 0,
    });

    while let Some(state) = heap.pop() {
        if state.configuration.is_end() {
            return state.steps;
        }

        for next_configuration in state.configuration.next_configurations() {
            let best_steps = *steps.get(&next_configuration).unwrap_or(&usize::MAX);

            if state.steps + 1 < best_steps {
                let next_state = State {
                    configuration: next_configuration.clone(),
                    steps: state.steps + 1,
                };

                steps.insert(next_configuration, next_state.steps);
                heap.push(next_state);
            }
        }
    }

    unreachable!()
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"(\b\w+\b)(?:-compatible)? (generator|microchip)").unwrap();

        let mut material_count = 0;
        let mut material_map: HashMap<String, usize> = HashMap::new();

        let mut generators: Vec<isize> = Vec::new();
        let mut microchips: Vec<isize> = Vec::new();

        for (i, line) in input.trim().lines().enumerate() {
            for cap in re.captures_iter(line) {
                let material_id = material_map.entry(cap[1].to_string()).or_insert_with(|| {
                    generators.push(0);
                    microchips.push(0);

                    material_count = microchips.len() - 1;
                    material_count
                });

                match &cap[2] {
                    "generator" => generators[*material_id] = i as isize,
                    "microchip" => microchips[*material_id] = i as isize,
                    _ => unreachable!(),
                }
            }
        }

        let state = Configuration {
            elevator: 0,
            positions: vec![generators, microchips],
        };

        let steps = solve(state);

        Option::from(steps.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"(\b\w+\b)(?:-compatible)? (generator|microchip)").unwrap();

        let mut material_count = 0;
        let mut material_map: HashMap<String, usize> = HashMap::new();

        let mut generators: Vec<isize> = Vec::new();
        let mut microchips: Vec<isize> = Vec::new();

        for (i, line) in input.trim().lines().enumerate() {
            let mut line = line.to_string();
            if i == 0 {
                line.push_str("An elerium generator. An elerium-compatible microchip. A dilithium generator. A dilithium-compatible microchip.")
            }

            for cap in re.captures_iter(&*line) {
                let material_id = material_map.entry(cap[1].to_string()).or_insert_with(|| {
                    generators.push(0);
                    microchips.push(0);

                    material_count = microchips.len() - 1;
                    material_count
                });

                match &cap[2] {
                    "generator" => generators[*material_id] = i as isize,
                    "microchip" => microchips[*material_id] = i as isize,
                    _ => unreachable!(),
                }
            }
        }

        let state = Configuration {
            elevator: 0,
            positions: vec![generators, microchips],
        };

        let steps = solve(state);

        Option::from(steps.to_string())
    }
}
