use crate::util::Day;
use rayon::iter::*;
use rayon::prelude::*;

pub struct Y15D20;

fn gifts(mut number: usize, gifts: usize, reach: Option<usize>) -> usize {
    let original_number = number;

    let mut sum = 1;

    // calculate via (1 + p1^1 + p1^2 + ...) * (1 + p2^1 + p2^2 + ...) * ...
    // TODO: problem here is that we need to test all combinations and check for the houses after that,
    //  this equation sadly doesn't work anymore

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

impl Day for Y15D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let limit = input.parse::<usize>().unwrap();

        let elf = (1usize..limit)
            .into_par_iter()
            .find_first(|&elf| gifts(elf, 10, None) >= limit)
            .unwrap();

        Option::from(elf.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let limit = input.parse::<usize>().unwrap();

        let elf = (1usize..limit)
            .into_par_iter()
            .find_first(|&elf| gifts(elf, 11, Option::from(50)) >= limit)
            .unwrap();

        Option::from(elf.to_string())
    }
}
