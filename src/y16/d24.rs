use crate::util::Day;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub struct D24;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        vec![
            Position { x: self.x + 1, y: self.y },
            Position { x: self.x - 1, y: self.y },
            Position { x: self.x, y: self.y + 1 },
            Position { x: self.x, y: self.y - 1 },
        ]
    }
}

fn parse_map(input: &str) -> (Vec<Vec<bool>>, HashMap<usize, Position>) {
    let mut walls = Vec::new();
    let mut numbers = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false);

                if ch.is_ascii_digit() {
                    numbers.insert(
                        ch.to_digit(10).unwrap() as usize,
                        Position { x: x as isize, y: y as isize },
                    );
                }
            }
        }
        walls.push(row);
    }

    (walls, numbers)
}

fn get_distances(walls: &Vec<Vec<bool>>, numbers: &HashMap<usize, Position>) -> HashMap<usize, HashMap<usize, usize>> {
    // TODO: optimize this, much room to improve lmao

    let mut number_distances = HashMap::new();

    for key in numbers.keys() {
        let start = numbers[key];

        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back((start, 0));
        distances.insert(start, 0);

        while let Some((pos, dist)) = queue.pop_front() {
            for neighbor in pos.neighbours() {
                // skip walls and out of bounds
                if walls.get(neighbor.y as usize).and_then(|row| row.get(neighbor.x as usize)) == Some(&false) {
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, dist + 1);
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }
        }

        number_distances.insert(*key, HashMap::new());

        for (other_key, &position) in numbers {
            if key == other_key {
                continue;
            }

            number_distances.get_mut(key).unwrap().insert(*other_key, *distances.get(&position).unwrap());
        }
    }

    number_distances
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (walls, numbers) = parse_map(input);

        let distances = get_distances(&walls, &numbers);

        let mut min_dist = usize::MAX;
        for d in (1..=distances.keys().len() - 1).permutations(distances.keys().len() - 1) {
            // always start at 0!
            let mut curr_dist = *distances.get(&0).unwrap().get(&d[0]).unwrap();

            for i in 0..d.len() - 1 {
                curr_dist += distances.get(&d[i]).unwrap().get(&d[i + 1]).unwrap();
            }

            min_dist = min_dist.min(curr_dist);
        }

        Option::from(min_dist.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (walls, numbers) = parse_map(input);

        let distances = get_distances(&walls, &numbers);

        let mut min_dist = usize::MAX;
        for d in (1..=distances.keys().len() - 1).permutations(distances.keys().len() - 1) {
            // always start at 0!
            let mut curr_dist = *distances.get(&0).unwrap().get(&d[0]).unwrap();

            for i in 0..d.len() - 1 {
                curr_dist += distances.get(&d[i]).unwrap().get(&d[i + 1]).unwrap();
            }

            // and end at 0!
            curr_dist += distances.get(&d[d.len() - 1]).unwrap().get(&0).unwrap();

            min_dist = min_dist.min(curr_dist);
        }

        Option::from(min_dist.to_string())
    }
}
