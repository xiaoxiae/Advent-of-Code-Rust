use crate::util::Day;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Day6;

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map = input
        .trim()
        .split_whitespace()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                start = (x, y);
            }
        }
    }

    (map, start)
}

/// Move forward on the map, marking the directions
fn move_forward(
    map: &mut Vec<Vec<char>>,
    position: (usize, usize),
    direction: usize,
    write: bool,
) -> Option<(usize, usize)> {
    let (dx, dy) = DIRECTIONS[direction];
    let (mut x, mut y) = position;

    loop {
        if write {
            map[y][x] = 'X';
        }

        let (nx, ny) = (x as i32 + dx, y as i32 + dy);

        match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            None => return None,
            Some('#') => return Some((x, y)),
            Some(_) => (x, y) = (nx as usize, ny as usize),
        }
    }
}

impl Day for Day6 {
    fn solve_part1(&self, input: &str) -> String {
        let (mut map, mut start) = parse_map(input);

        let mut direction = 0;

        loop {
            let result = move_forward(&mut map, start, direction, true);

            match result {
                Some(p) => {
                    start = p;
                    direction = (direction + 1) % DIRECTIONS.len();
                }
                None => break,
            }
        }

        map.iter()
            .flatten()
            .filter(|&&c| c == 'X')
            .count()
            .to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let (mut map, mut start) = parse_map(input);

        let initial_start = start.clone();
        let initial_direction = 0;

        // Use part 1 to get possible barrel positions, as well as shortcuts
        let mut direction = initial_direction;
        let mut shortcuts: HashMap<((usize, usize), usize), ((usize, usize), usize)> =
            HashMap::new();
        loop {
            let result = move_forward(&mut map, start, direction, true);

            match result {
                Some(p) => {
                    shortcuts.insert((start, direction), (p, (direction + 1) % DIRECTIONS.len()));

                    start = p;
                    direction = (direction + 1) % DIRECTIONS.len();
                }
                None => break,
            }
        }

        let valid_obstructions = AtomicUsize::new(0);

        // Go through all barrel positions and simulate (tehehe)
        let positions: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &cell)| cell == 'X')
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        // Process each (x, y) in parallel
        positions.par_iter().for_each(|&(x, y)| {
            let mut local_map = map.clone();
            local_map[y][x] = '#';

            let mut start = initial_start.clone();
            let mut direction = 0;
            let mut reached_states: HashSet<((usize, usize), usize)> = HashSet::new();

            loop {
                // Use shortcuts if possible
                if x != start.0 && y != start.1 {
                    if let Some(&(new_start, new_direction)) = shortcuts.get(&(start, direction)) {
                        start = new_start;
                        direction = new_direction;
                        continue;
                    }
                }

                if let Some(p) = move_forward(&mut local_map, start, direction, false) {
                    start = p;
                    direction = (direction + 1) % DIRECTIONS.len();
                } else {
                    break;
                }

                if reached_states.contains(&(start, direction)) {
                    valid_obstructions.fetch_add(1, Ordering::Relaxed);
                    break;
                }

                reached_states.insert((start, direction));
            }

            local_map[y][x] = '.';
        });

        valid_obstructions.load(Ordering::Relaxed).to_string()
    }
}
