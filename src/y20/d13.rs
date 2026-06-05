//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/13
use crate::util::Day;

pub struct D13;

fn lines(input: &str) -> Vec<&str> {
    input.trim_end_matches('\n').trim().lines().collect()
}

/// Extended Euclidean algorithm. Returns (gcd, x, y).
fn get_extended_gcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    let (mut x, mut y, mut u, mut v) = (0i128, 1i128, 1i128, 0i128);
    while a != 0 {
        // Python uses floor division/modulo (q, r = b // a, b % a).
        let q = floor_div(b, a);
        let r = floor_mod(b, a);
        let m = x - u * q;
        let n = y - v * q;
        b = a;
        a = r;
        x = u;
        y = v;
        u = m;
        v = n;
    }
    let gcd = b;
    (gcd, x, y)
}

fn floor_div(a: i128, b: i128) -> i128 {
    let q = a / b;
    if (a % b != 0) && ((a < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

fn floor_mod(a: i128, b: i128) -> i128 {
    let r = a % b;
    if r != 0 && ((r < 0) != (b < 0)) {
        r + b
    } else {
        r
    }
}

fn get_lcm(a: i128, b: i128) -> i128 {
    let (gcd, _, _) = get_extended_gcd(a, b);
    ((a * b) / gcd).abs()
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let ls = lines(input);
        let wait: i128 = ls[0].trim().parse().unwrap();
        let times: Vec<&str> = ls[1].trim().split(',').collect();

        let mut minimum_time: i128 = i128::MAX;
        let mut minimum_bus: i128 = 0;

        for t in &times {
            if *t == "x" {
                continue;
            }
            let time: i128 = t.parse().unwrap();
            let until_next = time - floor_mod(wait, time);
            if until_next < minimum_time {
                minimum_time = until_next;
                minimum_bus = time;
            }
        }

        Some((minimum_time * minimum_bus).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let ls = lines(input);
        let times: Vec<&str> = ls[1].trim().split(',').collect();

        let mut equations: Vec<(i128, i128)> = Vec::new();
        let base: i128 = times[0].parse().unwrap();
        for (i, &t) in times.iter().enumerate().skip(1) {
            if t == "x" {
                continue;
            }
            let time: i128 = t.parse().unwrap();

            let (_, x, _y) = get_extended_gcd(base, time);
            let lcm = get_lcm(base, time);

            equations.push((lcm, -base * x * (i as i128)));
        }

        while equations.len() != 1 {
            let e2 = equations.pop().unwrap();
            let e1 = equations.pop().unwrap();

            let mut a = e1.0;
            let mut b = -e2.0;
            let mut s = -(e1.1 - e2.1);

            let (gcd, _x, _y) = get_extended_gcd(a, b);

            a = floor_div(a, gcd);
            b = floor_div(b, gcd);
            s = floor_div(s, gcd);

            let mut j: i128 = -1;
            loop {
                if floor_mod(b * j + s, a) == 0 && floor_div(b * j + s, a) > 0 {
                    break;
                }
                j -= 1;
            }

            let i = floor_div(b * j + s, a);

            let lcm = get_lcm(a, b);

            equations.push((lcm, gcd * a * i + e1.1));
        }

        Some(equations[0].1.to_string())
    }
}
