use crate::util::Day;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let up = input.chars().filter(|&c| c == '(').count();
        let down = input.chars().filter(|&c| c == ')').count();
        Some((up as isize - down as isize).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut position = 0;
        for (i, step) in input.chars().enumerate() {
            position += if step == '(' { 1 } else { -1 };
            if position == -1 {
                return Some((i + 1).to_string());
            }
        }
        None
    }
}
