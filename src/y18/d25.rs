//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/25
use crate::util::Day;

pub struct D25;

type Coord = [i64; 4];

fn distance(c1: &Coord, c2: &Coord) -> i64 {
    c1.iter().zip(c2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn in_constellation(constellation: &[Coord], c1: &Coord) -> bool {
    constellation.iter().any(|c2| distance(c1, c2) <= 3)
}

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // faithful translation of 25-1.py
        let coordinates: Vec<Coord> = input
            .split_whitespace()
            .map(|x| {
                let mut it = x.split(',').map(|n| n.parse::<i64>().unwrap());
                [
                    it.next().unwrap(),
                    it.next().unwrap(),
                    it.next().unwrap(),
                    it.next().unwrap(),
                ]
            })
            .collect();

        // Mirrors the original quirk: the first constellation is seeded with
        // coordinates[1] before the main loop runs.
        let mut constellations: Vec<Vec<Coord>> = vec![vec![coordinates[1]]];

        for &coordinate in &coordinates {
            let mut shared_constellation: Vec<Coord> = Vec::new();

            let mut i = 0;
            while i < constellations.len() {
                if in_constellation(&constellations[i], &coordinate) {
                    let popped = constellations.remove(i);
                    shared_constellation.extend(popped);
                } else {
                    i += 1;
                }
            }

            shared_constellation.push(coordinate);
            constellations.push(shared_constellation);
        }

        Some(constellations.len().to_string())
    }
    // Day 25 has no real part 2 — leave solve_part2 defaulted (the harness handles it).
}
