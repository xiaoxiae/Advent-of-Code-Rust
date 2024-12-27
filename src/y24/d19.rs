use crate::util::Day;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (stripes, patterns) = input.split_once("\n\n").unwrap();

    (
        stripes.split(", ").collect::<Vec<_>>(),
        patterns.split_whitespace().collect::<Vec<_>>(),
    )
}

fn count_recursive(current: &str, trie: &Trie, cache: &mut Vec<usize>) -> usize {
    if current.len() == 0 {
        return 1;
    }

    if cache[current.len()] != usize::MAX {
        return cache[current.len()];
    }

    let mut total = 0;
    for p in trie.matches(current) {
        if current.starts_with(p) {
            total += count_recursive(&current[p.len()..], trie, cache);
        }
    }

    cache[current.len()] = total;

    total
}


#[derive(Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Trie {
    // Create a new empty Trie
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    fn from(words: Vec<&str>) -> Self {
        let mut trie = Self::new();

        for word in words {
            trie.insert(word);
        }

        trie
    }

    // Insert a word into the Trie
    fn insert(&mut self, word: &str) {
        let mut current_node = self;
        for c in word.chars() {
            current_node = current_node
                .children
                .entry(c)
                .or_insert_with(Trie::new);
        }
        current_node.is_end_of_word = true;
    }

    // Find all prefixes of the given string in the Trie
    fn matches<'a>(&self, input: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        let mut current_node = self;

        for (i, c) in input.chars().enumerate() {
            // If the character is not found, stop the search
            if let Some(next_node) = current_node.children.get(&c) {
                if next_node.is_end_of_word {
                    // Add a slice of the input up to this index + 1
                    results.push(&input[0..=i]);
                }
                current_node = next_node;
            } else {
                break;
            }
        }

        results
    }
}


pub struct D19;

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (stripes, patterns) = parse_input(input);

        let r = format!("^({})+$", stripes.join("|"));
        let regex = Regex::new(r.as_str()).unwrap();

        let count = patterns
            .into_iter()
            .filter(|l| regex.is_match(l))
            .count();

        Option::from(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (stripes, patterns) = parse_input(input);

        let trie = Trie::from(stripes);

        let count: usize = patterns
            .par_iter()
            .map(|&l| {
                let mut cache = vec![usize::MAX; l.len() + 1];

                count_recursive(l, &trie, &mut cache)
            })
            .sum();

        Option::from(count.to_string())
    }
}
