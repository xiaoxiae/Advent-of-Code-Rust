//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/19
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};

pub struct D19;

/// Run a fresh copy of the program with the two coordinate inputs and return
/// its single output value (0 or 1).
fn beam(prog: &[i64], x: i64, y: i64) -> i64 {
    let mut c = Intcode::new(prog);
    c.input(x);
    c.input(y);
    match c.run() {
        Step::Output(v) => v,
        _ => panic!("expected output"),
    }
}

/// Return the proper start of the beam (first row where the beam is present).
fn get_beam_start(prog: &[i64]) -> (i64, i64) {
    let mut d = 1;
    loop {
        for i in 0..d {
            if beam(prog, i, d) == 1 {
                return (i, d);
            } else if beam(prog, d, i) == 1 {
                return (d, i);
            }
        }
        d += 1;
    }
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        let mut total = 0;
        for x in 0..50 {
            for y in 0..50 {
                total += beam(&prog, x, y);
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let prog = Intcode::parse(input);
        const R: i64 = 99; // delta of x, y

        let (mut x, mut y) = get_beam_start(&prog);

        loop {
            // go all the way to the right of the beam
            while beam(&prog, x, y) == 1 {
                x += 1;
            }

            // if the 100 x 100 box fits, return the result
            if x - R - 1 >= 0 && beam(&prog, x - R - 1, y + R) == 1 {
                return Some(((x - R - 1) * 10000 + y).to_string());
            }

            y += 1;
        }
    }
}
