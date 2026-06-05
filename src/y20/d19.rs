//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/19
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D19;

enum Rule {
    Char(String),
    Options(Vec<Vec<usize>>),
}

fn parse(input: &str) -> (FxHashMap<usize, Rule>, Vec<&str>) {
    let lines: Vec<&str> = input.trim_matches('\n').lines().collect();

    let mut rules: FxHashMap<usize, Rule> = FxHashMap::default();
    let mut i = 0;
    while i < lines.len() && !lines[i].is_empty() {
        let line = lines[i];
        let (rule, content) = line.split_once(": ").unwrap();
        let rule: usize = rule.parse().unwrap();

        if content.contains('"') {
            rules.insert(rule, Rule::Char(content.trim_matches('"').to_string()));
        } else {
            let options: Vec<Vec<usize>> = content
                .split(" | ")
                .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect();
            rules.insert(rule, Rule::Options(options));
        }

        i += 1;
    }

    let messages: Vec<&str> = lines[i + 1..].to_vec();
    (rules, messages)
}

/// Part 1: greedy match, returns the remainder of the string if it starts to
/// match a rule, else None.
fn matches_rule_greedy<'a>(
    rules: &FxHashMap<usize, Rule>,
    string: &'a str,
    rule: usize,
) -> Option<&'a str> {
    match &rules[&rule] {
        Rule::Char(c) => {
            if string.starts_with(c.as_str()) {
                Some(&string[c.len()..])
            } else {
                None
            }
        }
        Rule::Options(options) => {
            for option in options {
                let mut s = Some(string);
                for &subrule in option {
                    match s {
                        Some(cur) => s = matches_rule_greedy(rules, cur, subrule),
                        None => break,
                    }
                }
                if s.is_some() {
                    return s;
                }
            }
            None
        }
    }
}

/// Part 2: returns all possible remainders of the string after matching the
/// rule (mirrors the generator yielding the full list of possible endings).
fn matches_rule_all<'a>(
    rules: &FxHashMap<usize, Rule>,
    string: &'a str,
    rule: usize,
) -> Vec<&'a str> {
    match &rules[&rule] {
        Rule::Char(c) => {
            if string.starts_with(c.as_str()) {
                vec![&string[c.len()..]]
            } else {
                // mirrors yielding [None]; the caller filters None out
                vec![]
            }
        }
        Rule::Options(options) => {
            let mut s_total: Vec<&'a str> = Vec::new();
            for option in options {
                let mut s: Vec<&'a str> = vec![string];
                for &subrule in option {
                    s = s
                        .iter()
                        .flat_map(|st| matches_rule_all(rules, st, subrule))
                        .collect();
                }
                s_total.extend(s);
            }
            s_total
        }
    }
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (rules, messages) = parse(input);

        let total = messages
            .iter()
            .filter(|line| matches_rule_greedy(&rules, line, 0) == Some(""))
            .count();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut rules, messages) = parse(input);

        rules.insert(8, Rule::Options(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Rule::Options(vec![vec![42, 31], vec![42, 11, 31]]));

        let total = messages
            .iter()
            .filter(|line| matches_rule_all(&rules, line, 0).iter().any(|&r| r.is_empty()))
            .count();

        Some(total.to_string())
    }
}
