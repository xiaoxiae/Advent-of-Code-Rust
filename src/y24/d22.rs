use crate::util::Day;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::{ParallelSlice, ParallelString};
use rustc_hash::{FxBuildHasher, FxHashMap as HashMap};
use std::simd::num::SimdUint;
use std::simd::Simd;
use itertools::Itertools;

pub struct D22;

const MOD: usize = 16_777_216;
const STEPS: usize = 2_000;

const SIMD_LANE_SIZE: usize = 16;

fn evolve(mut number: usize) -> usize {
    number = (number ^ (number * 64)) % MOD;
    number = (number ^ (number / 32)) % MOD;

    (number ^ (number * 2048)) % MOD
}

fn evolve_simd(numbers: &mut Simd<usize, SIMD_LANE_SIZE>) {
    *numbers ^= (*numbers * Simd::splat(64)) % Simd::splat(MOD);
    *numbers ^= (*numbers / Simd::splat(32)) % Simd::splat(MOD);
    *numbers ^= (*numbers * Simd::splat(2048)) % Simd::splat(MOD);
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let numbers: Vec<_> = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let mut simd_sum = Simd::<usize, SIMD_LANE_SIZE>::default();

        for chunk in numbers.chunks(SIMD_LANE_SIZE) {
            let mut simd_numbers: Simd<usize, SIMD_LANE_SIZE> = Simd::load_or_default(chunk);

            for _ in 0..STEPS {
                evolve_simd(&mut simd_numbers);
            }

            simd_sum += simd_numbers;
        }

        Option::from(simd_sum.reduce_sum().to_string())
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
                let mut map: HashMap<u32, u8> = HashMap::with_capacity_and_hasher(2_000, FxBuildHasher::default());
                
                for _ in 0..STEPS {
                    number = evolve(number);

                    let value = (number % 10) as u8;
                    let delta = (number % 10) as isize - (previous_number % 10) as isize;

                    key = (key << 8) + (delta + 10) as u32;

                    // we should be checking if we added at least 4 numbers (i.e. if the last bytes is non-zero),
                    // but this won't be max anyway so it doesn't really matter
                    map.entry(key).or_insert(value);

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
