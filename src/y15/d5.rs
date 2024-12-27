use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D5;

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let nice = input
            .lines()
            .map(|l| {
                let vowels = vec!['a', 'e', 'i', 'o', 'u'];

                let vowel_count = l.chars().filter(|c| vowels.contains(c)).count();
                if vowel_count < 3 {
                    return false;
                }

                let repeat_letter = l.chars().tuple_windows().any(|(a, b)| a == b);

                if !repeat_letter {
                    return false;
                }

                for bad_string in vec!["ab", "cd", "pq", "xy"] {
                    if l.contains(bad_string) {
                        return false;
                    }
                }

                true
            })
            .filter(|&b| b)
            .count();

        Option::from(nice.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let nice = input
            .lines()
            .map(|l| {
                let chars = l.chars().collect_vec();

                let in_between = chars.windows(3).any(|w| w[0] == w[2]);

                if !in_between {
                    return false;
                }

                let mut pairs: HashMap<(char, char), usize> = HashMap::new();

                for i in 0..chars.len() - 1 {
                    let k = (chars[i], chars[i + 1]);

                    if pairs.contains_key(&k) {
                        let j = *pairs.get(&k).unwrap();

                        // non-overlapping
                        if i != j + 1 {
                            return true;
                        }
                    } else {
                        pairs.insert(k, i);
                    }
                }

                false
            })
            .filter(|&b| b)
            .count();

        Option::from(nice.to_string())
    }
}
