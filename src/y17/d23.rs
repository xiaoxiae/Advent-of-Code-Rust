use crate::util::Day;
use crate::y17::d23::Instruction::*;
use crate::y17::d23::Operand::*;

pub struct D23;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Register(usize),
    Value(isize),
}

impl Operand {
    fn parse(register: &str) -> Self {
        match register.parse::<isize>() {
            Ok(v) => Value(v),
            Err(_) => Register(register.chars().next().unwrap() as usize - 'a' as usize),
        }
    }

    fn evaluate(&self, registers: &Vec<isize>) -> isize {
        match self {
            Register(r) => registers[*r],
            Value(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Jnz(Operand, Operand),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let instruction = match parts.as_slice() {
            ["set", x, y] => Set(Operand::parse(x), Operand::parse(y)),
            ["sub", x, y] => Sub(Operand::parse(x), Operand::parse(y)),
            ["mul", x, y] => Mul(Operand::parse(x), Operand::parse(y)),
            ["jnz", x, y] => Jnz(Operand::parse(x), Operand::parse(y)),
            _ => unreachable!(),
        };

        instructions.push(instruction);
    }

    instructions
}

fn is_composite(i: isize) -> bool {
    for j in 2..(i.isqrt() + 1) {
        if i % j == 0 {
            return true;
        }
    }

    false
}

fn fast() -> usize {
    let mut a: usize;
    let mut b: usize;
    let mut c: usize;
    let mut d: usize;
    let mut e: usize;
    let mut f: usize;
    let mut h: usize = 0;

    // (my inputs)
    a = 1;
    b = 57 * 100 + 100000;
    c = b + 17000;

    loop {
        f = 1;
        d = 2;

        // this is just double for-loop over d and e
        // which checks if b is composite...
        'outer: loop {
            e = 2;

            loop {
                if b == d * e {
                    f = 0;
                    break 'outer;
                }

                e += 1;

                if b == e {
                    break;
                }
            }

            if b == d {
                break;
            }

            d += 1;
        }

        // ... and we count them
        if f == 0 {
            h += 1;
        }

        if b == c {
            break;
        }

        b += 17;
    }

    h
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut instructions = parse_instructions(input);

        let mut registers = vec![0isize; 8];
        let mut ip: isize = 0;

        let mut muls = 0;

        loop {
            if ip < 0 || ip >= instructions.len() as isize {
                break;
            }

            match instructions[ip as usize] {
                Set(Register(r), v) => registers[r] = v.evaluate(&registers),
                Sub(Register(r), v) => registers[r] -= v.evaluate(&registers),
                Mul(Register(r), v) => {
                    registers[r] *= v.evaluate(&registers);
                    muls += 1;
                }
                Jnz(r, v) => {
                    if r.evaluate(&registers) != 0 {
                        ip += v.evaluate(&registers);
                        continue;
                    }
                }
                _ => unreachable!(),
            }

            ip += 1;
        }

        Option::from(muls.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut instructions = parse_instructions(input);

        let mut registers = vec![0isize; 8];
        registers[0] = 1;
        let mut ip: isize = 0;

        loop {
            // first 8 instructions are the constants
            if ip > 8 {
                break;
            }

            match instructions[ip as usize] {
                Set(Register(r), v) => registers[r] = v.evaluate(&registers),
                Sub(Register(r), v) => registers[r] -= v.evaluate(&registers),
                Mul(Register(r), v) => registers[r] *= v.evaluate(&registers),
                Jnz(r, v) => {
                    if r.evaluate(&registers) != 0 {
                        ip += v.evaluate(&registers);
                        continue;
                    }
                }
                _ => unreachable!(),
            }

            ip += 1;
        }

        // the program just counts composite numbers between b and c, incrementing by 17
        // (maybe this is different for other inputs, in which case parse from file...)
        let mut b = registers[1];
        let c = registers[2];

        let mut composites = 0;
        while b <= c {
            if is_composite(b) {
                composites += 1;
            }

            b += 17;
        }

        Option::from(composites.to_string())
    }
}
