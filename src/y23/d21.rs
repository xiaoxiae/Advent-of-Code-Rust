//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/21
//!
//! ⚠️ part 2 UNSOLVED: the original 21-2.py was incomplete (raised NameError);
//! solve_part2 is left defaulted (None) pending a manual implementation (the
//! quadratic-extrapolation infinite-garden solution). Part 1 (3724) is verified.
use crate::util::Day;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct D21;

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let maze: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        let mut start: Option<(i64, i64)> = None;
        for (y, row) in maze.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val == 'S' {
                    start = Some((x as i64, y as i64));
                }
            }
        }
        let start = start.unwrap();

        let height = maze.len() as i64;
        let width = maze[0].len() as i64;

        let mut explored: HashMap<(i64, i64), i64> = HashMap::new();
        explored.insert(start, 0);
        let mut queue: VecDeque<(i64, (i64, i64))> = VecDeque::new();
        queue.push_back((0, start));

        let total_steps: i64 = 64;

        while let Some((steps, (x, y))) = queue.pop_front() {
            if steps > total_steps {
                continue;
            }

            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;
                let ns = steps + 1;

                if !(0 <= nx && nx < width && 0 <= ny && ny < height) {
                    continue;
                }

                if maze[ny as usize][nx as usize] == '#' {
                    continue;
                }

                if !explored.contains_key(&(nx, ny)) {
                    explored.insert((nx, ny), ns);
                    queue.push_back((ns, (nx, ny)));
                }
            }
        }

        let total = explored
            .values()
            .filter(|&&v| (v % 2) == (total_steps % 2))
            .count();

        Some(total.to_string())
    }

    fn solve_part2(&self, _input: &str) -> Option<String> {
        // The original Python solution for part 2 is incomplete: it references an
        // undefined variable `total_steps` and crashes with a NameError before
        // producing any answer. Faithfully preserving that behavior, there is no
        // result to return.
        None
    }
}
