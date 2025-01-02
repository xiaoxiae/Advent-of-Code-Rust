use crate::util::Day;
use itertools::Itertools;

pub struct D10;

fn look_and_say(input: &Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut intermediate = Vec::new();

    // Flatten the input into a single list
    for &(digit, count) in input {
        intermediate.push(count);
        intermediate.push(digit);
    }

    let mut result = Vec::new();
    let mut i = 0;

    // Group consecutive identical values
    while i < intermediate.len() {
        let current = intermediate[i];
        let mut count = 1;

        // Count occurrences of the current value
        while i + count < intermediate.len() && intermediate[i + count] == current {
            count += 1;
        }

        // Append the grouped result
        result.push((current, count as u8));

        // Move to the next group
        i += count;
    }

    result
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut input = input
        .trim()
        .chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(_, group)| {
            let items = group.collect::<Vec<_>>();

            (items[0].to_digit(10).unwrap() as u8, items.len() as u8)
        })
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        input = look_and_say(&input);
    }

    input.into_iter().map(|(_, c)| c as usize).sum::<usize>()
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let length = solve(input, 40);

        Option::from(length.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let length = solve(input, 50);

        Option::from(length.to_string())
    }
}
