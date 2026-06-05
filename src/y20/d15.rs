//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/15
use crate::util::Day;

pub struct D15;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut numbers = parse(input);

        // rfind(numbers[:-1], n): find last index in numbers[..len-1] where value == n,
        // else return len(numbers[:-1]) == len(numbers) - 1.
        while numbers.len() < 2020 {
            let last = *numbers.last().unwrap();
            let slice_len = numbers.len() - 1; // length of numbers[:-1]
            // default return value len(numbers[:-1])
            let rfind = numbers[..slice_len]
                .iter()
                .rposition(|&x| x == last)
                .unwrap_or(slice_len);
            // append len(numbers) - 1 - rfind, where len(numbers) is current length (slice_len+1)
            let next = (numbers.len() as i64) - 1 - (rfind as i64);
            numbers.push(next);
        }

        Some(numbers.last().unwrap().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let numbers = parse(input);

        // positions[n] stores last two indices (Python keeps full list but only uses last two).
        // Use (count, last, second_last).
        let limit: i64 = 30_000_000;
        let mut last_pos: Vec<i32> = vec![-1; limit as usize + 1];
        let mut prev_pos: Vec<i32> = vec![-1; limit as usize + 1];

        for (i, &num) in numbers.iter().enumerate() {
            let v = num as usize;
            prev_pos[v] = last_pos[v];
            last_pos[v] = i as i32;
        }

        let mut n: i64 = 0;
        let mut i: i64 = numbers.len() as i64;

        while i < limit - 1 {
            let v = n as usize;
            // append i to positions[n]
            prev_pos[v] = last_pos[v];
            last_pos[v] = i as i32;

            if prev_pos[v] == -1 {
                // len(positions[n]) == 1
                n = 0;
            } else {
                n = (last_pos[v] - prev_pos[v]) as i64;
            }

            i += 1;
        }

        Some(n.to_string())
    }
}
