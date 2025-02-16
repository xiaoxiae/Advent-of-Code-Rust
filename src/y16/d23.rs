use crate::util::Day;

pub struct D23;


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
}


#[derive(Debug)]
enum Instruction {
    Copy(Operand, Operand),
    Increment(Operand),
    Decrement(Operand),
    JumpIf(Operand, Operand),
    Toggle(Operand),
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.trim().lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let instruction = match parts[0] {
            "inc" => Instruction::Increment(Operand::parse(parts[1])),
            "dec" => Instruction::Decrement(Operand::parse(parts[1])),
            "cpy" => Instruction::Copy(
                Operand::parse(parts[1]),
                Operand::parse(parts[2]),
            ),
            "jnz" => Instruction::JumpIf(
                Operand::parse(parts[1]),
                Operand::parse(parts[2]),
            ),
            "tgl" => Instruction::Toggle(Operand::parse(parts[1])),
            _ => panic!("Invalid instruction!"),
        };

        instructions.push(instruction);
    }

    instructions
}

fn solve(instructions: &mut Vec<Instruction>, registers: &mut Vec<isize>) {
    let mut ip: usize = 0;

    let mut last_b = 0;

    while ip < instructions.len() {
        match &instructions[ip] {
            Instruction::Copy(r_from, r_to) => {
                match r_to {
                    Operand::Value(_) => {}  // invalid, so we skip
                    Operand::Register(r_to) => {
                        match r_from {
                            Operand::Register(r_from) => registers[*r_to] = registers[*r_from],
                            Operand::Value(v_from) => registers[*r_to] = *v_from,
                        }
                    }
                }
            }
            Instruction::Increment(r) => match r {
                Operand::Register(r) => registers[*r] += 1,
                Operand::Value(_) => {}  // invalid
            },
            Instruction::Decrement(r) => match r {
                Operand::Register(r) => registers[*r] -= 1,
                Operand::Value(_) => {}  // invalid
            },
            Instruction::JumpIf(v, d) => {
                let d = match d {
                    Operand::Register(r) => registers[*r],
                    Operand::Value(d) => *d,
                };

                let v = match v {
                    Operand::Register(r) => registers[*r],
                    Operand::Value(v) => *v,
                };

                if v != 0 {
                    ip = (ip as isize + d) as usize;
                    continue;
                }
            }
            Instruction::Toggle(d) => {
                let d = match d {
                    Operand::Register(r) => registers[*r],
                    Operand::Value(d) => *d,
                };

                let p = ip as isize + d;

                // toggling outside program does nothing
                if p < 0 || p >= instructions.len() as isize {
                    ip += 1;
                    continue;
                }


                let p = p as usize;

                let new = match instructions[p] {
                    Instruction::Increment(v) => Instruction::Decrement(v),
                    Instruction::Decrement(v) => Instruction::Increment(v),
                    Instruction::Toggle(v) => Instruction::Increment(v),
                    Instruction::Copy(a, b) => Instruction::JumpIf(a, b),
                    Instruction::JumpIf(a, b) => Instruction::Copy(a, b),
                };

                instructions[p] = new;
            }
        }

        ip += 1;
    }
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut registers = vec![7, 0, 0, 0];
        let mut instructions = parse(input);

        solve(&mut instructions, &mut registers);

        Option::from(registers[0].to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // printing the registers for changing register b does this:
        // [8, 8, 0, 0]
        // [8, 7, 0, 0]
        // [56, 6, 0, 0]
        // [336, 5, 0, 0]
        // [1680, 4, 0, 0]
        // [6720, 3, 0, 0]
        // [20160, 2, 0, 0]
        // [40320, 1, 0, 0]
        // after which the result is 45503 = 40320 + 5183
        // the addition holds true for larger initial a values

        let mut registers = vec![7, 0, 0, 0];
        let mut instructions = parse(input);

        solve(&mut instructions, &mut registers);

        let delta = registers[0] as usize - (1..=7).product::<usize>();
        let result = (1..=12).product::<usize>() + delta;

        Option::from(result.to_string())
    }
}
