use crate::util::Day;
use rustc_hash::FxHashMap as HashMap;

pub struct Y24D11;

/// A wrapper for caching stone results.
fn _count_stone(stone: i64, iterations: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(&result) = cache.get(&(stone, iterations)) {
        result
    } else {
        let result = count_stone(stone, iterations, cache);
        cache.insert((stone, iterations), result);
        result
    }
}

fn count_stone(stone: i64, iterations: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if iterations == 0 {
        return 1;
    }

    match stone {
        0 => _count_stone(stone + 1, iterations - 1, cache),
        v => {
            let digits = v.ilog10() + 1;

            if digits % 2 == 0 {
                let offset = 10_i64.pow(digits / 2);

                _count_stone(stone / offset, iterations - 1, cache)
                    + _count_stone(stone % offset, iterations - 1, cache)
            } else {
                _count_stone(v * 2024, iterations - 1, cache)
            }
        }
    }
}

impl Day for Y24D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let stones = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut stone_cache: HashMap<(i64, i64), i64> = HashMap::default();

        let total = stones
            .iter()
            .map(|s| count_stone(*s, 25, &mut stone_cache))
            .sum::<i64>();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let stones = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut stone_cache: HashMap<(i64, i64), i64> = HashMap::default();

        let total = stones
            .iter()
            .map(|s| count_stone(*s, 75, &mut stone_cache))
            .sum::<i64>();

        Option::from(total.to_string())
    }
}
