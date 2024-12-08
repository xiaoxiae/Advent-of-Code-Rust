use crate::util::Day;

pub struct Day2;

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = (report[0] - report[1]).signum();

    for i in 0..report.len() - 1 {
        let delta = report[i] - report[i + 1];

        if delta.signum() != sign {
            return false;
        } else if !(1 <= delta.abs() && delta.abs() <= 3) {
            return false;
        }
    }

    true
}

fn is_any_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let result: Vec<_> = report[..i].iter().chain(&report[i+1..]).cloned().collect();

        if is_safe(&result) {
            return true;
        }
    }

    false
}

impl Day for Day2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut safe = 0;

        for line in input.trim().lines() {
            let numbers = line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if is_safe(&numbers) {
                safe += 1;
            }
        }

        Option::from(safe.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut safe = 0;

        for line in input.trim().lines() {
            let numbers = line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if is_any_safe(&numbers) {
                safe += 1;
            }
        }

        Option::from(safe.to_string())
    }
}
