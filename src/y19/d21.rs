//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/21
//!
//! ⚠️ part 2 UNSOLVED: the original springscript was buggy (the droid falls, no
//! value printed); `solve_part2` returns None pending a manual rewrite. Part 1
//! (19354818) is verified.
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};

pub struct D21;

/// Feed the springscript program, then read outputs until the VM emits a value
/// >= 256 (the hull damage). Mirrors the Python `while val < 256` loop.
fn run_springscript(input: &str, script: &str) -> Option<String> {
    let mut c = Intcode::from_input(input);
    c.input_ascii(script);

    // Drain any prompt output / consume queued input until the VM halts or
    // produces the hull-damage value. The Python keeps calling run() (feeding 0
    // once the script is loaded) and stops at the first output >= 256.
    let mut ascii = String::new();
    loop {
        match c.run() {
            Step::Output(v) => {
                if v >= 256 {
                    return Some(v.to_string());
                }
                if let Some(ch) = char::from_u32(v as u32) {
                    ascii.push(ch);
                }
            }
            Step::NeedInput => c.input(0),
            Step::Halt => break,
        }
    }

    // No hull-damage value was produced (the droid fell); faithfully return the
    // ASCII rendering the Python would have printed.
    Some(ascii)
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let script = "NOT C J\nAND D J\nNOT A T\nOR T J\nWALK\n";
        run_springscript(input, script)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // The original part-2 springscript was buggy (the droid falls; no
        // hull-damage value is produced). Left unsolved pending a manual rewrite.
        let _ = input;
        None
    }
}
