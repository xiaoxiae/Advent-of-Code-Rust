use crate::util::Day;
use regex::Regex;
use std::collections::VecDeque;

type Position = (usize, usize);
type Distance = usize;

type Bytes = Vec<Position>;
type Map = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Bytes {
    let re = Regex::new(r"\d+").unwrap();

    let numbers = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    numbers
        .chunks(2)
        .map(|parts| (parts[0], parts[1]))
        .collect()
}

#[derive(Debug)]
struct State {
    position: Position,
    distance: Distance,
}

impl State {
    fn next_states(&self, map: &Map) -> Vec<State> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().map(|(dx, dy)| {
            let (x, y) = self.position;

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !(0 <= ny && ny < map.len() as isize) || !(0 <= nx && nx < map[ny as usize].len() as isize) {
                None
            } else {
                let p: Position = (nx as usize, ny as usize);

                if map[p.1][p.0] {
                    None
                } else {
                    Option::from(State {
                        position: p,
                        distance: self.distance + 1,
                    })
                }
            }
        }
        ).filter_map(|v| v).collect::<Vec<_>>()
    }
}

fn solve(
    map: &Map,
    start: Position,
    end: Position,
) -> Option<Distance> {
    let mut explored: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();

    explored[start.1][start.0] = true;
    queue.push_back(State { position: start, distance: 0 });

    while let Some(state) = queue.pop_front() {
        if state.position == end {
            return Option::from(state.distance);
        }

        for next_state in state.next_states(&map) {
            let &(x, y) = &next_state.position;
            // New/improved
            if !explored[y][x] {
                explored[y][x] = true;
                queue.push_back(next_state);
            }
        }
    }

    None
}

pub struct Y24D18;

impl Day for Y24D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bytes = parse_input(input);

        // (128 is arbitrarily set for testing instances)
        let (width, height, fallen_bytes) = if bytes.len() <= 128 { (7, 7, 12) } else { (71, 71, 1024) };

        let mut map: Vec<Vec<bool>> = vec![vec![false; width]; height];

        for fallen_byte in 0..fallen_bytes {
            let (x, y) = bytes[fallen_byte];
            map[y][x] = true;
        }

        let distance = solve(&map, (0, 0), (width - 1, height - 1));

        Option::from(distance.expect("No path found!").to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bytes = parse_input(input);

        // (128 is arbitrarily set for testing instances)
        let (width, height) = if bytes.len() <= 128 { (7, 7) } else { (71, 71) };

        let mut lo = 0;
        let mut hi = bytes.len();

        while lo < hi {
            let mut map: Vec<Vec<bool>> = vec![vec![false; width]; height];
            let mid = (lo + hi) / 2;

            for fallen_byte in 0..=mid {
                let (x, y) = bytes[fallen_byte];
                map[y][x] = true;
            }

            match solve(&map, (0, 0), (width - 1, height - 1)) {
                Some(_) => lo = mid + 1,
                None => hi = mid,
            }
        }

        if lo == bytes.len() {
            panic!("Path is never blocked!")
        }

        let (x, y) = bytes[lo];
        return Option::from(format!("{},{}", x, y).to_string());
    }
}
