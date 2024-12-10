use crate::util::Day;
use std::collections::{HashSet, VecDeque};

pub struct Day10;

fn walk_trail(map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let mut queue = VecDeque::from([(x, y, 0)]);
    let mut count = 0;

    let mut ends = HashSet::new();

    while !queue.is_empty() {
        let (x, y, digit) = queue.pop_front().unwrap();

        if digit == 9 {
            ends.insert((x, y));
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                Some(&d) if d == digit + 1 => {
                    queue.push_back((nx as usize, ny as usize, digit + 1));
                }
                _ => continue,
            }
        }
    }

    ends.len()
}

fn walk_trail_distinct(map: &Vec<Vec<usize>>, x: usize, y: usize, digit: usize) -> usize {
    if digit == 9 {
        return 1;
    }

    let mut count = 0;
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some(&d) if d == digit + 1 => {
                count += walk_trail_distinct(&map, nx as usize, ny as usize, digit + 1);
            }
            _ => continue,
        }
    }

    count
}

fn parse_map(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split_terminator('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

impl Day for Day10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let map = parse_map(input);

        let mut total = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    total += walk_trail(&map, x, y);
                }
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let map = parse_map(input);

        let mut total = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    total += walk_trail_distinct(&map, x, y, 0);
                }
            }
        }

        Option::from(total.to_string())
    }
}
