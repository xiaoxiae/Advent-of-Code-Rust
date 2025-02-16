use std::collections::HashMap;
use crate::util::Day;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let number: isize = input.trim().parse().ok()?;
        let square = (((number as f64).sqrt() - 1.0) / 2.0).ceil() as isize;

        let mut min_distance = isize::MAX;
        for i in 0..4 {
            let distance = ((number - ((square * 2 + 1).pow(2) - i * (square * 2) - square)).abs()) + square;
            if distance < min_distance {
                min_distance = distance;
            }
        }

        Some(min_distance.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let target: isize = input.trim().parse().ok()?;
        let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
        let deltas = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];

        let mut x = 0;
        let mut y = 0;
        let mut steps = 1;
        let mut value = 1;
        grid.insert((x, y), value);

        loop {
            // walking right-up-left-down
            // each 2 turns, the number of steps we have to take increases by 1 (draw it out)
            for (i, &dir) in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter().enumerate() {
                for _ in 0..steps {
                    x += dir.0;
                    y += dir.1;
                    value = deltas.iter().map(|&(dx, dy)| grid.get(&(x + dx, y + dy)).unwrap_or(&0)).sum();
                    grid.insert((x, y), value);

                    if value > target {
                        return Some(value.to_string());
                    }
                }

                if i % 2 == 1 {
                    steps += 1;
                }
            }
        }
    }
}
