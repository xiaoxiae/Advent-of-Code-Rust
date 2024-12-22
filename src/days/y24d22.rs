use crate::util::Day;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelString;
use rustc_hash::FxHashMap as HashMap;

pub struct Y24D22;

static MOD: usize = 16_777_216;
static STEPS: usize = 2_000;

fn evolve(number: usize) -> usize {
    let number = (number ^ (number * 64)) % MOD;
    let number = (number ^ (number / 32)) % MOD;
    let number = (number ^ (number * 2048)) % MOD;

    number
}

impl Day for Y24D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let sum: usize = input
            .trim()
            .par_split_whitespace()
            .into_par_iter()
            .map(|n| {
                let mut number = n.parse().unwrap();

                for _ in 0..STEPS {
                    number = evolve(number);
                }

                number
            })
            .sum();

        Option::from(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let combined_maps: HashMap<u32, usize> = input
            .trim()
            .par_split_whitespace()
            .fold(HashMap::default, |mut acc: HashMap<u32, usize>, n| {
                let mut number: usize = n.parse().unwrap();
                let mut previous_number = number;

                // we are hashing each sequence [a, b, c, d] as an u32 number in the following way:
                //
                // 87654321 87654321 87654321 87654321
                //     a+10     b+10     c+10     d+10
                let mut key: u32 = 0;
                let mut map: HashMap<u32, u8> = HashMap::default();

                for _ in 0..STEPS {
                    number = evolve(number);

                    let value = (number % 10) as u8;
                    let delta = ((number % 10) as isize - (previous_number % 10) as isize);

                    key = (key << 8) + (delta + 10) as u32;

                    // if the last byte has something, we added at least 4 numbers
                    if key & (0b11111111 << 24) != 0 {
                        map.entry(key).or_insert(value);
                    }

                    previous_number = number;
                }

                // only remember the first key occurrence, since that's where the monkey stops
                for (key, value) in map {
                    *acc.entry(key).or_insert(0) += value as usize;
                }

                acc
            })
            .reduce(HashMap::default, |mut acc1, acc2| {
                for (key, value) in acc2 {
                    *acc1.entry(key).or_insert(0) += value;
                }

                acc1
            });

        let bananas = combined_maps.values().max();
        match bananas {
            Some(v) => Some(v.to_string()),
            None => panic!("Something went terribly wrong!"),
        }
    }
}
