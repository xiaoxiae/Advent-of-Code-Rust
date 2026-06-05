//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/25
use crate::util::Day;

pub struct D25;

const CHARS: &[u8] = b"=-012";

fn snafu_to_decimal(number: &str) -> i64 {
    let mut total: i64 = 0;
    for (i, ch) in number.bytes().rev().enumerate() {
        let idx = CHARS.iter().position(|&c| c == ch).unwrap() as i64;
        total += (idx - 2) * 5i64.pow(i as u32);
    }
    total
}

fn decimal_to_snafu(mut number: i64) -> String {
    let mut i: i64 = 1;
    while i <= number {
        i *= 5;
    }
    i /= 5;

    // get the powers of 5
    let mut powers: Vec<i64> = Vec::new();
    while i != 0 {
        powers.push(number / i);
        number %= i;
        i /= 5;
    }

    while !powers.iter().all(|&d| (-2..=2).contains(&d)) {
        let mut i = powers.len() as isize - 1;
        while i >= 0 {
            let ui = i as usize;
            if powers[ui] >= 3 {
                if i == 0 {
                    powers.insert(0, 0);
                    i += 1;
                }
                let ui = i as usize;
                powers[ui] -= 5;
                powers[ui - 1] += 1;
            }
            i -= 1;
        }
    }

    powers
        .iter()
        .map(|&p| CHARS[(p + 2) as usize] as char)
        .collect()
}

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total: i64 = input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(snafu_to_decimal)
            .sum();
        Some(decimal_to_snafu(total))
    }
}
