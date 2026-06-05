//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/19
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D19;

fn parse_workflows(workflows_str: &str) -> FxHashMap<String, Vec<String>> {
    let mut workflows: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for line in workflows_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (label, parts) = line.split_once('{').unwrap();
        // parts ends with '}', drop it
        let parts = &parts[..parts.len() - 1];
        let rules: Vec<String> = parts.split(',').map(|s| s.to_string()).collect();
        workflows.insert(label.to_string(), rules);
    }
    workflows
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let input = input.trim();
        let (workflows_str, inputs_str) = input.split_once("\n\n").unwrap();
        let workflows = parse_workflows(workflows_str);

        let mut total: i64 = 0;
        for line in inputs_str.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // strip leading '{' and trailing '}'
            let inner = &line[1..line.len() - 1];
            let mut d: FxHashMap<String, i64> = FxHashMap::default();
            for part in inner.split(',') {
                let (a, b) = part.split_once('=').unwrap();
                d.insert(a.to_string(), b.parse::<i64>().unwrap());
            }

            let mut workflow = String::from("in");
            while workflow != "A" && workflow != "R" {
                for rule in &workflows[&workflow] {
                    if rule.contains('<') || rule.contains('>') {
                        let symbol = if rule.contains('<') { '<' } else { '>' };
                        let (a, b) = rule.split_once(symbol).unwrap();
                        let (val, c) = b.split_once(':').unwrap();
                        let val: i64 = val.parse().unwrap();
                        let dv = d[a];
                        let cond = if symbol == '<' { dv < val } else { dv > val };
                        if cond {
                            workflow = c.to_string();
                            break;
                        }
                    } else {
                        workflow = rule.to_string();
                        break;
                    }
                }
            }

            if workflow == "A" {
                total += d.values().sum::<i64>();
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let input = input.trim();
        let (workflows_str, _) = input.split_once("\n\n").unwrap();
        let workflows = parse_workflows(workflows_str);

        fn recursive(
            workflows: &FxHashMap<String, Vec<String>>,
            mut d: FxHashMap<String, (i64, i64)>,
            workflow: &str,
            total: &mut i64,
        ) {
            if workflow == "A" {
                let local_total: i64 = d.values().map(|(a, b)| b - a + 1).product();
                *total += local_total;
                return;
            } else if workflow == "R" {
                return;
            }

            for rule in &workflows[workflow] {
                if rule.contains('<') || rule.contains('>') {
                    let symbol = if rule.contains('<') { '<' } else { '>' };
                    let (a, b) = rule.split_once(symbol).unwrap();
                    let (val, dest) = b.split_once(':').unwrap();
                    let val: i64 = val.parse().unwrap();

                    let (l, h) = d[a];

                    if (symbol == '<' && l > val) || (symbol == '>' && h < val) {
                        continue;
                    }

                    let mut d_new = d.clone();
                    if symbol == '<' {
                        d_new.insert(a.to_string(), (l, val - 1));
                        recursive(workflows, d_new, dest, total);
                        d.insert(a.to_string(), (val, h));
                    } else {
                        d_new.insert(a.to_string(), (val + 1, h));
                        recursive(workflows, d_new, dest, total);
                        d.insert(a.to_string(), (l, val));
                    }
                } else {
                    recursive(workflows, d.clone(), rule, total);
                }
            }
        }

        let r = (1i64, 4000i64);
        let mut d: FxHashMap<String, (i64, i64)> = FxHashMap::default();
        d.insert("x".to_string(), r);
        d.insert("m".to_string(), r);
        d.insert("a".to_string(), r);
        d.insert("s".to_string(), r);

        let mut total: i64 = 0;
        recursive(&workflows, d, "in", &mut total);

        Some(total.to_string())
    }
}
