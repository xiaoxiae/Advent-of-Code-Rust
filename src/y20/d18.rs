//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/18
use crate::util::Day;

pub struct D18;

fn tokenize(line: &str) -> Vec<String> {
    line.replace(')', " ) ")
        .replace('(', " ( ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Evaluate an expression with the given operator precedences.
/// `plus_prec` and `times_prec` map "+" and "*" to their precedence values.
fn evaluate(expression: &[String], plus_prec: i64, times_prec: i64) -> i64 {
    let prec = |op: &str| -> i64 {
        match op {
            "+" => plus_prec,
            "*" => times_prec,
            _ => -1, // not an operator (e.g. "(")
        }
    };
    let is_operator = |op: &str| op == "+" || op == "*";
    let apply = |op: &str, a: i64, b: i64| -> i64 {
        match op {
            "+" => a + b,
            "*" => a * b,
            _ => unreachable!(),
        }
    };

    let mut number_stack: Vec<i64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    for term in expression {
        if term.chars().all(|c| c.is_ascii_digit()) && !term.is_empty() {
            number_stack.push(term.parse::<i64>().unwrap());
        } else if term == "(" {
            operator_stack.push("(".to_string());
        } else if term == ")" {
            while operator_stack.last().is_some_and(|o| is_operator(o)) {
                let op = operator_stack.pop().unwrap();
                let b = number_stack.pop().unwrap();
                let a = number_stack.pop().unwrap();
                number_stack.push(apply(&op, a, b));
            }
            operator_stack.pop();
        } else if is_operator(term) {
            let new_prec = prec(term);

            while !operator_stack.is_empty() && is_operator(operator_stack.last().unwrap()) {
                let stack_prec = prec(operator_stack.last().unwrap());
                if stack_prec >= new_prec {
                    let op = operator_stack.pop().unwrap();
                    let b = number_stack.pop().unwrap();
                    let a = number_stack.pop().unwrap();
                    number_stack.push(apply(&op, a, b));
                } else {
                    break;
                }
            }

            operator_stack.push(term.clone());
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        let b = number_stack.pop().unwrap();
        let a = number_stack.pop().unwrap();
        number_stack.push(apply(&op, a, b));
    }

    number_stack[0]
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // Part 1: "+" and "*" have equal precedence (both 1).
        let sum: i64 = input
            .trim()
            .lines()
            .map(|line| evaluate(&tokenize(line), 1, 1))
            .sum();
        Some(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        // Part 2: "+" (2) has higher precedence than "*" (1).
        let sum: i64 = input
            .trim()
            .lines()
            .map(|line| evaluate(&tokenize(line), 2, 1))
            .sum();
        Some(sum.to_string())
    }
}
