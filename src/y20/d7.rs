//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/07
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D7;

/// Replicates Python's `str.strip(chars)` semantics: removes any leading and
/// trailing characters that are contained in `chars`.
fn strip_chars<'a>(s: &'a str, chars: &str) -> &'a str {
    s.trim_matches(|c: char| chars.contains(c))
}

/// Parse the input into a map: bag -> {contained_bag -> count}.
fn parse(input: &str) -> FxHashMap<String, FxHashMap<String, i64>> {
    let mut bags: FxHashMap<String, FxHashMap<String, i64>> = FxHashMap::default();

    for line in input.lines() {
        let line = line.trim_end_matches(['\r']);
        if line.is_empty() {
            continue;
        }

        let mut it = line.splitn(2, " bags contain ");
        let bag = it.next().unwrap();
        let spec = it.next().unwrap();

        for other in spec.split(", ") {
            // .strip(".").strip("bag").strip("bags").strip()
            let other = strip_chars(other, ".");
            let other = strip_chars(other, "bag");
            let other = strip_chars(other, "bags");
            let other = other.trim();

            let inner = bags.entry(bag.to_string()).or_default();

            if other != "no other" {
                // count, o = other.split(" ", 1)
                let mut it = other.splitn(2, ' ');
                let count = it.next().unwrap();
                let o = it.next().unwrap();

                inner.insert(o.to_string(), count.parse::<i64>().unwrap());
            }
        }
    }

    bags
}

fn contains_target(
    bags: &FxHashMap<String, FxHashMap<String, i64>>,
    bag: &str,
    target: &str,
) -> bool {
    let inner = match bags.get(bag) {
        Some(m) => m,
        None => return false,
    };

    if inner.contains_key(target) {
        return true;
    }

    inner.keys().any(|b| contains_target(bags, b, target))
}

fn recursive_sum(bags: &FxHashMap<String, FxHashMap<String, i64>>, bag: &str) -> i64 {
    let inner = match bags.get(bag) {
        Some(m) => m,
        None => return 0,
    };

    if inner.is_empty() {
        return 0;
    }

    inner
        .iter()
        .map(|(b, i)| recursive_sum(bags, b) * i + i)
        .sum()
}

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bags = parse(input);
        let target = "shiny gold";

        let count = bags
            .keys()
            .filter(|bag| contains_target(&bags, bag, target))
            .count();

        Some(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bags = parse(input);
        Some(recursive_sum(&bags, "shiny gold").to_string())
    }
}
