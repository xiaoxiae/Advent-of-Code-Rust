//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/18
use crate::util::Day;

pub struct D18;

#[derive(Clone, Copy, PartialEq)]
enum Token {
    Open,
    Close,
    Num(i64),
}

fn from_string(string: &str) -> Vec<Token> {
    let mut number = Vec::new();
    let mut num = String::new();

    for ch in string.chars() {
        if ch == '[' || ch == ']' {
            if !num.is_empty() {
                number.push(Token::Num(num.parse().unwrap()));
                num.clear();
            }
            number.push(if ch == '[' { Token::Open } else { Token::Close });
        } else if ch == ',' {
            if !num.is_empty() {
                number.push(Token::Num(num.parse().unwrap()));
                num.clear();
            }
        } else {
            num.push(ch);
        }
    }

    number
}

fn explode(number: &mut Vec<Token>) -> bool {
    let mut depth = 0;

    for i in 0..number.len() {
        match number[i] {
            Token::Open => depth += 1,
            Token::Close => depth -= 1,
            _ => {}
        }

        if depth == 5 {
            // number[i] is Open; number[i+1], number[i+2] are the pair values.
            let l = if let Token::Num(v) = number.remove(i + 1) { v } else { 0 };
            let r = if let Token::Num(v) = number.remove(i + 1) { v } else { 0 };

            // pop the Open and the Close, then insert 0.
            number.remove(i);
            number.remove(i);
            number.insert(i, Token::Num(0));

            for j in (0..i).rev() {
                if let Token::Num(v) = number[j] {
                    number[j] = Token::Num(v + l);
                    break;
                }
            }

            for j in (i + 1)..number.len() {
                if let Token::Num(v) = number[j] {
                    number[j] = Token::Num(v + r);
                    break;
                }
            }

            return true;
        }
    }

    false
}

fn split(number: &mut Vec<Token>) -> bool {
    for i in 0..number.len() {
        if let Token::Num(c) = number[i] {
            if c >= 10 {
                let l = c / 2;
                let r = (c + 1) / 2;

                number.remove(i);
                number.insert(i, Token::Close);
                number.insert(i, Token::Num(r));
                number.insert(i, Token::Num(l));
                number.insert(i, Token::Open);

                return true;
            }
        }
    }

    false
}

fn reduce(number: &mut Vec<Token>) {
    loop {
        if explode(number) {
            continue;
        }
        if split(number) {
            continue;
        }
        break;
    }
}

fn add(n1: &[Token], n2: &[Token]) -> Vec<Token> {
    let mut number = Vec::with_capacity(n1.len() + n2.len() + 2);
    number.push(Token::Open);
    number.extend_from_slice(n1);
    number.extend_from_slice(n2);
    number.push(Token::Close);
    reduce(&mut number);
    number
}

fn magnitude(number: &[Token]) -> i64 {
    let mut number = number.to_vec();

    while number.len() != 1 {
        // find first i in 0..len-2 with two adjacent ints at i, i+1
        let mut found = None;
        for i in 0..number.len().saturating_sub(2) {
            if matches!(number[i], Token::Num(_)) && matches!(number[i + 1], Token::Num(_)) {
                found = Some(i);
                break;
            }
        }

        if let Some(i) = found {
            // Python: pop(i-1) removes '['; then result = pop(i-1)*3 + pop(i-1)*2
            // pattern is [ l r ] occupying i-1 .. i+2
            number.remove(i - 1);
            let l = if let Token::Num(v) = number.remove(i - 1) { v } else { 0 };
            let r = if let Token::Num(v) = number.remove(i - 1) { v } else { 0 };
            let result = l * 3 + r * 2;
            number[i - 1] = Token::Num(result);
        } else {
            break;
        }
    }

    if let Token::Num(v) = number[0] {
        v
    } else {
        0
    }
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

        let mut number = from_string(lines[0]);
        for line in &lines[1..] {
            number = add(&number, &from_string(line));
        }

        Some(magnitude(&number).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        let numbers: Vec<Vec<Token>> = lines.iter().map(|l| from_string(l)).collect();

        let mut max_magnitude = 0;

        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if i == j {
                    continue;
                }
                let number = add(&numbers[i], &numbers[j]);
                let m = magnitude(&number);
                if m > max_magnitude {
                    max_magnitude = m;
                }
            }
        }

        Some(max_magnitude.to_string())
    }
}
