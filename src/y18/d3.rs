use crate::util::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut board = vec![vec![0; 1000]; 1000];
        let mut total = 0;
        let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        for line in input.lines() {
            let caps = re.captures(line).unwrap();
            let start_x: usize = caps[1].parse().unwrap();
            let start_y: usize = caps[2].parse().unwrap();
            let width: usize = caps[3].parse().unwrap();
            let height: usize = caps[4].parse().unwrap();

            for x in start_x..start_x + width {
                for y in start_y..start_y + height {
                    board[x][y] += 1;
                    if board[x][y] == 2 {
                        total += 1;
                    }
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut board = vec![vec![-1; 1000]; 1000];
        let mut elfs = vec![true; input.lines().count()];
        let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        for (i, line) in input.lines().enumerate() {
            let caps = re.captures(line).unwrap();
            let start_x: usize = caps[1].parse().unwrap();
            let start_y: usize = caps[2].parse().unwrap();
            let width: usize = caps[3].parse().unwrap();
            let height: usize = caps[4].parse().unwrap();

            for x in start_x..start_x + width {
                for y in start_y..start_y + height {
                    if board[x][y] != -1 {
                        elfs[i] = false;
                        elfs[board[x][y] as usize] = false;
                    } else {
                        board[x][y] = i as i32;
                    }
                }
            }
        }

        Some((elfs.iter().position(|&x| x).unwrap() + 1).to_string())
    }
}
