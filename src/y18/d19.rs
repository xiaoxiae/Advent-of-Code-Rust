use crate::util::Day;

pub struct D19;

#[derive(Debug)]
enum Instruction {
    AddR(usize, usize, usize),
    AddI(usize, usize, usize),
    MulR(usize, usize, usize),
    MulI(usize, usize, usize),
    BanR(usize, usize, usize),
    BanI(usize, usize, usize),
    BorR(usize, usize, usize),
    BorI(usize, usize, usize),
    SetR(usize, usize),
    SetI(usize, usize),
    GtIR(usize, usize, usize),
    GtII(usize, usize, usize),
    GtRR(usize, usize, usize),
    EqIR(usize, usize, usize),
    EqII(usize, usize, usize),
    EqRR(usize, usize, usize),
}

fn parse(line: &str) -> (usize, Vec<Instruction>) {
    let mut lines = line.lines();

    let ip_register = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut instructions = vec![];

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let a = parts[1].parse::<usize>().unwrap();
        let b = parts[2].parse::<usize>().unwrap();
        let c = parts[3].parse::<usize>().unwrap();

        let instruction = match parts[0].as_ref() {
            "addr" => Instruction::AddR(a, b, c),
            "addi" => Instruction::AddI(a, b, c),
            "mulr" => Instruction::MulR(a, b, c),
            "muli" => Instruction::MulI(a, b, c),
            "banr" => Instruction::BanR(a, b, c),
            "bani" => Instruction::BanI(a, b, c),
            "borr" => Instruction::BorR(a, b, c),
            "bori" => Instruction::BorI(a, b, c),
            "setr" => Instruction::SetR(a, c),
            "seti" => Instruction::SetI(a, c),
            "gtir" => Instruction::GtIR(a, b, c),
            "gtii" => Instruction::GtII(a, b, c),
            "gtrr" => Instruction::GtRR(a, b, c),
            "eqir" => Instruction::EqIR(a, b, c),
            "eqii" => Instruction::EqII(a, b, c),
            "eqrr" => Instruction::EqRR(a, b, c),
            _ => unreachable!(),
        };

        instructions.push(instruction);
    }

    (ip_register, instructions)
}

// // decompiled version; calculates the sum of devisors after initializing with random values
// // is super slow since it's a double for-loop over gigantic numbers
// fn fast() -> usize {
//     let (mut a, mut b, mut c, mut d, mut e, mut ip) = (1, 0, 0, 0, 0, 0);
//
//     d += 2;
//     d *= d;
//     d *= 19;
//     d *= 11;
//
//     b += 4;
//     b *= 22;
//     b += 2;
//
//     d += b;
//
//     // these are the big boi constants setting register 0 adds
//     // b = 28;
//     // b *= 29;
//     // b += 30;
//     // b *= 31;
//     // b *= 14;
//     // b *= 33;
//     // d += b;
//
//     a = 0;
//     c = 1;
//
//     loop {
//         e = 1;
//
//         loop {
//             if c * e == d {
//                 a += c;
//             }
//
//             e += 1;
//
//             if e <= d {
//                 continue;
//             }
//
//             c += 1;
//             break;
//         }
//
//         if c > d {
//             return a;
//         }
//     }
// }

fn solve(input: &str, r: &mut Vec<usize>, break_on: Option<usize>) {
    let (ip_register, instructions) = parse(input);
    let mut ip = 0;

    while ip < instructions.len() {
        r[ip_register] = ip;

        if let Some(bip) = break_on {
            if ip == bip {
                return;
            }
        }

        let instruction = &instructions[ip];

        match instruction {
            Instruction::AddR(a, b, c) => r[*c] = r[*a] + r[*b],
            Instruction::AddI(a, b, c) => r[*c] = r[*a] + *b,
            Instruction::MulR(a, b, c) => r[*c] = r[*a] * r[*b],
            Instruction::MulI(a, b, c) => r[*c] = r[*a] * *b,
            Instruction::BanR(a, b, c) => r[*c] = r[*a] & r[*b],
            Instruction::BanI(a, b, c) => r[*c] = r[*a] & *b,
            Instruction::BorR(a, b, c) => r[*c] = r[*a] | r[*b],
            Instruction::BorI(a, b, c) => r[*c] = r[*a] | *b,
            Instruction::SetR(a, c) => r[*c] = r[*a],
            Instruction::SetI(a, c) => r[*c] = *a,
            Instruction::GtIR(a, b, c) => r[*c] = (*a > r[*b]) as usize,
            Instruction::GtII(a, b, c) => r[*c] = (r[*a] > *b) as usize,
            Instruction::GtRR(a, b, c) => r[*c] = (r[*a] > r[*b]) as usize,
            Instruction::EqIR(a, b, c) => r[*c] = (*a == r[*b]) as usize,
            Instruction::EqII(a, b, c) => r[*c] = (r[*a] == *b) as usize,
            Instruction::EqRR(a, b, c) => r[*c] = (r[*a] == r[*b]) as usize,
        }

        ip = r[ip_register] + 1;
    }
}

fn sum_of_divisors(mut n: usize) -> usize {
    let mut sum = 1;
    let mut p = 2;

    while p * p <= n {
        let mut power_sum = 1;
        let mut term = 1;

        while n % p == 0 {
            n /= p;
            term *= p;
            power_sum += term;
        }

        sum *= power_sum;
        p += 1;
    }

    if n > 1 {
        sum *= (1 + n);
    }

    sum
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut r = vec![0, 0, 0, 0, 0, 0];

        solve(input, &mut r, None);

        Option::from(r[0].to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut r = vec![1, 0, 0, 0, 0, 0];

        solve(input, &mut r, Some(1));
        
        Option::from(sum_of_divisors(r[4]).to_string())
    }
}
