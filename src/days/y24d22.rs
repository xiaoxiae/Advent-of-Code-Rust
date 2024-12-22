use crate::util::Day;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelString;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct Y24D22;

static MOD: usize = 16_777_216;
static STEPS: usize = 2_000;
static SEQUENCE_SIZE: usize = 4;

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
        let price_maps: Vec<_> = input
            .trim()
            .par_split_whitespace()
            .into_par_iter()
            .map(|n| {
                let mut number: usize = n.parse().unwrap();
                let mut numbers: Vec<u8> = Vec::with_capacity(STEPS + 1);
                numbers.push((number % 10) as u8);

                for _ in 0..STEPS {
                    number = evolve(number);
                    numbers.push((number % 10) as u8);
                }

                let deltas: Vec<_> = numbers
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| *a as i8 - *b as i8)
                    .collect();

                let mut map: HashMap<Vec<i8>, u8> = HashMap::default();

                for (i, window) in deltas.windows(SEQUENCE_SIZE).enumerate() {
                    let value = numbers[i + SEQUENCE_SIZE];
                    map.entry(window.to_vec()).or_insert(value);
                }

                map
            })
            .collect();

        // only iterate over change sequences we've seen
        let mut changes: HashSet<_> = HashSet::default();
        for dict in &price_maps {
            changes.extend(dict.keys().cloned());
        }

        let bananas = changes
            .into_par_iter()
            .map(|k| {
                let mut total: usize = 0;

                for map in &price_maps {
                    total += *map.get(&k).unwrap_or(&0) as usize;
                }

                total
            })
            .max();

        match bananas {
            Some(v) => Some(v.to_string()),
            None => panic!("Something went terribly wrong!"),
        }
    }
}
