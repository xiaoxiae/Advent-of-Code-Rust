use crate::util::Day;
use itertools;
use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use rustc_hash::{FxHashMap as HashMap};

pub struct D4;

fn check_pattern(lines: &Vec<Vec<char>>, pattern: &str, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    for (i, c) in pattern.chars().enumerate() {
        let nx = x + dx * i as i32;
        let ny = y + dy * i as i32;

        let char = lines.get(ny as usize).and_then(|row| row.get(nx as usize));

        match char {
            Some(&_c) if c == _c => continue,
            Some(_) => return false,
            None => return false,
        }
    }

    true
}

fn next_states(
    lines: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    character: char,
) -> impl Iterator<Item = ((usize, usize), char, usize)> + '_ {
    iproduct!(
        [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1)
        ],
        1..=5,
    )
    .filter_map(move |((dx, dy), step)| {
        let i = "XMAS".find(character).unwrap();
        let expected_char = "XMAS".chars().nth((i + 1) % 4).unwrap();

        let nx = (x as i32 + dx * step) as usize;
        let ny = (y as i32 + dy * step) as usize;

        match lines.get(ny).and_then(|row| row.get(nx)) {
            Some(char) if char == &expected_char => Some(((nx, ny), *char, 1usize)),
            _ => None,
        }
    })
}
#[derive(Debug, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    character: char,
    distance: usize,
}

impl State {
    fn new(position: (usize, usize), character: char, distance: usize) -> Self {
        State {
            position,
            character,
            distance,
        }
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

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut xmas_count = 0;

        for y in 0..lines.len() as i32 {
            for x in 0..lines.len() as i32 {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if check_pattern(&lines, "XMAS", x, y, dx, dy) {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }

        Option::from(xmas_count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut x_das_mas = 0;

        for y in 0..lines.len() as i32 {
            for x in 0..lines.len() as i32 {
                if lines[y as usize][x as usize] != 'A' {
                    continue;
                }

                if !["SAM", "MAS"]
                    .iter()
                    .any(|&pattern| check_pattern(&lines, &pattern, x - 1, y - 1, 1, 1))
                {
                    continue;
                }

                if !["SAM", "MAS"]
                    .iter()
                    .any(|&pattern| check_pattern(&lines, &pattern, x - 1, y + 1, 1, -1))
                {
                    continue;
                }

                x_das_mas += 1;
            }
        }

        Option::from(x_das_mas.to_string())
    }

    /// --- Tom's Part 3 ---
    /// Find the shortest path from top left to bottom right in the repeating XMAS pattern.
    /// You can move left/top/right/bottom by at most 3 steps.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let start = (0, 0);
        let end = (lines[0].len() - 1, lines.len() - 1);

        let mut distances: HashMap<((usize, usize), char), (((usize, usize), char), usize)> =
            HashMap::default();
        let mut heap = BinaryHeap::new();

        distances.insert(
            (start, lines[start.1][start.0]),
            ((start, lines[start.1][start.0]), 0),
        );
        heap.push(State::new(start, lines[start.1][start.0], 0));

        while let Some(State {
            position,
            character,
            distance,
        }) = heap.pop()
        {
            if position == end {
                let mut c_position = position;
                let mut c_character = character;

                let mut total = 0;
                while let Some(&((p_position, p_character), distance)) =
                    distances.get(&(c_position, c_character))
                {
                    total += c_position.0 * c_position.1;

                    c_position = p_position;
                    c_character = p_character;

                    if distance == 0 {
                        break;
                    }
                }

                return Some(total.to_string());
            }

            // Skip processing if we have already found a shorter distance
            if distance
                > *distances
                    .get(&(position, character))
                    .and_then(|(_, d)| Option::from(d))
                    .unwrap_or(&usize::MAX)
            {
                continue;
            }

            // Get the next possible positions and distances
            for (next_position, next_character, next_distance) in
                next_states(&lines, position, character)
            {
                let new_distance = distance + next_distance;

                // If we found a shorter path to the next position, update it
                if new_distance
                    < *distances
                        .get(&(next_position, next_character))
                        .and_then(|(_, d)| Option::from(d))
                        .unwrap_or(&usize::MAX)
                {
                    distances.insert((next_position, next_character), ((position, character), new_distance));
                    heap.push(State::new(next_position, next_character, new_distance));
                }
            }
        }

        Some("No path found!".to_string())
    }
}
