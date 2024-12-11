use rustc_hash::{FxHashMap as HashMap};
use crate::util::Day;

pub struct Day11;

/// A wrapper for caching stone results.
fn _count_stone(stone: i64, iterations: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    let key = (stone, iterations);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    } else {
        let result = count_stone(stone, iterations, cache);
        cache.insert(key, result);
        return result;
    }
}

fn count_stone(stone: i64, iterations: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if iterations == 0 {
        return 1;
    }

    match stone {
        0 => _count_stone(stone + 1, iterations - 1, cache),
        v if (v.ilog10() + 1) % 2 == 0 => {
            let digits = (v.ilog10() + 1) / 2;

            _count_stone(stone / 10_i64.pow(digits), iterations - 1, cache)
                + _count_stone(stone % 10_i64.pow(digits), iterations - 1, cache)
        }
        v => _count_stone(v * 2024, iterations - 1, cache)
    }
}

impl Day for Day11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let stones = input.trim().split_whitespace()
            .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut stone_cache: HashMap<(i64, i64), i64> = HashMap::default();

        let mut total = 0;
        for s in stones {
            total += count_stone(s, 25, &mut stone_cache);
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let stones = input.trim().split_whitespace()
            .map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut stone_cache: HashMap<(i64, i64), i64> = HashMap::default();

        let mut total = 0;
        for s in stones {
            total += count_stone(s, 75, &mut stone_cache);
        }

        Option::from(total.to_string())
    }
}
