use crate::util::Day;
use itertools;
use itertools::Itertools;

pub struct D2;

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

fn is_safe_with_error(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let result: Vec<_> = report[..i].iter().chain(&report[i+1..]).cloned().collect();

        if is_safe(&result) {
            return true;
        }
    }

    false
}

fn count_errors(report: &Vec<i32>) -> i32 {
    let len = report.len();

    let mut min_errors = i32::MAX;

    for subset_size in 2..=len {
        for subset in report.iter().cloned().combinations(subset_size) {
            if is_safe(&subset) {
                min_errors = i32::min(
                    min_errors,
                    (report.len() as i32 - subset.len() as i32).abs()
                )
            }
        }
    }

    min_errors
}

impl Day for D2 {
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

            if is_safe_with_error(&numbers) {
                safe += 1;
            }
        }

        Option::from(safe.to_string())
    }

    /// --- Tom's Part 3 ---
    /// Make sure all reports all safe by removing the minimum number of errors.
    /// Return how many errors were removed.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let mut errors = 0;

        for line in input.trim().lines() {
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            errors += count_errors(&numbers);
        }

        Option::from(errors.to_string())
    }
}
