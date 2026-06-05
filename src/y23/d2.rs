//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/02
use crate::util::Day;

pub struct D2;

fn parse_color(possible: &[i64; 3], color: &str) -> i64 {
    match color {
        "red" => possible[0],
        "green" => possible[1],
        "blue" => possible[2],
        _ => 0,
    }
}

fn is_possible(line: &str) -> bool {
    let possible: [i64; 3] = [12, 13, 14]; // red, green, blue

    let rest = match line.splitn(2, ": ").nth(1) {
        Some(r) => r,
        None => return true,
    };

    for part in rest.split("; ") {
        for instance in part.split(", ") {
            let mut it = instance.split_whitespace();
            let count: i64 = match it.next() {
                Some(c) => c.parse().unwrap_or(0),
                None => continue,
            };
            let color = match it.next() {
                Some(c) => c,
                None => continue,
            };

            if parse_color(&possible, color) < count {
                return false;
            }
        }
    }

    true
}

fn power(line: &str) -> i64 {
    let mut minimum: [i64; 3] = [0, 0, 0]; // red, green, blue

    let rest = match line.splitn(2, ": ").nth(1) {
        Some(r) => r,
        None => "",
    };

    for part in rest.split("; ") {
        for instance in part.split(", ") {
            let mut it = instance.split_whitespace();
            let count: i64 = match it.next() {
                Some(c) => c.parse().unwrap_or(0),
                None => continue,
            };
            let color = match it.next() {
                Some(c) => c,
                None => continue,
            };

            let idx = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => continue,
            };
            if count > minimum[idx] {
                minimum[idx] = count;
            }
        }
    }

    minimum[0] * minimum[1] * minimum[2]
}

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let possible_games: i64 = input
            .lines()
            .enumerate()
            .filter(|(_, line)| is_possible(line))
            .map(|(i, _)| i as i64 + 1)
            .sum();
        Some(possible_games.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let total_power: i64 = input.lines().map(power).sum();
        Some(total_power.to_string())
    }
}
