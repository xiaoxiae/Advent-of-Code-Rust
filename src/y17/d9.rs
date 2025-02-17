use crate::util::Day;

pub struct D9;

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let input = input.trim().chars().collect::<Vec<_>>();

        let mut i = 0;

        let mut nesting = 0;
        let mut score = 0;
        let mut in_garbage = false;

        while i < input.len() {
            match input[i..] {
                ['{', ..] if !in_garbage => {
                    nesting += 1;
                    score += nesting;
                }
                ['}', ..] if !in_garbage => nesting -= 1,
                ['<', ..] => in_garbage = true,
                ['>', ..] => in_garbage = false,
                ['!', ..] => i += 1,
                _ => {}
            }

            i += 1;
        }

        Option::from(score.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let input = input.trim().chars().collect::<Vec<_>>();

        let mut i = 0;

        let mut garbage = 0;
        let mut in_garbage = false;

        while i < input.len() {
            if in_garbage {
                garbage += 1;
            }

            match input[i..] {
                ['<', ..] => in_garbage = true,
                ['>', ..] => {
                    in_garbage = false;
                    garbage -= 1;
                }
                ['!', ..] => {
                    i += 1;
                    garbage -= 1;
                }
                _ => {}
            }

            i += 1;
        }

        Option::from(garbage.to_string())
    }
}
