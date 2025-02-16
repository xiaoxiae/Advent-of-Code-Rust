use crate::util::Day;

pub struct D5;

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut instructions: Vec<isize> = input.lines().filter_map(|line| line.parse().ok()).collect();
        let mut index: isize = 0;
        let mut steps: usize = 0;

        while let Some(jump) = instructions.get_mut(index as usize) {
            *jump += 1;
            index += *jump - 1;
            steps += 1;
        }

        Some(steps.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut instructions: Vec<isize> = input.lines().filter_map(|line| line.parse().ok()).collect();
        let mut index: isize = 0;
        let mut steps: usize = 0;

        while let Some(jump) = instructions.get_mut(index as usize) {
            if *jump >= 3 {
                *jump -= 1;
                index += *jump + 1;
            } else {
                *jump += 1;
                index += *jump - 1;
            }
            steps += 1;
        }

        Some(steps.to_string())
    }
}
