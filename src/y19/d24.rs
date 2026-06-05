//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/24
//!
//! ⚠️ part 2 UNSOLVED: the original 24-2.py crashed (never solved); `solve_part2`
//! returns None pending a manual recursive-Eris implementation. Part 1 (19923473)
//! is verified.
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D24;

type Grid = Vec<Vec<char>>;

fn to_string(bugs: &[Vec<char>]) -> String {
    bugs.iter().flatten().collect()
}

fn score(bugs: &[Vec<char>]) -> u64 {
    let mut total = 0u64;
    for (i, char) in to_string(bugs).chars().enumerate() {
        total += (1u64 << i) * if char == '.' { 0 } else { 1 };
    }
    total
}

fn evolve(bugs: &[Vec<char>]) -> Grid {
    let height = bugs.len();
    let width = bugs[0].len();
    let mut new_bugs = vec![vec!['.'; width]; height];

    for y in 0..height {
        for x in 0..width {
            let mut adjacent = 0;

            for (yd, xd) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let yn = y as i32 + yd;
                let xn = x as i32 + xd;

                if 0 <= yn
                    && yn < height as i32
                    && 0 <= xn
                    && xn < width as i32
                    && bugs[yn as usize][xn as usize] == '#'
                {
                    adjacent += 1;
                }
            }

            new_bugs[y][x] = if (bugs[y][x] == '#' && adjacent == 1)
                || (bugs[y][x] == '.' && (adjacent == 1 || adjacent == 2))
            {
                '#'
            } else {
                '.'
            };
        }
    }

    new_bugs
}

fn run(input: &str) -> u64 {
    let mut bugs: Grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut explored = FxHashSet::default();
    explored.insert(to_string(&bugs));

    loop {
        bugs = evolve(&bugs);
        let string = to_string(&bugs);

        if !explored.insert(string) {
            return score(&bugs);
        }
    }
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(run(input).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // The original 24-2.py crashed (never solved); a faithful port is not
        // available. Left unsolved pending a manual recursive-Eris implementation.
        let _ = input;
        None
    }
}
