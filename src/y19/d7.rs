//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/07
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};
use itertools::Itertools;

pub struct D7;

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        let mut max_input_value = 0;

        for perm in (0..=4).permutations(5) {
            let mut input_value = 0;
            for &phase in &perm {
                let mut c = Intcode::new(&prog);
                c.input(phase);
                c.input(input_value);
                // run until the single output; the amplifier programs each
                // produce exactly one output before halting.
                if let Step::Output(v) = c.run() {
                    input_value = v;
                }
            }

            if max_input_value < input_value {
                max_input_value = input_value;
            }
        }

        Some(max_input_value.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        let mut max_input_value = 0;

        for perm in (5..=9).permutations(5) {
            let mut amplifiers: Vec<Intcode> =
                (0..5).map(|_| Intcode::new(&prog)).collect();

            // queue the phases first
            for (amp, &phase) in amplifiers.iter_mut().zip(&perm) {
                amp.input(phase);
            }

            let mut input_value = 0;
            let mut i = 0usize;

            loop {
                let idx = i % 5;
                amplifiers[idx].input(input_value);
                match amplifiers[idx].run() {
                    Step::Output(v) => {
                        input_value = v;
                        i += 1;
                    }
                    Step::Halt | Step::NeedInput => {
                        if input_value > max_input_value {
                            max_input_value = input_value;
                        }
                        break;
                    }
                }
            }
        }

        Some(max_input_value.to_string())
    }
}
