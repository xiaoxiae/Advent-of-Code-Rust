//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/11
use crate::util::Day;

pub struct D11;

#[derive(Clone)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    mod_val: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn from_string(string: &str) -> Monkey {
        let lines: Vec<&str> = string.lines().collect();

        // Starting items: split(maxsplit=2)[-1] then split on ","
        let items_part = lines[1].trim().splitn(3, char::is_whitespace).last().unwrap();
        let items: Vec<i64> = items_part
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        // Operation: take last two tokens (op, val)
        let toks: Vec<&str> = lines[2].split_whitespace().collect();
        let op_sym = toks[toks.len() - 2];
        let val = toks[toks.len() - 1];
        let op = if op_sym == "+" {
            Op::Add(val.parse::<i64>().unwrap())
        } else if val == "old" {
            Op::Square
        } else {
            Op::Mul(val.parse::<i64>().unwrap())
        };

        let n = lines.len();
        let mod_val = lines[n - 3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let if_true = lines[n - 2]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = lines[n - 1]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            op,
            mod_val,
            if_true,
            if_false,
        }
    }

    fn apply_op(&self, x: i64) -> i64 {
        match self.op {
            Op::Add(v) => x + v,
            Op::Mul(v) => x * v,
            Op::Square => x * x,
        }
    }

    fn throw_target(&self, x: i64) -> usize {
        if x % self.mod_val == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .replace("\r\n", "\n")
        .trim_end_matches('\n')
        .split("\n\n")
        .map(Monkey::from_string)
        .collect()
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut monkeys = parse(input);
        let mut activity = vec![0u64; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let items = std::mem::take(&mut monkeys[i].items);
                for item in items {
                    let mut item = monkeys[i].apply_op(item);
                    item /= 3;
                    let target = monkeys[i].throw_target(item);
                    activity[i] += 1;
                    monkeys[target].items.push(item);
                }
            }
        }

        activity.sort_unstable();
        let answer = activity[activity.len() - 1] * activity[activity.len() - 2];
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut monkeys = parse(input);
        let big_mod: i64 = monkeys.iter().map(|m| m.mod_val).product();
        let mut activity = vec![0u64; monkeys.len()];

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let items = std::mem::take(&mut monkeys[i].items);
                for item in items {
                    let mut item = monkeys[i].apply_op(item);
                    item %= big_mod;
                    let target = monkeys[i].throw_target(item);
                    activity[i] += 1;
                    monkeys[target].items.push(item);
                }
            }
        }

        activity.sort_unstable();
        let answer = activity[activity.len() - 1] * activity[activity.len() - 2];
        Some(answer.to_string())
    }
}
