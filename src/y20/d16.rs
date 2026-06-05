//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/16
use crate::util::Day;

pub struct D16;

/// Parse the input into (rules, my_ticket, tickets).
/// `rules` preserves insertion order (name, list of (lo, hi) ranges).
fn parse(input: &str) -> (Vec<(String, Vec<(i64, i64)>)>, Vec<i64>, Vec<Vec<i64>>) {
    let lines: Vec<&str> = input.trim_end_matches('\n').lines().collect();

    let mut rules: Vec<(String, Vec<(i64, i64)>)> = Vec::new();

    let mut i = 0;
    while !lines[i].is_empty() {
        let (rule, values) = lines[i].split_once(": ").unwrap();
        let mut ranges = Vec::new();
        for value in values.split(" or ") {
            let (lo, hi) = value.split_once('-').unwrap();
            ranges.push((lo.parse::<i64>().unwrap(), hi.parse::<i64>().unwrap()));
        }
        rules.push((rule.to_string(), ranges));
        i += 1;
    }

    let my_ticket: Vec<i64> = lines[i + 2]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let tickets: Vec<Vec<i64>> = lines[i + 5..]
        .iter()
        .map(|ticket| {
            ticket
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    (rules, my_ticket, tickets)
}

/// Whether `value` is valid for any rule (r=None) or a specific rule.
fn is_valid(rules: &[(String, Vec<(i64, i64)>)], value: i64, r: Option<usize>) -> bool {
    let in_range = |&(lo, hi): &(i64, i64)| lo <= value && value <= hi;
    match r {
        None => rules.iter().any(|(_, ranges)| ranges.iter().any(in_range)),
        Some(idx) => rules[idx].1.iter().any(in_range),
    }
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (rules, _my_ticket, tickets) = parse(input);

        let mut error: i64 = 0;
        for ticket in &tickets {
            for &value in ticket {
                if !is_valid(&rules, value, None) {
                    error += value;
                }
            }
        }

        Some(error.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (rules, my_ticket, mut tickets) = parse(input);

        // Discard invalid tickets.
        let mut i = 0;
        while i < tickets.len() {
            let mut invalid = false;
            for &value in &tickets[i] {
                if !is_valid(&rules, value, None) {
                    invalid = true;
                    break;
                }
            }
            if invalid {
                tickets.remove(i);
            } else {
                i += 1;
            }
        }

        let n = tickets[0].len();
        let mut assigned_rules: Vec<Option<usize>> = vec![None; n];

        for _ in 0..n {
            for column in 0..n {
                // Candidate rules not yet assigned anywhere.
                let mut possible: Vec<usize> = (0..rules.len())
                    .filter(|r| !assigned_rules.contains(&Some(*r)))
                    .collect();

                for row in 0..tickets.len() {
                    possible.retain(|&rule| is_valid(&rules, tickets[row][column], Some(rule)));
                }

                if possible.len() == 1 {
                    assigned_rules[column] = Some(possible[0]);
                }
            }
        }

        let mut prod: i64 = 1;
        for (i, rule) in assigned_rules.iter().enumerate() {
            if let Some(idx) = rule {
                if rules[*idx].0.starts_with("departure") {
                    prod *= my_ticket[i];
                }
            }
        }

        Some(prod.to_string())
    }
}
