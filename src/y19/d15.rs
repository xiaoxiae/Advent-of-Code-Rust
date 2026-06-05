//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/15
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};
use rustc_hash::FxHashMap;

pub struct D15;

// directions[d] for d in 1..=4
const DIRECTIONS: [(i64, i64); 5] = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0)];

/// Push a single input value and run until the VM produces one output,
/// mirroring the Python `Computer.run(input_value)` semantics.
fn step(c: &mut Intcode, input_value: i64) -> i64 {
    c.input(input_value);
    loop {
        match c.run() {
            Step::Output(v) => return v,
            Step::Halt => return -1, // Python returns None on halt; unused here.
            Step::NeedInput => {
                // Should not happen: we always queue one input before running.
                panic!("Intcode needs more input");
            }
        }
    }
}

/// Reverse of a move (the input that undoes it).
fn reverse(move_: i64) -> i64 {
    match move_ {
        2 => 1,
        1 => 2,
        3 => 4,
        _ => 3,
    }
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut computer = Intcode::from_input(input);
        let mut area: FxHashMap<(i64, i64), char> = FxHashMap::default();
        area.insert((0, 0), '.');

        let mut stack: Vec<Vec<i64>> = vec![vec![]];

        while let Some(moves) = stack.pop() {
            let (mut x, mut y) = (0i64, 0i64);

            // get to the end spot
            for &move_ in &moves {
                step(&mut computer, move_);
                x += DIRECTIONS[move_ as usize].0;
                y += DIRECTIONS[move_ as usize].1;
            }

            // run the robot in each of the 4 directions
            for d in 1..5i64 {
                let x_n = x + DIRECTIONS[d as usize].0;
                let y_n = y + DIRECTIONS[d as usize].1;

                if *area.get(&(x_n, y_n)).unwrap_or(&' ') != ' ' {
                    continue;
                }

                let status = step(&mut computer, d);

                if status == 0 {
                    area.insert((x_n, y_n), '#');
                } else if status == 1 {
                    area.insert((x_n, y_n), '.');
                    let mut next = moves.clone();
                    next.push(d);
                    stack.push(next);
                    step(&mut computer, reverse(d));
                } else {
                    return Some((moves.len() + 1).to_string());
                }
            }

            // get back to the beginning
            for &move_ in moves.iter().rev() {
                step(&mut computer, reverse(move_));
            }
        }

        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut computer = Intcode::from_input(input);

        // Faithful translation: bfs() is called twice on the same shared computer.
        // The first call explores until it finds the oxygen system and returns early
        // (None); the second call runs to completion (no early return because the
        // oxygen tile is now reached differently) and returns the max distance.
        let bfs = |computer: &mut Intcode| -> Option<usize> {
            let mut max_distance = 0usize;
            let mut area: FxHashMap<(i64, i64), char> = FxHashMap::default();
            area.insert((0, 0), '.');

            let mut stack: Vec<Vec<i64>> = vec![vec![]];

            while let Some(moves) = stack.pop() {
                let (mut x, mut y) = (0i64, 0i64);

                max_distance = max_distance.max(moves.len());

                for &move_ in &moves {
                    step(computer, move_);
                    x += DIRECTIONS[move_ as usize].0;
                    y += DIRECTIONS[move_ as usize].1;
                }

                for d in 1..5i64 {
                    let x_n = x + DIRECTIONS[d as usize].0;
                    let y_n = y + DIRECTIONS[d as usize].1;

                    if *area.get(&(x_n, y_n)).unwrap_or(&' ') != ' ' {
                        continue;
                    }

                    let status = step(computer, d);

                    if status == 0 {
                        area.insert((x_n, y_n), '#');
                    } else if status == 1 {
                        area.insert((x_n, y_n), '.');
                        let mut next = moves.clone();
                        next.push(d);
                        stack.push(next);
                        step(computer, reverse(d));
                    } else {
                        return None;
                    }
                }

                for &move_ in moves.iter().rev() {
                    step(computer, reverse(move_));
                }
            }

            Some(max_distance)
        };

        bfs(&mut computer);
        let result = bfs(&mut computer);

        result.map(|v| v.to_string())
    }
}
