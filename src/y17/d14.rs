use crate::util::Day;
use crate::y17::D10;
use std::collections::VecDeque;

pub struct D14;

static SIZE: usize = 128;

fn bfs(grid: &Vec<Vec<bool>>, explored: &mut Vec<Vec<bool>>, start: (usize, usize)) {
    let mut queue = VecDeque::from([start]);
    explored[start.1][start.0] = true;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !(0 <= nx && nx < grid[0].len() as isize && 0 <= ny && ny < grid.len() as isize) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] && !explored[ny][nx] {
                explored[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }
}

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let hash = input.trim();

        let mut squares = 0;
        for i in 0..SIZE {
            let knot_hash = D10::solve_part2(&D10, format!("{}-{}", hash, i).as_str()).unwrap();

            for char in knot_hash.chars() {
                squares += char.to_digit(16).unwrap().count_ones();
            }
        }

        Option::from(squares.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let hash = input.trim();

        let mut grid = Vec::new();
        for i in 0..SIZE {
            let mut row = Vec::new();
            let knot_hash = D10::solve_part2(&D10, format!("{}-{}", hash, i).as_str()).unwrap();

            for char in knot_hash.chars() {
                let bits = char.to_digit(16).unwrap();

                for i in 0..4 {
                    row.push((bits & (1 << (3 - i))) != 0);
                }
            }

            grid.push(row);
        }

        let mut explored = vec![vec![false; SIZE]; SIZE];
        let mut components = 0;

        for y in 0..SIZE {
            for x in 0..SIZE {
                if grid[y][x] && !explored[y][x] {
                    bfs(&grid, &mut explored, (x, y));
                    components += 1;
                }
            }
        }

        Option::from(components.to_string())
    }
}
