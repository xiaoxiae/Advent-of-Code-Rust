use crate::util::Day;
use rayon::prelude::*;

pub struct Day7;

/// NOTE: we can take a different approach and go from the other side
///  in that case, we can immediately filter out concat (result has to end in that),
///  as well as multiplication (result has to be divisible by it)

fn parse_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .trim()
        .split_terminator("\n")
        .map(|line| {
            let parts = line.split_once(':').unwrap();

            (
                parts.0.parse::<i64>().unwrap(),
                parts
                    .1
                    .split_whitespace()
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

/// Recursive function to check whether the numbers can be satisfied using given operators
fn can_be_satisfied(
    result: i64,
    current: i64,
    numbers: &Vec<i64>,
    i: usize,
    operators: &Vec<fn(i64, i64) -> Option<i64>>,
) -> bool {
    if numbers.len() == i {
        return result == current;
    }

    operators
        .iter()
        .map(|operator| match operator(current, numbers[i]) {
            None => false,
            Some(number) => {
                if result >= current {
                    can_be_satisfied(result, number, numbers, i + 1, operators)
                } else {
                    false
                }
            }
        })
        .any(|b| b)
}

/// Sum all equations that can be satisfied given a list of operator functions
fn sum_equations(
    equations: &Vec<(i64, Vec<i64>)>,
    operators: &Vec<fn(i64, i64) -> Option<i64>>,
) -> i64 {
    equations
        .par_iter()
        .filter_map(|(result, numbers)| {
            if can_be_satisfied(*result, numbers[0], numbers, 1, operators) {
                Some(*result)
            } else {
                None
            }
        })
        .sum::<i64>()
}

impl Day for Day7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let equations = parse_equations(&input);

        let operators: Vec<fn(i64, i64) -> Option<i64>> =
            vec![|a, b| a.checked_add(b), |a, b| a.checked_mul(b)];

        Option::from(sum_equations(&equations, &operators).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let equations = parse_equations(&input);
        let operators: Vec<fn(i64, i64) -> Option<i64>> =
            vec![|a, b| a.checked_add(b), |a, b| a.checked_mul(b), |a, b| {
                let digits = b.ilog10() as i64;

                let result = a
                    .checked_mul(10_i64.pow((digits + 1) as u32))
                    .and_then(|n| n.checked_add(b));

                result
            }];

        Option::from(sum_equations(&equations, &operators).to_string())
    }
}
