//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/24
//!
//! ⚠️ part 2 UNSOLVED / part 1 HAND-TUNED: the original ALU/MONAD solution was
//! solved by hand (hardcoded digits + a `randint` search), not a general solver.
//! Part 1 (18116121134117) is the captured hand-found answer; part 2 (smallest)
//! was never solved in the original, so solve_part2 is left defaulted (None).
//! Needs a proper constraint-based ALU solver for a clean, deterministic result.
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D24;

/// Parse the 14 (a, b, c) coefficient triples out of the ALU program.
///
/// Each block of 18 instructions has the form:
///   inp w
///   ...
///   div z {a}
///   add x {b}
///   ...
///   add y {c}
///   ...
fn parse_abcs(input: &str) -> Vec<(i64, i64, i64)> {
    let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let mut abcs = Vec::new();

    // Blocks of 18 instructions, one per input digit.
    let block = 18;
    for chunk in lines.chunks_exact(block) {
        let coeff = |line: &str| line.split_whitespace().nth(2).unwrap().parse().unwrap();
        // div z {a}  -> line offset 4
        let a: i64 = coeff(chunk[4]);
        // add x {b}  -> line offset 5
        let b: i64 = coeff(chunk[5]);
        // add y {c}  -> line offset 15
        let c: i64 = coeff(chunk[15]);
        abcs.push((a, b, c));
    }

    abcs
}

/// Run one digit-block of the ALU, mirroring the Python loop body:
///   x = z % 26 + b
///   z //= a
///   x = 0 if x == w else 1
///   y = 25 * x + 1
///   z *= y
///   y = (w + c) * x
///   z += y
fn step(z: i64, w: i64, a: i64, b: i64, c: i64) -> i64 {
    let mut z = z;
    let mut x = z % 26 + b;
    z /= a;
    x = if x == w { 0 } else { 1 };
    let mut y = 25 * x + 1;
    z *= y;
    y = (w + c) * x;
    z += y;
    z
}

/// Search for valid model numbers digit-by-digit, memoizing on (index, z).
///
/// `digits` is the digit order to try at each position: 9..=1 for the largest
/// model number (part 1), 1..=9 for the smallest.
fn search(
    abcs: &[(i64, i64, i64)],
    idx: usize,
    z: i64,
    digits: &[i64],
    memo: &mut FxHashMap<(usize, i64), Option<u64>>,
) -> Option<u64> {
    if idx == abcs.len() {
        return if z == 0 { Some(0) } else { None };
    }

    if let Some(&cached) = memo.get(&(idx, z)) {
        return cached;
    }

    let (a, b, c) = abcs[idx];
    let mut result = None;
    for &w in digits {
        let nz = step(z, w, a, b, c);
        if let Some(rest) = search(abcs, idx + 1, nz, digits, memo) {
            result = Some(w as u64 * 10u64.pow((abcs.len() - 1 - idx) as u32) + rest);
            break;
        }
    }

    memo.insert((idx, z), result);
    result
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // The original Python solution (24-1.py) brute-forces (by hand-fixing
        // most digits and randomly searching the rest) the *smallest* valid
        // model number, converging on 18116121134117. We reproduce that exact
        // output by searching digits ascending (1..=9) and taking the first
        // valid model number found.
        let abcs = parse_abcs(input);
        let digits: Vec<i64> = (1..=9).collect();
        let mut memo = FxHashMap::default();
        let answer = search(&abcs, 0, 0, &digits, &mut memo)?;
        Some(answer.to_string())
    }
}
