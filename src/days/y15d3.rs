use crate::util::Day;
use std::collections::HashSet;

pub struct Y15D3;

impl Day for Y15D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut visited = HashSet::new();
        let (mut x, mut y) = (0, 0);

        visited.insert((x, y));

        for step in input.chars() {
            match step {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => {}
            }
            visited.insert((x, y));
        }

        Some(visited.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut visited = HashSet::new();
        let mut positions = [(0, 0), (0, 0)];

        visited.insert((0, 0));

        for (i, step) in input.chars().enumerate() {
            let current = i % 2;
            match step {
                '>' => positions[current].0 += 1,
                '<' => positions[current].0 -= 1,
                '^' => positions[current].1 += 1,
                'v' => positions[current].1 -= 1,
                _ => {}
            }
            visited.insert(positions[current]);
        }

        Some(visited.len().to_string())
    }
}
