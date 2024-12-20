use crate::util::Day;
use rayon::prelude::*;
use std::collections::HashMap;

static MIN_SAVE_TIME: usize = 100;

type Map = Vec<Vec<char>>;
type DistanceMap = Vec<Vec<isize>>;

type Position = (usize, usize);

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

/// Trace the path from end to start, returning the distances
fn get_distances(input: &str) -> DistanceMap {
    let (map, start, mut end) = parse_input(input);

    let mut distances: DistanceMap = vec![vec![-1; map[0].len()]; map.len()];

    distances[end.1][end.0] = 0;
    while end != start {
        let (x, y) = end;

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            match map.get(ny).and_then(|r| r.get(nx)) {
                Some('.') if distances[ny][nx] == -1 => {
                    distances[ny][nx] = distances[y][x] + 1;
                    end = (nx, ny);
                }
                _ => continue,
            }
        }
    }

    distances
}

/// Get a dictionary of cheat saving times
fn get_cheats(distances: &DistanceMap, cheat_time: isize) -> HashMap<usize, usize> {
    let mut cheats: HashMap<usize, usize> = HashMap::new();

    let cheat_updates: Vec<HashMap<usize, usize>> = (0..distances.len())
        .into_par_iter()
        .map(|y| {
            let mut local_cheats: HashMap<usize, usize> = HashMap::new();

            for x in 0..distances[y].len() {
                if distances[y][x] == -1 {
                    continue;
                }

                for dx in -cheat_time..=cheat_time {
                    for dy in -cheat_time..=cheat_time {
                        if (dx.abs() + dy.abs()) > cheat_time {
                            continue;
                        }

                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;

                        match distances.get(ny).and_then(|r| r.get(nx)) {
                            Some(-1) | None => continue,
                            _ => {
                                let delta =
                                    distances[y][x] - distances[ny][nx] - (dx.abs() + dy.abs());

                                if delta <= 0 {
                                    continue;
                                }

                                *local_cheats.entry(delta as usize).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }

            local_cheats
        })
        .collect();

    for update in cheat_updates {
        for (key, value) in update {
            *cheats.entry(key).or_insert(0) += value;
        }
    }

    cheats
}

pub struct Y24D20;

impl Day for Y24D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let distances = get_distances(input);
        let cheats = get_cheats(&distances, 2);

        let mut total = 0;
        for (save, count) in cheats {
            if save >= MIN_SAVE_TIME {
                total += count;
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let distances = get_distances(input);
        let cheats = get_cheats(&distances, 20);

        let mut total = 0;
        for (save, count) in cheats {
            if save >= MIN_SAVE_TIME {
                total += count;
            }
        }

        Option::from(total.to_string())
    }
}
