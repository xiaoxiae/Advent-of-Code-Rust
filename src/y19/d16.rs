//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/16
//!
//! ⚠️ SLOW (~90s): part 2 is an FFT over 6.5M digits for 100 phases (faithful to
//! the original's per-element prefix-sum approach). Flagged for manual
//! optimization. The original Python part 2 was too slow to finish, so its answer
//! (12482168) has no Python oracle; part 1 (44098263) matched Python.
use crate::util::Day;

pub struct D16;

/// Calculate the sum of a segment [s, e] using prefix sums.
fn get_prefix_sum(s: i64, e: i64, prefixes: &[i64]) -> i64 {
    // restrict start and end
    let s = s.max(0);
    let e = e.min(prefixes.len() as i64 - 1);

    prefixes[e as usize] - if s - 1 < 0 { 0 } else { prefixes[(s - 1) as usize] }
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

const PATTERN: [i64; 4] = [0, 1, 0, -1];
const DELTA: i64 = 1;

/// Run the FFT-like transform for the given list, 100 phases, and return the
/// resulting list.
fn run(mut input_list: Vec<i64>) -> Vec<i64> {
    let n = input_list.len();
    let mut prefixes = vec![0i64; n];
    let mut output_list = vec![0i64; n];

    for _ in 0..100 {
        // calculate prefix sums and zero the output list
        let mut total = 0i64;
        for i in 0..n {
            total += input_list[i];
            prefixes[i] = total;
            output_list[i] = 0;
        }

        for i in 0..n {
            let mut index = DELTA / (i as i64 + 1); // index in pattern
            let mut j = -DELTA; // starting position in the list

            while j < n as i64 {
                let s = j;
                let e = j + i as i64;

                // ignore nonsensical values
                if s >= 0 || e >= 0 {
                    let prefix_sum = get_prefix_sum(s, e, &prefixes);
                    output_list[i] +=
                        prefix_sum * PATTERN[(index.rem_euclid(PATTERN.len() as i64)) as usize];
                    index += 1;
                }

                j += i as i64 + 1;
            }

            output_list[i] = output_list[i].abs() % 10;
        }

        std::mem::swap(&mut input_list, &mut output_list);
    }

    input_list
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let input_list = parse(input);
        let result = run(input_list);

        let answer: String = result[..8].iter().map(|d| d.to_string()).collect();
        Some(answer)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let base = parse(input);

        // input_list = base * 10000
        let input_list = base.repeat(10000);

        // pos = int("".join(input_list[:7]))
        let pos: usize = input_list[..7]
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        let result = run(input_list);

        let answer: String = result[pos..pos + 8].iter().map(|d| d.to_string()).collect();
        Some(answer)
    }
}
