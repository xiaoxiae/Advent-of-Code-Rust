use std::ptr::replace;
use rayon::join;
use crate::util::Day;

pub struct D15;

// https://en.wikipedia.org/wiki/Lehmer_random_number_generator#Sample_C99_code
fn lcg_parkmiller(state: &mut usize, multiplier: usize) {
    let product = (*state) * (multiplier);
    let x = (product & 0x7fffffff) + (product >> 31);
    *state = (x & 0x7fffffff) + (x >> 31);
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut lines = input.lines();

        // the unholy triple-unwrap in a single line; they said it couldn't be done
        let mut a = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let mut b = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

        let mut count = 0;

        for _ in 0..40_000_000 {
            lcg_parkmiller(&mut a, 16807);
            lcg_parkmiller(&mut b, 48271);

            if (a & 0xFFFF) == (b & 0xFFFF) {
                count += 1;
            }
        }

        Option::from(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut lines = input.lines();

        // the unholy triple-unwrap in a single line; they said it couldn't be done
        let mut a = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let mut b = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

        let mut count = 0;

        for _ in 0..5_000_000 {
            loop {
                lcg_parkmiller(&mut a, 16807);

                if a % 4 == 0 {
                    break;
                }
            }

            loop {
                lcg_parkmiller(&mut b, 48271);

                if b % 8 == 0 {
                    break;
                }
            }

            if (a & 0xFFFF) == (b & 0xFFFF) {
                count += 1;
            }
        }

        Option::from(count.to_string())
    }
}
