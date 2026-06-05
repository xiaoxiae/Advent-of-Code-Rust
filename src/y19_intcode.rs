//! Shared Intcode virtual machine for Advent of Code 2019.
//!
//! Consolidates the Intcode interpreter that the original Python embedded
//! separately in each of days 2, 5, 7, 9, 11, 13, 15, 17, 19, 21. The core is the
//! full Intcode spec (opcodes 1-9, position/immediate/relative modes, relative
//! base, auto-extending memory, i64 cells). Per-day driver logic (amplifier
//! feedback loops, the painting robot, the breakout game, ASCII I/O, ...) lives in
//! the individual `src/y19/dN.rs` files and is built on top of this `step` API.

use std::collections::VecDeque;

/// The result of running the VM until it next pauses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// The program produced an output value.
    Output(i64),
    /// The program executed an input instruction but the input queue was empty.
    /// The instruction pointer is left on the input instruction, so pushing an
    /// input and calling `run` again resumes correctly.
    NeedInput,
    /// The program executed opcode 99.
    Halt,
}

#[derive(Clone)]
pub struct Intcode {
    mem: Vec<i64>,
    ip: usize,
    base: i64,
    inputs: VecDeque<i64>,
    halted: bool,
}

impl Intcode {
    /// Parse a comma-separated program (the puzzle input).
    pub fn parse(input: &str) -> Vec<i64> {
        input
            .trim()
            .split(',')
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect()
    }

    /// Build a VM from a program. Convenience: `Intcode::from_input(input)`.
    pub fn new(program: &[i64]) -> Self {
        Intcode {
            mem: program.to_vec(),
            ip: 0,
            base: 0,
            inputs: VecDeque::new(),
            halted: false,
        }
    }

    /// Build a VM directly from the puzzle input string.
    pub fn from_input(input: &str) -> Self {
        Intcode::new(&Intcode::parse(input))
    }

    /// Queue an input value.
    pub fn input(&mut self, v: i64) {
        self.inputs.push_back(v);
    }

    /// Queue many input values.
    pub fn input_all<I: IntoIterator<Item = i64>>(&mut self, vs: I) {
        self.inputs.extend(vs);
    }

    /// Queue an ASCII string as input (each byte as a value).
    pub fn input_ascii(&mut self, s: &str) {
        self.inputs.extend(s.bytes().map(|b| b as i64));
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    /// Read memory at `addr` (auto-extending).
    pub fn get(&mut self, addr: usize) -> i64 {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr]
    }

    /// Write memory at `addr` (auto-extending).
    pub fn set(&mut self, addr: usize, v: i64) {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = v;
    }

    fn read(&mut self, ip_off: usize, mode: i64) -> i64 {
        let raw = self.get(self.ip + ip_off);
        match mode {
            0 => self.get(raw as usize),         // position
            1 => raw,                            // immediate
            2 => self.get((self.base + raw) as usize), // relative
            _ => panic!("bad parameter mode {mode}"),
        }
    }

    /// The write address for parameter `ip_off`, honoring position/relative mode.
    fn write_addr(&mut self, ip_off: usize, mode: i64) -> usize {
        let raw = self.get(self.ip + ip_off);
        match mode {
            0 => raw as usize,
            2 => (self.base + raw) as usize,
            _ => panic!("bad write mode {mode}"),
        }
    }

    /// Run until the next `Output`, `NeedInput`, or `Halt`.
    pub fn run(&mut self) -> Step {
        loop {
            let instr = self.get(self.ip);
            let opt = instr % 100;
            let m1 = (instr / 100) % 10;
            let m2 = (instr / 1000) % 10;
            let m3 = (instr / 10000) % 10;

            match opt {
                1 => {
                    let v = self.read(1, m1) + self.read(2, m2);
                    let a = self.write_addr(3, m3);
                    self.set(a, v);
                    self.ip += 4;
                }
                2 => {
                    let v = self.read(1, m1) * self.read(2, m2);
                    let a = self.write_addr(3, m3);
                    self.set(a, v);
                    self.ip += 4;
                }
                3 => {
                    let Some(v) = self.inputs.pop_front() else {
                        return Step::NeedInput; // leave ip on this instruction
                    };
                    let a = self.write_addr(1, m1);
                    self.set(a, v);
                    self.ip += 2;
                }
                4 => {
                    let v = self.read(1, m1);
                    self.ip += 2;
                    return Step::Output(v);
                }
                5 => {
                    if self.read(1, m1) != 0 {
                        self.ip = self.read(2, m2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.read(1, m1) == 0 {
                        self.ip = self.read(2, m2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let v = (self.read(1, m1) < self.read(2, m2)) as i64;
                    let a = self.write_addr(3, m3);
                    self.set(a, v);
                    self.ip += 4;
                }
                8 => {
                    let v = (self.read(1, m1) == self.read(2, m2)) as i64;
                    let a = self.write_addr(3, m3);
                    self.set(a, v);
                    self.ip += 4;
                }
                9 => {
                    self.base += self.read(1, m1);
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    return Step::Halt;
                }
                _ => panic!("bad opcode {opt} at ip {}", self.ip),
            }
        }
    }

    /// Run to halt, collecting every output. Panics if the program asks for input
    /// that has not been queued (queue inputs up front with `input`/`input_all`).
    pub fn run_collect(&mut self) -> Vec<i64> {
        let mut out = Vec::new();
        loop {
            match self.run() {
                Step::Output(v) => out.push(v),
                Step::Halt => return out,
                Step::NeedInput => panic!("Intcode needs input but queue is empty"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_mem(prog: &[i64]) -> Vec<i64> {
        let mut c = Intcode::new(prog);
        c.run_collect();
        c.mem
    }

    #[test]
    fn day2_examples() {
        assert_eq!(run_mem(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[0], 3500);
        assert_eq!(run_mem(&[1, 0, 0, 0, 99])[0], 2);
        assert_eq!(run_mem(&[2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(run_mem(&[2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn day5_io_and_modes() {
        // immediate mode add/mul leaving 1101*... ; echo: input -> output
        let mut c = Intcode::from_input("3,0,4,0,99");
        c.input(42);
        assert_eq!(c.run(), Step::Output(42));
        // equal-to-8 (position mode): input 8 -> 1, else 0
        let prog = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut a = Intcode::from_input(prog);
        a.input(8);
        assert_eq!(a.run(), Step::Output(1));
        let mut b = Intcode::from_input(prog);
        b.input(7);
        assert_eq!(b.run(), Step::Output(0));
    }

    #[test]
    fn day9_relative_base_and_bignum() {
        // quine: outputs a copy of itself
        let quine = [
            109i64, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut c = Intcode::new(&quine);
        assert_eq!(c.run_collect(), quine.to_vec());
        // outputs a 16-digit number
        let mut d = Intcode::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(d.run_collect()[0].to_string().len(), 16);
        // outputs the large middle value
        let mut e = Intcode::new(&[104, 1125899906842624, 99]);
        assert_eq!(e.run_collect(), vec![1125899906842624]);
    }

    #[test]
    fn needs_input_then_resumes() {
        let mut c = Intcode::from_input("3,0,4,0,99");
        assert_eq!(c.run(), Step::NeedInput);
        c.input(7);
        assert_eq!(c.run(), Step::Output(7));
        assert_eq!(c.run(), Step::Halt);
    }
}
