//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/03
use crate::util::Day;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim().lines().collect();

        let mut x: usize = 0;
        let mut trees: u64 = 0;

        for y in 1..lines.len() {
            let line = lines[y].as_bytes();
            x = (x + 3) % line.len();

            if line[x] == b'#' {
                trees += 1;
            }
        }

        Some(trees.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.trim().lines().collect();

        let slopes = [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut product: u64 = 1;

        for (xd, yd) in slopes {
            let mut x: usize = 0;
            let mut trees: u64 = 0;

            let mut y = yd;
            while y < lines.len() {
                let line = lines[y].as_bytes();
                x = (x + xd) % line.len();

                if line[x] == b'#' {
                    trees += 1;
                }
                y += yd;
            }
            product *= trees;
        }

        Some(product.to_string())
    }
}
