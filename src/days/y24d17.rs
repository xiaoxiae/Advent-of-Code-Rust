use crate::util::Day;
use itertools::Itertools;
use regex::Regex;
use rand::Rng;
use rayon::prelude::*;

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
fn recursive(value: usize, power: usize, instructions: &Instructions) -> Option<usize> {
    for i in 0..(1 << 3) {
        let new_value = value + i * (1 << (3 * power));

        let mut registers = vec![new_value, 0, 0];
        let out = run(&mut registers, &instructions);

        if out.get(power) == instructions.get(power) {
            if power == 0 {
                return Some(new_value);
            }

            if let Some(v) = recursive(new_value, power - 1, instructions) {
                return Some(v);
            }
        }
    }

    None
}

/// Flip a random bit
fn mutate(value: usize, bits: usize) -> usize {
    let bit_to_flip = rand::thread_rng().gen_range(0..bits);
    value ^ (1 << bit_to_flip)
}

/// Fitness of an individual is how many places it matches with instructions
fn fitness(value: usize, instructions: &Instructions) -> usize {
    let mut registers = vec![value, 0, 0];
    let out = run(&mut registers, &instructions);

    out.iter().zip(instructions.iter())
        .filter(|(&i, &j)| i == j)
        .count()
}

/// A genetic solution that randomly flips bits of the register A until we get a quine
fn genetic(instructions: &Instructions) -> Option<usize> {
    const PATIENCE: usize = 3_200;  // how many generations without improvement we wait
    const POOL_SIZE: usize = 1_024;  // generation size

    let mut pool: Vec<usize> = vec![0; POOL_SIZE];

    let mut min_a = usize::MAX;
    let mut generation = 0;
    loop {
        // calculate fitness
        let mut values: Vec<(usize, usize)> = pool.par_iter()
            .map(|v| (fitness(*v, instructions), *v))
            .collect::<Vec<_>>();

        values.sort_by_key(|v| v.0);

        for &(value, a) in &values {
            if value == instructions.len() && min_a > a {
                min_a = a;
                generation = 0;
            }
        }

        // kill half the population, mutating the remaining ones
        let mut offspring = values.iter().map(|v| v.1)
            .collect::<Vec<usize>>();

        offspring = offspring[offspring.len() / 2..].to_owned()
            .iter().flat_map(|&v| [mutate(v, instructions.len() * 3), mutate(v, instructions.len() * 3)])
            .collect::<Vec<usize>>();

        pool = offspring;

        generation += 1;

        if generation > PATIENCE {
            break;
        }
    }

    match min_a {
        usize::MAX => None,
        v => Option::from(v),
    }
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

        // I was bored, so I also solved it with a genetic algorithm
        //
        // let a = genetic(&instructions)
        //     .expect("No A value found!");

        let a = recursive(0, instructions.len() - 1, &instructions)
            .expect("No A value found!");

        Option::from(a.to_string())
    }
}
