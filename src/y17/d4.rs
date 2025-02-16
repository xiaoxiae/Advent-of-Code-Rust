use crate::util::Day;
use std::collections::HashSet;

pub struct D4;

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total = input.lines()
            .filter(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                let unique_words: HashSet<&str> = words.iter().cloned().collect();
                words.len() == unique_words.len()
            })
            .count();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let total = input.lines()
            .filter(|line| {
                let anagram_words: Vec<String> = line.split_whitespace()
                    .map(|word| {
                        let mut chars: Vec<char> = word.chars().collect();
                        chars.sort_unstable();
                        chars.into_iter().collect::<String>()
                    })
                    .collect();
                let unique_anagrams: HashSet<String> = anagram_words.iter().cloned().collect();
                anagram_words.len() == unique_anagrams.len()
            })
            .count();

        Some(total.to_string())
    }
}
