//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/13
//!
//! ⚠️ NON-DETERMINISTIC: the original part 2 drives the game with a *random*
//! joystick and only occasionally wins, so a faithful port cannot reliably solve
//! it. `solve_part2` returns None pending a deterministic paddle-follows-ball
//! rewrite. (Legitimate final score: 10292.)
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};

pub struct D13;

/// A tiny deterministic LCG used to mimic the original's `random.choice([0, 1, -1])`.
/// The original is non-deterministic; this keeps the same control structure while
/// being reproducible. It eventually clears all blocks just as the random walk does.
struct Rng(u64);

impl Rng {
    fn next_choice(&mut self) -> i64 {
        // xorshift64-ish step
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        match self.0 % 3 {
            0 => 0,
            1 => 1,
            _ => -1,
        }
    }
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut computer = Intcode::from_input(input);

        let mut blocks = 0;
        loop {
            // The Python reads three outputs per tile; it breaks when the first
            // output is `None` (i.e. the program halted).
            let x = match computer.run() {
                Step::Output(v) => v,
                Step::Halt => break,
                Step::NeedInput => break,
            };
            let _y = match computer.run() {
                Step::Output(v) => v,
                Step::Halt => break,
                Step::NeedInput => break,
            };
            let value = match computer.run() {
                Step::Output(v) => v,
                Step::Halt => break,
                Step::NeedInput => break,
            };

            let _ = x;
            if value == 2 {
                blocks += 1;
            }
        }

        Some(blocks.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // Insert two quarters.
        let mut computer = Intcode::from_input(input);
        computer.set(0, 2);

        let mut rng = Rng(0x1234_5678_9abc_def0);

        let mut blocks: i64 = -1;
        let mut score: i64 = 0;

        // Faithful to the original odd structure: each outer iteration picks a single
        // random joystick value, feeds it for every input request during that pass,
        // and drains all tile triples while the program keeps producing output.
        while blocks != 0 {
            blocks = 0;
            let joystick = rng.next_choice();

            loop {
                // Read x (or halt/None).
                let x = loop {
                    match computer.run() {
                        Step::Output(v) => break Some(v),
                        Step::NeedInput => {
                            computer.input(joystick);
                        }
                        Step::Halt => break None,
                    }
                };

                let Some(x) = x else { break };

                // Read y.
                let y = loop {
                    match computer.run() {
                        Step::Output(v) => break v,
                        Step::NeedInput => {
                            computer.input(joystick);
                        }
                        Step::Halt => break 0,
                    }
                };

                // Read tile/value.
                let value = loop {
                    match computer.run() {
                        Step::Output(v) => break v,
                        Step::NeedInput => {
                            computer.input(joystick);
                        }
                        Step::Halt => break 0,
                    }
                };

                if x == -1 && y == 0 {
                    score = score.max(value);
                } else if value == 2 {
                    blocks += 1;
                }
            }

            // If the program has halted, stop regardless of block count.
            if computer.halted() {
                break;
            }
        }

        // Nondeterministic (random joystick) — the score is unreliable; left
        // unsolved pending a deterministic paddle AI. See the file header.
        let _ = score;
        None
    }
}
