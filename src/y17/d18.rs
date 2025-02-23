use crate::util::Day;
use std::collections::VecDeque;

pub struct D18;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Register(usize),
    Value(isize),
}

impl Operand {
    fn parse(register: &str) -> Self {
        match register.parse::<isize>() {
            Ok(v) => Operand::Value(v),
            Err(_) => Operand::Register(register.chars().next().unwrap() as usize - 'a' as usize),
        }
    }

    fn evaluate(&self, registers: &Vec<isize>) -> isize {
        match self {
            Operand::Register(r) => registers[*r],
            Operand::Value(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let instruction = match parts.as_slice() {
            ["snd", x] => Instruction::Snd(Operand::parse(x)),
            ["set", x, y] => Instruction::Set(Operand::parse(x), Operand::parse(y)),
            ["add", x, y] => Instruction::Add(Operand::parse(x), Operand::parse(y)),
            ["mul", x, y] => Instruction::Mul(Operand::parse(x), Operand::parse(y)),
            ["mod", x, y] => Instruction::Mod(Operand::parse(x), Operand::parse(y)),
            ["rcv", x] => Instruction::Rcv(Operand::parse(x)),
            ["jgz", x, y] => Instruction::Jgz(Operand::parse(x), Operand::parse(y)),
            _ => unreachable!(),
        };

        instructions.push(instruction);
    }

    instructions
}

fn run(
    instructions: &Vec<Instruction>,
    ip: &mut isize,
    registers: &mut Vec<isize>,
    queue: &mut VecDeque<isize>,
    other_queue: &mut VecDeque<isize>,
) {
    loop {
        match instructions[*ip as usize] {
            Instruction::Snd(v) => {
                other_queue.push_back(v.evaluate(&registers));
            }
            Instruction::Set(Operand::Register(r), v) => {
                registers[r] = v.evaluate(&registers);
            }
            Instruction::Add(Operand::Register(r), v) => {
                registers[r] += v.evaluate(&registers);
            }
            Instruction::Mul(Operand::Register(r), v) => {
                registers[r] *= v.evaluate(&registers);
            }
            Instruction::Mod(Operand::Register(r), v) => {
                registers[r] %= v.evaluate(&registers);
            }
            Instruction::Rcv(Operand::Register(r)) => {
                if queue.is_empty() {
                    return;
                }

                registers[r] = queue.pop_front().unwrap();
            }
            Instruction::Jgz(r, v) => {
                if r.evaluate(&registers) > 0 {
                    *ip += v.evaluate(&registers);
                    continue;
                }
            }
            _ => unreachable!(),
        }

        *ip += 1;
    }
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut instructions = parse_instructions(input);

        let mut registers = vec![0isize; 26];
        let mut last_sound = None;
        let mut ip: isize = 0;

        loop {
            match instructions[ip as usize] {
                Instruction::Snd(Operand::Register(r)) => {
                    last_sound.replace(registers[r]);
                }
                Instruction::Set(Operand::Register(r), v) => {
                    registers[r] = v.evaluate(&registers);
                }
                Instruction::Add(Operand::Register(r), v) => {
                    registers[r] += v.evaluate(&registers);
                }
                Instruction::Mul(Operand::Register(r), v) => {
                    registers[r] *= v.evaluate(&registers);
                }
                Instruction::Mod(Operand::Register(r), v) => {
                    registers[r] %= v.evaluate(&registers);
                }
                Instruction::Rcv(v) => {
                    if v.evaluate(&registers) != 0 {
                        return last_sound.map(|v| v.to_string());
                    }
                }
                Instruction::Jgz(r, v) => {
                    if r.evaluate(&registers) > 0 {
                        ip += v.evaluate(&registers);
                        continue;
                    }
                }
                _ => unreachable!(),
            }

            ip += 1;
        }
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut instructions = parse_instructions(input);

        let mut q1 = VecDeque::new();
        let mut ip1 = 0;
        let mut r1 = vec![0isize; 26];
        r1['p' as usize - 'a' as usize] = 0;

        let mut q2 = VecDeque::new();
        let mut ip2 = 0;
        let mut r2 = vec![0isize; 26];
        r2['p' as usize - 'a' as usize] = 1;

        let mut p1_sent = 0;

        loop {
            run(&instructions, &mut ip1, &mut r1, &mut q1, &mut q2);

            run(&instructions, &mut ip2, &mut r2, &mut q2, &mut q1);
            p1_sent += q1.len(); // program 1 is the 2nd one /:

            if q1.is_empty() && q2.is_empty() {
                return Option::from(p1_sent.to_string());
            }
        }
    }
}
