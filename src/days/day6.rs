use std::collections::{HashMap, HashSet};
use crate::util::Day;

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
) -> Option<(usize, usize)> {
    let (dx, dy) = DIRECTIONS[direction];
    let (mut x, mut y) = position;

    loop {
        map[y][x] = 'X';

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
            let result = move_forward(&mut map, start, direction);

            match result {
                Some(p) => start = p,
                None => break,
            }

            direction = (direction + 1) % DIRECTIONS.len();
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

        // Use part 1 to get possible barrel positions
        let mut direction = initial_direction;
        loop {
            let result = move_forward(&mut map, start, direction);

            match result {
                Some(p) => start = p,
                None => break,
            }

            direction = (direction + 1) % DIRECTIONS.len();
        }

        // Go through all barrel positions and simulate (tehehe)
        let mut valid_obstructions = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != 'X' {
                    continue;
                }

                map[y][x] = '#';

                start = initial_start.clone();
                direction = 0;
                let mut reached_states: HashSet<((usize, usize), usize)> = HashSet::new();

                loop {
                    let result = move_forward(&mut map, start, direction);

                    match result {
                        Some(p) => start = p,
                        None => break,
                    }

                    direction = (direction + 1) % DIRECTIONS.len();

                    if reached_states.contains(&(start, direction)) {
                        valid_obstructions += 1;
                        break;
                    }

                    reached_states.insert((start, direction));
                }

                map[y][x] = '.';
            }
        }

        valid_obstructions.to_string()
    }
}
