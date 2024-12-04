use crate::util::Day;

pub struct Day4;

fn check_pattern(lines: &Vec<Vec<char>>, pattern: &str, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    for (i, c) in pattern.chars().enumerate() {
        let nx = x + dx * i as i32;
        let ny = y + dy * i as i32;

        if nx < 0 || ny < 0 || nx as usize >= lines[0].len() || ny as usize >= lines.len() {
            return false;
        }

        if lines[ny as usize][nx as usize] != c {
            return false;
        }
    }

    true
}

impl Day for Day4 {
    fn solve_part1(&self, input: &str) -> String {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut xmas_count = 0;

        for y in 0..lines.len() as i32 {
            for x in 0..lines.len() as i32 {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if check_pattern(&lines, "XMAS", x, y, dx, dy) {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }

        xmas_count.to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let lines = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut x_das_mas = 0;

        for y in 0..lines.len() as i32 {
            for x in 0..lines.len() as i32 {
                if lines[y as usize][x as usize] != 'A' {
                    continue;
                }

                if !["SAM", "MAS"]
                    .iter()
                    .any(|&pattern| check_pattern(&lines, &pattern, x - 1, y - 1, 1, 1))
                {
                    continue;
                }

                if !["SAM", "MAS"]
                    .iter()
                    .any(|&pattern| check_pattern(&lines, &pattern, x - 1, y + 1, 1, -1))
                {
                    continue;
                }

                x_das_mas += 1;
            }
        }

        x_das_mas.to_string()
    }
}
