//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/04
use crate::util::Day;

pub struct D4;

/// Parses the input as `start,end`.
fn parse_range(input: &str) -> (u32, u32) {
    let (a, b) = input.trim().split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

/// Part 1: digits non-decreasing and at least one digit repeated (>= 2).
fn check_condition_1(mut number: u32) -> bool {
    let mut occurrences = [0u32; 10];
    let mut was_digit_repeated = false;
    let mut previous_digit: u32 = 10;

    while number != 0 {
        let digit = (number % 10) as usize;

        if previous_digit < digit as u32 {
            return false;
        }

        previous_digit = digit as u32;
        occurrences[digit] += 1;

        if occurrences[digit] >= 2 {
            was_digit_repeated = true;
        }

        number /= 10;
    }

    was_digit_repeated
}

/// Part 2: digits non-decreasing and some digit appears exactly twice.
fn check_condition_2(mut number: u32) -> bool {
    let mut occurrences = [0u32; 10];
    let mut previous_digit: u32 = 10;

    while number != 0 {
        let digit = (number % 10) as usize;

        if previous_digit < digit as u32 {
            return false;
        }

        previous_digit = digit as u32;
        occurrences[digit] += 1;

        number /= 10;
    }

    occurrences.iter().any(|&o| o == 2)
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (start, end) = parse_range(input);
        let total = (start..end).filter(|&i| check_condition_1(i)).count();
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (start, end) = parse_range(input);
        let total = (start..end).filter(|&i| check_condition_2(i)).count();
        Some(total.to_string())
    }
}
