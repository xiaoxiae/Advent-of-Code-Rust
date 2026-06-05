//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/09
use crate::util::Day;
use crate::y19_intcode::Intcode;

pub struct D9;

fn run_with_input(input: &str, value: i64) -> i64 {
    let mut c = Intcode::from_input(input);
    c.input(value);
    let outputs = c.run_collect();
    *outputs.last().unwrap()
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(run_with_input(input, 1).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(run_with_input(input, 2).to_string())
    }
}
