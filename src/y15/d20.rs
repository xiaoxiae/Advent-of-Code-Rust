use crate::util::Day;
use rayon::iter::*;

pub struct D20;

fn gifts(mut number: usize, gifts: usize, reach: Option<usize>) -> usize {
    let original_number = number;

    let mut sum = 1;

    for p in 2..(number.isqrt() + 1) {
        let mut power = 0;

        while number % p == 0 {
            power += 1;
            number /= p;
        }

        let mut n = 1;

        for i in (1..=power).rev() {
            match reach {
                Some(v) if (original_number / p.pow(i)) > v => break,
                _ => n += p.pow(i),
            }
        }

        sum *= n
    }

    if number != 1 {
        let p = 1 + number;

        match reach {
            Some(v) if (original_number / number) > v => {}
            _ => sum *= p,
        }
    }

    sum * gifts
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let limit = input.trim().parse::<usize>().unwrap();

        let elf = (1usize..limit)
            .into_par_iter()
            .find_first(|&elf| gifts(elf, 10, None) >= limit)
            .unwrap();

        Option::from(elf.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let limit = input.trim().parse::<usize>().unwrap();
        let mut houses = vec![0; limit];

        for elf in 1..limit {
            let mut i = elf;
            while i < limit.min(elf * 50) {
                houses[i] += 11 * elf;
                i += elf;
            }

            if houses[elf] >= limit {
                return Option::from(elf.to_string());
            }
        }

        panic!("No elfs found!")
    }
}
