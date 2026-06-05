//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/21
use crate::util::Day;

pub struct D21;

fn parse_positions(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|x| {
            // int(x[28:]) - 1, where x[28:] is everything from index 28 on
            let s: String = x.chars().skip(28).collect();
            s.trim().parse::<i64>().unwrap() - 1
        })
        .collect()
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut positions = parse_positions(input);
        let mut scores = [0i64, 0i64];

        let mut turn = 0usize;
        let mut i: i64 = 0;
        while scores[0] < 1000 && scores[1] < 1000 {
            for _ in 0..3 {
                positions[turn] += i + 1;
                i += 1;
            }

            positions[turn] = positions[turn].rem_euclid(10);
            scores[turn] += positions[turn] + 1;

            turn = (turn + 1) % 2;
        }

        let answer = scores.iter().min().unwrap() * i;
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // throw_sums: sum of three d3 throws -> number of ways
        let mut counts = std::collections::HashMap::new();
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    *counts.entry(a + b + c).or_insert(0i64) += 1;
                }
            }
        }
        // insertion order is irrelevant here; collect into vec
        let throw_sums: Vec<(i64, i64)> = counts.into_iter().collect();

        fn recursive_count(
            scores: &mut [i64; 2],
            positions: &mut [i64; 2],
            turn: usize,
            throw_sums: &[(i64, i64)],
            limit: i64,
        ) -> [i64; 2] {
            let mut universes = [0i64, 0i64];

            if scores[0] >= limit || scores[1] >= limit {
                universes[(turn + 1) % 2] = 1;
                return universes;
            }

            for &(s, ways) in throw_sums {
                positions[turn] = (positions[turn] + s).rem_euclid(10);
                scores[turn] += positions[turn] + 1;

                let mut universe_delta =
                    recursive_count(scores, positions, (turn + 1) % 2, throw_sums, limit);

                universe_delta[0] *= ways;
                universe_delta[1] *= ways;

                universes[0] += universe_delta[0];
                universes[1] += universe_delta[1];

                scores[turn] -= positions[turn] + 1;
                positions[turn] = (positions[turn] - s).rem_euclid(10);
            }

            universes
        }

        let parsed = parse_positions(input);
        let mut positions = [parsed[0], parsed[1]];
        let mut scores = [0i64, 0i64];

        let universes = recursive_count(&mut scores, &mut positions, 0, &throw_sums, 21);
        let answer = *universes.iter().max().unwrap();
        Some(answer.to_string())
    }
}
