use crate::util::Day;
use std::collections::HashSet;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let steps = input.trim().split(", ").collect::<Vec<&str>>();
        let mut direction = 0;
        let mut x = 0;
        let mut y = 0;

        for step in steps {
            let (turn, value) = step.split_at(1);
            let value: i32 = value.parse().ok()?;
            direction = (direction + if turn == "R" { 1 } else { -1 } + 4) % 4;

            match direction {
                0 => y += value,
                1 => x -= value,
                2 => y -= value,
                3 => x += value,
                _ => unreachable!(),
            }
        }

        Some((x.abs() + y.abs()).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let steps = input.trim().split(", ").collect::<Vec<&str>>();
        let mut direction = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut visited = HashSet::new();

        for step in steps {
            let (turn, value) = step.split_at(1);
            let mut value: i32 = value.parse().ok()?;
            direction = (direction + if turn == "R" { 1 } else { -1 } + 4) % 4;

            while value > 0 {
                match direction {
                    0 => y += 1,
                    1 => x -= 1,
                    2 => y -= 1,
                    3 => x += 1,
                    _ => unreachable!(),
                }

                if !visited.insert((x, y)) {
                    return Some((x.abs() + y.abs()).to_string());
                }

                value -= 1;
            }
        }

        None
    }
}
