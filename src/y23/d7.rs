//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/07
use crate::util::Day;

pub struct D7;

fn occurrences(hand: &[char]) -> Vec<usize> {
    let mut sorted: Vec<char> = hand.to_vec();
    sorted.sort();
    let mut occ: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < sorted.len() {
        let mut j = i;
        while j < sorted.len() && sorted[j] == sorted[i] {
            j += 1;
        }
        occ.push(j - i);
        i = j;
    }
    occ.sort_unstable_by(|a, b| b.cmp(a));
    occ
}

fn kind_from_occ(occ: &[usize]) -> i32 {
    if occ[0] == 5 {
        6
    } else if occ[0] == 4 {
        5
    } else if occ[0] == 3 && occ.len() > 1 && occ[1] == 2 {
        4
    } else if occ[0] == 3 {
        3
    } else if occ[0] == 2 && occ.len() > 1 && occ[1] == 2 {
        2
    } else if occ[0] == 2 {
        1
    } else {
        0
    }
}

fn kind_part1(hand: &[char]) -> i32 {
    kind_from_occ(&occurrences(hand))
}

fn kind_part2(hand: &[char]) -> i32 {
    let jokers = hand.iter().filter(|&&c| c == 'J').count();

    if jokers == 0 {
        return kind_from_occ(&occurrences(hand));
    }

    // The optimal substitution is to turn every joker into the most common
    // non-joker card (or, for the all-joker hand, into a single value).
    let non_jokers: Vec<char> = hand.iter().copied().filter(|&c| c != 'J').collect();

    if non_jokers.is_empty() {
        // All jokers -> five of a kind.
        return 6;
    }

    let mut best_occ = occurrences(&non_jokers);
    // Add the jokers to the largest group.
    best_occ[0] += jokers;
    kind_from_occ(&best_occ)
}

fn solve(input: &str, strength: &str, joker: bool) -> i64 {
    let strength_chars: Vec<char> = strength.chars().collect();
    let str_index = |c: char| -> usize { strength_chars.iter().position(|&x| x == c).unwrap() };

    let mut hands: Vec<(Vec<char>, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let hand: Vec<char> = parts.next().unwrap().chars().collect();
        let bid: i64 = parts.next().unwrap().parse().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| {
        let ka = if joker { kind_part2(&a.0) } else { kind_part1(&a.0) };
        let kb = if joker { kind_part2(&b.0) } else { kind_part1(&b.0) };
        if ka != kb {
            return ka.cmp(&kb);
        }
        let ia: Vec<usize> = a.0.iter().map(|&c| str_index(c)).collect();
        let ib: Vec<usize> = b.0.iter().map(|&c| str_index(c)).collect();
        ia.cmp(&ib)
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as i64 + 1) * bid)
        .sum()
}

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(solve(input, "23456789TJQKA", false).to_string())
    }
    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(solve(input, "J23456789TQKA", true).to_string())
    }
}
