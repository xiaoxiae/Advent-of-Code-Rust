//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/03
use crate::util::Day;

pub struct D3;

fn count_at(lines: &[&[u8]], i: usize) -> (usize, usize) {
    let mut ones = 0;
    let mut zeroes = 0;
    for line in lines {
        match line[i] {
            b'1' => ones += 1,
            b'0' => zeroes += 1,
            _ => {}
        }
    }
    (ones, zeroes)
}

fn find_set<'a>(input: &[&'a [u8]], inverted: bool) -> &'a [u8] {
    let width = input[0].len();
    let mut l: Vec<&'a [u8]> = input.to_vec();

    for i in 0..width {
        let (ones, zeroes) = count_at(&l, i);

        let target: u8 = if !inverted {
            if zeroes > ones { b'0' } else { b'1' }
        } else if zeroes > ones {
            b'1'
        } else {
            b'0'
        };

        l = l.into_iter().filter(|x| x[i] == target).collect();

        if l.len() == 1 {
            return l[0];
        }
    }

    l[0]
}

fn bin_to_int(bytes: &[u8]) -> u64 {
    let s = std::str::from_utf8(bytes).unwrap();
    u64::from_str_radix(s, 2).unwrap()
}

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&[u8]> = input.trim().lines().map(|l| l.as_bytes()).collect();
        let width = lines[0].len();

        let mut gamma_rate = String::new();
        let mut epsilon_rate = String::new();

        for i in 0..width {
            let (ones, zeroes) = count_at(&lines, i);

            gamma_rate.push(if zeroes > ones { '0' } else { '1' });
            epsilon_rate.push(if zeroes < ones { '0' } else { '1' });
        }

        let gamma = u64::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon = u64::from_str_radix(&epsilon_rate, 2).unwrap();

        Some((gamma * epsilon).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&[u8]> = input.trim().lines().map(|l| l.as_bytes()).collect();

        let a = bin_to_int(find_set(&lines, false));
        let b = bin_to_int(find_set(&lines, true));

        Some((a * b).to_string())
    }
}
