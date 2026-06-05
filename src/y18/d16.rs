//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/16
use crate::util::Day;

pub struct D16;

/// The 16 opcodes as functions taking registers `reg` and instruction `inst`.
/// Mirrors the lambda list in the original Python.
const OPCODES: [fn(&[i64], &[i64]) -> i64; 16] = [
    |reg, inst| reg[inst[1] as usize] + reg[inst[2] as usize], // addr
    |reg, inst| reg[inst[1] as usize] + inst[2],               // addi
    |reg, inst| reg[inst[1] as usize] * reg[inst[2] as usize], // mulr
    |reg, inst| reg[inst[1] as usize] * inst[2],               // muli
    |reg, inst| reg[inst[1] as usize] & reg[inst[2] as usize], // banr
    |reg, inst| reg[inst[1] as usize] & inst[2],               // bani
    |reg, inst| reg[inst[1] as usize] | reg[inst[2] as usize], // borr
    |reg, inst| reg[inst[1] as usize] | inst[2],               // bori
    |reg, _inst| reg[_inst[1] as usize],                       // setr
    |_reg, inst| inst[1],                                      // seti
    |reg, inst| (inst[1] > reg[inst[2] as usize]) as i64,      // gtir
    |reg, inst| (reg[inst[1] as usize] > inst[2]) as i64,      // gtri
    |reg, inst| (reg[inst[1] as usize] > reg[inst[2] as usize]) as i64, // gtrr
    |reg, inst| (inst[1] == reg[inst[2] as usize]) as i64,     // eqir
    |reg, inst| (reg[inst[1] as usize] == inst[2]) as i64,     // eqri
    |reg, inst| (reg[inst[1] as usize] == reg[inst[2] as usize]) as i64, // eqrr
];

/// Extract all non-negative integers from a string (like `findall(r"\d+", ...)`).
fn find_ints(s: &str) -> Vec<i64> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push(s[start..i].parse::<i64>().unwrap());
        } else {
            i += 1;
        }
    }
    out
}

/// Whether applying `result` to register at `index` of `before` yields `after`.
fn opt_result_match(before: &[i64], after: &[i64], index: usize, result: i64) -> i64 {
    let mut candidate = before.to_vec();
    candidate[index] = result;
    (candidate == after) as i64
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // Match Python's splitlines(): strip trailing \r, keep blank lines.
        let data: Vec<&str> = input.lines().map(|l| l.trim_end_matches('\r')).collect();

        let mut i = 0usize;
        let mut total = 0i64;
        while !data[i].is_empty() {
            let before = find_ints(data[i]);
            let after = find_ints(data[i + 2]);
            let instruction: Vec<i64> = data[i + 1]
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            // count opcodes that work
            let working: i64 = OPCODES
                .iter()
                .map(|ocode| {
                    opt_result_match(&before, &after, instruction[3] as usize, ocode(&before, &instruction))
                })
                .sum();

            if working >= 3 {
                total += 1;
            }

            i += 4;
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let data: Vec<&str> = input.lines().map(|l| l.trim_end_matches('\r')).collect();

        // Mutable copy of opcodes; matched ones get replaced by a dummy that
        // always fails the comparison (returns None in Python -> here a sentinel).
        let mut opcodes: Vec<Option<fn(&[i64], &[i64]) -> i64>> =
            OPCODES.iter().map(|&f| Some(f)).collect();

        // key: opcode number, value: index into the (original) opcode functions
        let mut opt_mapping: std::collections::HashMap<i64, fn(&[i64], &[i64]) -> i64> =
            std::collections::HashMap::new();

        let mut i = 0usize;
        while !data[i].is_empty() {
            let before = find_ints(data[i]);
            let after = find_ints(data[i + 2]);
            let instruction: Vec<i64> = data[i + 1]
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            // results for each opcode (dummy ones never match)
            let result: Vec<i64> = opcodes
                .iter()
                .map(|maybe| match maybe {
                    Some(ocode) => opt_result_match(
                        &before,
                        &after,
                        instruction[3] as usize,
                        ocode(&before, &instruction),
                    ),
                    None => 0,
                })
                .collect();

            if result.iter().sum::<i64>() == 1 {
                let idx = result.iter().position(|&r| r == 1).unwrap();
                opt_mapping.insert(instruction[0], opcodes[idx].unwrap());
                // replace with dummy so it always fails future comparisons
                opcodes[idx] = None;
            }

            i += 4;
        }

        // perform the program, starting at i + 2 (mirrors Python's range)
        let mut registers = vec![0i64, 0, 0, 0];
        for j in (i + 2)..data.len() {
            let inst: Vec<i64> = data[j]
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let f = opt_mapping[&inst[0]];
            let dst = inst[3] as usize;
            registers[dst] = f(&registers, &inst);
        }

        Some(registers[0].to_string())
    }
}
