//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/21
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D21;

enum Op {
    Num(i64),
    Expr(String, char, String),
}

fn parse(input: &str) -> FxHashMap<String, Op> {
    let mut operations: FxHashMap<String, Op> = FxHashMap::default();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].trim_end_matches(':').to_string();

        if parts.len() == 4 {
            operations.insert(
                name,
                Op::Expr(
                    parts[1].to_string(),
                    parts[2].chars().next().unwrap(),
                    parts[3].to_string(),
                ),
            );
        } else {
            operations.insert(name, Op::Num(parts[1].parse::<i64>().unwrap()));
        }
    }

    operations
}

fn get_result(operations: &FxHashMap<String, Op>, node: &str) -> i64 {
    match &operations[node] {
        Op::Num(n) => *n,
        Op::Expr(a, op, b) => {
            let av = get_result(operations, a);
            let bv = get_result(operations, b);
            match op {
                '+' => av + bv,
                '-' => av - bv,
                '*' => av * bv,
                '/' => av.div_euclid(bv),
                _ => unreachable!(),
            }
        }
    }
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let operations = parse(input);
        Some(get_result(&operations, "root").to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut operations = parse(input);

        let (root_left, root_right) = match &operations["root"] {
            Op::Expr(a, _, b) => (a.clone(), b.clone()),
            _ => unreachable!(),
        };

        let mut lo: i64 = 1;
        let mut hi: i64 = 1;

        operations.insert("humn".to_string(), Op::Num(0));
        let ineq = get_result(&operations, &root_left) < get_result(&operations, &root_right);

        loop {
            operations.insert("humn".to_string(), Op::Num(hi));

            if ineq
                != (get_result(&operations, &root_left) <= get_result(&operations, &root_right))
            {
                break;
            }

            hi *= 2;
        }

        let mut avg: i64 = lo;
        while lo < hi {
            avg = (lo + hi) / 2;

            operations.insert("humn".to_string(), Op::Num(avg));

            if ineq
                == (get_result(&operations, &root_left) <= get_result(&operations, &root_right))
            {
                lo = avg + 1;
            } else {
                hi = avg;
            }
        }

        Some(avg.to_string())
    }
}
