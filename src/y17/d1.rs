use crate::util::Day;

pub struct D1;

impl Day for D1 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let data: Vec<char> = input.trim().chars().collect();
        let mut total = 0;

        for i in 0..data.len() {
            let curr = data[i];
            let next = data[(i + 1) % data.len()];
            if curr == next {
                total += curr.to_digit(10)?;
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let data: Vec<char> = input.trim().chars().collect();
        let len = data.len();
        let mut total = 0;

        for i in 0..len {
            let curr = data[i];
            let next = data[(i + len / 2) % len];
            if curr == next {
                total += curr.to_digit(10)?;
            }
        }

        Some(total.to_string())
    }
}
