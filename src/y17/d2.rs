use crate::util::Day;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total: usize = input
            .lines()
            .map(|line| {
                let numbers: Vec<usize> = line.split('\t')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect();
                numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
            })
            .sum();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let total: usize = input
            .lines()
            .map(|line| {
                let mut numbers: Vec<usize> = line.split('\t')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect();
                numbers.sort_unstable();

                for (i, &a) in numbers.iter().enumerate() {
                    for &b in &numbers[i + 1..] {
                        if b % a == 0 {
                            return b / a;
                        }
                    }
                }
                0
            })
            .sum();

        Some(total.to_string())
    }
}
