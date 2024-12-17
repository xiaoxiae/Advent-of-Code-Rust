use crate::util::Day;
use itertools::Itertools;
use regex::Regex;

type Registers = Vec<usize>;
type Instructions = Vec<usize>;

static A: usize = 0;
static B: usize = 1;
static C: usize = 2;


fn parse_input(input: &str) -> (Registers, Instructions) {
    let parts = input.trim().split("\n\n").collect::<Vec<_>>();

    let re = Regex::new(r"\d+").unwrap();

    let registers = re.find_iter(parts[0])
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let instructions = re.find_iter(parts[1])
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    (registers, instructions)
}

fn combo(val: usize, registers: &Registers) -> usize {
    if val <= 3 {
        val
    } else {
        assert_ne!(val, 7);

        registers[val - 4]
    }
}

fn run(registers: &mut Registers, instructions: &Instructions) -> Vec<usize> {
    let mut ip: usize = 0;
    let mut out: Vec<usize> = Vec::new();

    while ip < instructions.len() {
        let opt = instructions[ip];
        let val = instructions[ip + 1];

        match opt {
            0 => registers[A] = registers[A] / (1 << combo(val, &registers)),
            1 => registers[B] ^= val,
            2 => registers[B] = combo(val, &registers) % 8,
            3 => {
                if registers[A] != 0 {
                    ip = val;
                    continue;
                }
            }
            4 => registers[B] = registers[B] ^ registers[C],
            5 => out.push(combo(val, &registers) % 8),
            6 => registers[B] = registers[A] / (1 << combo(val, &registers)),
            7 => registers[C] = registers[A] / (1 << combo(val, &registers)),
            _ => panic!("Unknown instruction '{}'!", opt),
        }

        ip += 2;
    }

    out
}

/// When running the program with increasing As, there is a pattern:
/// - when increasing i for 2^(3*i), the i-th digit loops
/// - when looping the digit doesn't change the digits after it
///
/// Combining these two facts, we can loop digits from the end and recurse.
fn brute(value: usize, power: usize, instructions: &Instructions) -> Option<usize> {
    for i in 0..(1 << 3) {
        let new_value = value + i * (1 << (3 * power));

        let mut registers = vec![new_value, 0, 0];
        let out = run(&mut registers, &instructions);

        if out.get(power) == instructions.get(power) {
            if power == 0 {
                return Some(new_value);
            }

            if let Some(v) = brute(new_value, power - 1, instructions) {
                return Some(v);
            }
        }
    }

    None
}

pub struct Y24D17;

impl Day for Y24D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut registers, instructions) = parse_input(input);

        let out = run(&mut registers, &instructions);

        Option::from(out.iter().map(|v| v.to_string()).join(","))
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (_, instructions) = parse_input(input);

        let a = brute(0, instructions.len() - 1, &instructions)
            .expect("No A value found!");

        Option::from(a.to_string())
    }
}
