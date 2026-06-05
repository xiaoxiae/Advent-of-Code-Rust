//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/18
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D18;

const STEPS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

/// Parse the input into a square grid of small ints: 0 = open, 1 = tree, 2 = lumberyard.
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => 1u8,
                    '#' => 2u8,
                    _ => 0u8,
                })
                .collect()
        })
        .collect()
}

/// Returns the frequency of [open, tree, lumberyard] appearing around a coordinate.
fn neighbourhood(x: usize, y: usize, world: &[Vec<u8>]) -> [usize; 3] {
    let rows = world.len() as isize;
    let cols = world[0].len() as isize;
    let mut neighbours = [0usize; 3];

    for (dx, dy) in STEPS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 && nx < rows && ny < cols {
            neighbours[world[nx as usize][ny as usize] as usize] += 1;
        }
    }

    neighbours
}

/// Advance the world one generation using the cellular-automaton rules.
fn step(world: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let rows = world.len();
    let cols = world[0].len();
    let mut new_world = vec![vec![0u8; cols]; rows];

    for x in 0..rows {
        for y in 0..cols {
            let neighbours = neighbourhood(x, y, world);
            new_world[x][y] = if world[x][y] == 0 && neighbours[1] >= 3 {
                1
            } else if world[x][y] == 1 && neighbours[2] >= 3 {
                2
            } else if world[x][y] == 2 && (neighbours[2] == 0 || neighbours[1] == 0) {
                0
            } else {
                world[x][y]
            };
        }
    }

    new_world
}

/// String key for a world state, matching the Python `mapToString`.
fn map_to_string(world: &[Vec<u8>]) -> String {
    let mut s = String::with_capacity(world.len() * world[0].len());
    for line in world {
        for &cell in line {
            s.push((b'0' + cell) as char);
        }
    }
    s
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut world = parse(input);

        for _ in 0..10 {
            world = step(&world);
        }

        let trees = world.iter().flatten().filter(|&&cell| cell == 1).count() as u64;
        let lumberjacks = world.iter().flatten().filter(|&&cell| cell == 2).count() as u64;

        Some((trees * lumberjacks).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let number_of_generations: u64 = 1_000_000_000;

        let mut world = parse(input);

        // Maps a world-state string to the generation at which it first appeared.
        let mut gen_dict: FxHashMap<String, u64> = FxHashMap::default();
        let mut generation: u64 = 0;

        loop {
            let key = map_to_string(&world);
            if gen_dict.contains_key(&key) {
                break;
            }
            let new_world = step(&world);
            gen_dict.insert(key, generation);
            generation += 1;
            world = new_world;
        }

        // The start and end of the cycle.
        let cycle_start = gen_dict[&map_to_string(&world)];
        let cycle_end = generation;

        // How many generations are left.
        let cycle_index = (number_of_generations - cycle_start) % (cycle_end - cycle_start);

        // Find and sum the generation whose index in the cycle matches the last generation.
        let mut trees = 0u64;
        let mut lumberjacks = 0u64;
        for (k, v) in &gen_dict {
            if *v >= cycle_start && v - cycle_start == cycle_index {
                for ch in k.chars() {
                    if ch == '1' {
                        trees += 1;
                    } else if ch == '2' {
                        lumberjacks += 1;
                    }
                }
            }
        }

        Some((trees * lumberjacks).to_string())
    }
}
