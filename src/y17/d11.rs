use std::collections::HashMap;
use crate::util::Day;

pub struct D11;

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // we convert the grid into the following equal representation
        //
        // . . . . .
        //  . . . .
        // . . o . .
        //  . . . .
        // . . . . .
        //
        // to calculate closest path distance, we can think of as
        // 1. getting to the right column (= x.abs())
        // 2. getting to the right row,
        //    possibly gaining some elevation when getting to the right column

        let deltas = HashMap::from([
            ("ne", (1, 1)),
            ("n", (0, 2)),
            ("nw", (-1, 1)),
            ("sw", (-1, -1)),
            ("s", (0, -2)),
            ("se", (1, -1)),
        ]);

        let mut x: isize = 0;
        let mut y: isize = 0;

        for direction in input.trim().split(",") {
            let (dx, dy) = deltas.get(&direction)?;

            x += dx;
            y += dy;
        }

        let distance = (x.abs() + (y.abs() - x.abs()).max(0) / 2) as usize;

        Option::from(distance.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let deltas = HashMap::from([
            ("ne", (1, 1)),
            ("n", (0, 2)),
            ("nw", (-1, 1)),
            ("sw", (-1, -1)),
            ("s", (0, -2)),
            ("se", (1, -1)),
        ]);

        let mut x: isize = 0;
        let mut y: isize = 0;

        let mut max_distance: usize = 0;

        for direction in input.trim().split(",") {
            let (dx, dy) = deltas.get(&direction)?;

            x += dx;
            y += dy;

            let distance = (x.abs() + (y.abs() - x.abs()).max(0) / 2) as usize;

            max_distance = max_distance.max(distance);
        }

        Option::from(max_distance.to_string())
    }
}
