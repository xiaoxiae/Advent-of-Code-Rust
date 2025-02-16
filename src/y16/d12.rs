use crate::util::Day;

pub struct D12;

#[derive(Debug)]
enum Instruction {
    CopyRegister(usize, usize), // (from, to)
    CopyValue(usize, usize),    // (value, to)
    Increment(usize),           // (register)
    Decrement(usize),           // (register)
    Jump(isize),                // (delta)
    JumpIf(usize, isize),       // (if not zero, delta)
    Nop,                        // <no operation>
}

fn reg_to_idx(register: &str) -> usize {
    register.chars().next().unwrap() as usize - 'a' as usize
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input.trim().lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let instruction = match parts[0] {
            "inc" => Instruction::Increment(reg_to_idx(parts[1])),
            "dec" => Instruction::Decrement(reg_to_idx(parts[1])),
            "cpy" => match parts[1].parse::<usize>() {
                Ok(value) => Instruction::CopyValue(value, reg_to_idx(parts[2])),
                Err(_) => Instruction::CopyRegister(reg_to_idx(parts[1]), reg_to_idx(parts[2])),
            },
            "jnz" => match parts[1].parse::<usize>() {
                Ok(value) => {
                    if value == 0 {
                        Instruction::Nop
                    } else {
                        Instruction::Jump(parts[2].parse::<isize>().unwrap())
                    }
                }
                Err(_) => {
                    Instruction::JumpIf(reg_to_idx(parts[1]), parts[2].parse::<isize>().unwrap())
                }
            },
            _ => panic!(),
        };

        instructions.push(instruction);
    }

    instructions
}

fn solve(instructions: &Vec<Instruction>, registers: &mut Vec<usize>) {
    let mut ip: usize = 0;
    while ip < instructions.len() {
        match &instructions[ip] {
            Instruction::CopyRegister(r_from, r_to) => registers[*r_to] = registers[*r_from],
            Instruction::CopyValue(v, r_to) => registers[*r_to] = *v,
            Instruction::Increment(r) => registers[*r] += 1,
            Instruction::Decrement(r) => registers[*r] -= 1,
            Instruction::Jump(d) => {
                ip = (ip as isize + d) as usize;
                continue;
            }
            Instruction::JumpIf(r, d) => {
                if registers[*r] != 0 {
                    ip = (ip as isize + d) as usize;
                    continue;
                }
            }
            Instruction::Nop => {}
        }

        ip += 1;
    }
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut registers = vec![0, 0, 0, 0];
        let instructions = parse(input);

        solve(&instructions, &mut registers);

        Option::from(registers[0].to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut registers = vec![0, 0, 1, 0];
        let instructions = parse(input);

        solve(&instructions, &mut registers);

        Option::from(registers[0].to_string())
    }
}
