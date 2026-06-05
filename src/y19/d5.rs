//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/05
use crate::util::Day;
use crate::y19_intcode::Intcode;

pub struct D5;

impl D5 {
    /// Run the diagnostic program with the given input value, returning the last
    /// produced output (the diagnostic code, matching the final line the original
    /// Python printed).
    fn diagnostic(input: &str, input_value: i64) -> i64 {
        let mut c = Intcode::from_input(input);
        c.input(input_value);
        let outputs = c.run_collect();
        *outputs.last().unwrap()
    }
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(D5::diagnostic(input, 1).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(D5::diagnostic(input, 5).to_string())
    }
}
