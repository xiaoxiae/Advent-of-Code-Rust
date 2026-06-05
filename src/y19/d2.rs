//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/02
use crate::util::Day;
use crate::y19_intcode::Intcode;

pub struct D2;

impl D2 {
    fn run(prog: &[i64], noun: i64, verb: i64) -> i64 {
        let mut c = Intcode::new(prog);
        c.set(1, noun);
        c.set(2, verb);
        c.run_collect();
        c.get(0)
    }
}

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        Some(D2::run(&prog, 12, 2).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        for n in 0..99 {
            for v in 0..99 {
                if D2::run(&prog, n, v) == 19690720 {
                    return Some((100 * n + v).to_string());
                }
            }
        }
        None
    }
}
