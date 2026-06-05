//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/12
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D12;

fn parse(input: &str) -> Vec<[i64; 3]> {
    input
        .trim()
        .lines()
        .map(|line| {
            let cleaned: String = line
                .chars()
                .filter(|c| !matches!(c, '<' | '>' | '=' | 'x' | 'y' | 'z' | ','))
                .collect();
            let mut it = cleaned.split_whitespace().map(|s| s.parse::<i64>().unwrap());
            [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
        })
        .collect()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let pos0 = parse(input);
        let n = pos0.len();
        let mut pos = pos0;
        let mut vel = vec![[0i64; 3]; n];

        for _ in 0..10000 {
            // gravitate
            for j in 0..n {
                for k in (j + 1)..n {
                    for d in 0..3 {
                        let force = (pos[k][d] - pos[j][d]).signum();
                        vel[j][d] += force;
                        vel[k][d] -= force;
                    }
                }
            }
            // move
            for i in 0..n {
                for d in 0..3 {
                    pos[i][d] += vel[i][d];
                }
            }
        }

        // energy = product over moons of sum(abs(vel))
        let prod: i64 = vel
            .iter()
            .map(|moon| moon.iter().map(|v| v.abs()).sum::<i64>())
            .product();

        Some(prod.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let pos0 = parse(input);
        let n = pos0.len();
        let mut pos = pos0;
        let mut vel = vec![[0i64; 3]; n];

        let mut position_history: [FxHashMap<Vec<(i64, i64)>, i64>; 3] =
            [FxHashMap::default(), FxHashMap::default(), FxHashMap::default()];
        let mut steps: [i64; 3] = [-1, -1, -1];

        let mut i: i64 = 0;
        while steps.contains(&-1) {
            for j in 0..3 {
                if steps[j] == -1 {
                    let p: Vec<(i64, i64)> = (0..n).map(|m| (pos[m][j], vel[m][j])).collect();
                    if position_history[j].contains_key(&p) {
                        steps[j] = i;
                    } else {
                        position_history[j].insert(p, i);
                    }
                }
            }

            // gravitate
            for a in 0..n {
                for b in (a + 1)..n {
                    for d in 0..3 {
                        let force = (pos[b][d] - pos[a][d]).signum();
                        vel[a][d] += force;
                        vel[b][d] -= force;
                    }
                }
            }
            // move
            for m in 0..n {
                for d in 0..3 {
                    pos[m][d] += vel[m][d];
                }
            }

            i += 1;
        }

        let tmp = (steps[0] * steps[1]) / gcd(steps[0], steps[1]);
        let lcm = (tmp * steps[2]) / gcd(tmp, steps[2]);

        Some(lcm.to_string())
    }
}
