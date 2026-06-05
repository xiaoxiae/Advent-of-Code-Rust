//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/22
use crate::util::Day;
use std::collections::VecDeque;
use rustc_hash::FxHashSet;

pub struct D22;

fn parse(input: &str) -> Vec<VecDeque<i64>> {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .skip(1)
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<VecDeque<i64>>()
        })
        .collect()
}

fn score(deck: &VecDeque<i64>) -> i64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i + 1) as i64)
        .sum()
}

/// Returns (winner, (h1, h2)) where winner is 0 or 1.
fn recursive_combat(
    mut h1: VecDeque<i64>,
    mut h2: VecDeque<i64>,
) -> (usize, (VecDeque<i64>, VecDeque<i64>)) {
    let mut seen: FxHashSet<(Vec<i64>, Vec<i64>)> = FxHashSet::default();
    seen.insert((
        h1.iter().copied().collect(),
        h2.iter().copied().collect(),
    ));

    while !h1.is_empty() && !h2.is_empty() {
        let c1 = h1.pop_front().unwrap();
        let c2 = h2.pop_front().unwrap();

        let winner: usize = if c1 as usize <= h1.len() && c2 as usize <= h2.len() {
            let sub1: VecDeque<i64> = h1.iter().take(c1 as usize).copied().collect();
            let sub2: VecDeque<i64> = h2.iter().take(c2 as usize).copied().collect();
            recursive_combat(sub1, sub2).0
        } else if c1 > c2 {
            0
        } else {
            1
        };

        if winner == 0 {
            h1.push_back(c1);
            h1.push_back(c2);
        } else {
            h2.push_back(c2);
            h2.push_back(c1);
        }

        let h = (
            h1.iter().copied().collect::<Vec<i64>>(),
            h2.iter().copied().collect::<Vec<i64>>(),
        );

        if seen.contains(&h) {
            return (0, (h1, h2));
        }

        seen.insert(h);
    }

    let winner = if !h1.is_empty() { 0 } else { 1 };
    (winner, (h1, h2))
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut players = parse(input);
        let mut p1 = players.remove(0);
        let mut p0 = players.remove(0);
        // p0 is players[0], p1 is players[1]
        std::mem::swap(&mut p0, &mut p1);

        while !p0.is_empty() && !p1.is_empty() {
            let c1 = p0.pop_front().unwrap();
            let c2 = p1.pop_front().unwrap();

            if c1 > c2 {
                p0.push_back(c1);
                p0.push_back(c2);
            } else {
                p1.push_back(c2);
                p1.push_back(c1);
            }
        }

        let deck = if !p0.is_empty() { &p0 } else { &p1 };
        Some(score(deck).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let players = parse(input);
        let h1 = players[0].clone();
        let h2 = players[1].clone();

        let (_, (p0, p1)) = recursive_combat(h1, h2);
        let deck = if !p0.is_empty() { &p0 } else { &p1 };
        Some(score(deck).to_string())
    }
}
