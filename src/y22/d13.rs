//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/13
use crate::util::Day;
use std::cmp::Ordering;

pub struct D13;

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

fn parse(s: &str) -> Packet {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    parse_value(&chars, &mut pos)
}

fn parse_value(chars: &[char], pos: &mut usize) -> Packet {
    if chars[*pos] == '[' {
        *pos += 1; // consume '['
        let mut items = Vec::new();
        while chars[*pos] != ']' {
            if chars[*pos] == ',' {
                *pos += 1;
                continue;
            }
            items.push(parse_value(chars, pos));
        }
        *pos += 1; // consume ']'
        Packet::List(items)
    } else {
        // parse an integer
        let start = *pos;
        while *pos < chars.len() && chars[*pos].is_ascii_digit() {
            *pos += 1;
        }
        let num: i64 = chars[start..*pos].iter().collect::<String>().parse().unwrap();
        Packet::Int(num)
    }
}

/// Mirrors the Python `compare`: returns -1, 0, or 1.
fn compare(a: &[Packet], b: &[Packet]) -> i32 {
    if a.is_empty() && b.is_empty() {
        return 0;
    }
    if a.is_empty() {
        return -1;
    }
    if b.is_empty() {
        return 1;
    }

    let u = &a[0];
    let v = &b[0];

    let result = match (u, v) {
        (Packet::Int(x), Packet::Int(y)) => {
            if x < y {
                -1
            } else if x > y {
                1
            } else {
                0
            }
        }
        (Packet::List(x), Packet::List(y)) => compare(x, y),
        (Packet::Int(x), Packet::List(y)) => compare(&[Packet::Int(*x)], y),
        (Packet::List(x), Packet::Int(y)) => compare(x, &[Packet::Int(*y)]),
    };

    if result != 0 {
        return result;
    }

    compare(&a[1..], &b[1..])
}

/// Compare two top-level packets (which are always lists in this input).
fn compare_packets(a: &Packet, b: &Packet) -> i32 {
    match (a, b) {
        (Packet::List(x), Packet::List(y)) => compare(x, y),
        (Packet::Int(x), Packet::List(y)) => compare(&[Packet::Int(*x)], y),
        (Packet::List(x), Packet::Int(y)) => compare(x, &[Packet::Int(*y)]),
        (Packet::Int(x), Packet::Int(y)) => {
            if x < y {
                -1
            } else if x > y {
                1
            } else {
                0
            }
        }
    }
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;
        for (i, pair) in input.split("\n\n").enumerate() {
            let mut lines = pair.lines().filter(|l| !l.trim().is_empty());
            let Some(a_line) = lines.next() else {
                continue;
            };
            let Some(b_line) = lines.next() else {
                continue;
            };
            let a = parse(a_line.trim());
            let b = parse(b_line.trim());

            if compare_packets(&a, &b) == -1 {
                total += i + 1;
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut packets: Vec<Packet> = Vec::new();

        let mut blocks: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();
        blocks.push("[[2]]\n[[6]]".to_string());

        for block in &blocks {
            let mut lines = block.lines().filter(|l| !l.trim().is_empty());
            let Some(a_line) = lines.next() else {
                continue;
            };
            let Some(b_line) = lines.next() else {
                continue;
            };
            packets.push(parse(a_line.trim()));
            packets.push(parse(b_line.trim()));
        }

        // Sort using compare_packets (mirrors the Python bubble-sort ordering).
        packets.sort_by(|a, b| match compare_packets(a, b) {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal,
        });

        let div2 = parse("[[2]]");
        let div6 = parse("[[6]]");

        let pos2 = packets.iter().position(|p| *p == div2).unwrap() + 1;
        let pos6 = packets.iter().position(|p| *p == div6).unwrap() + 1;

        Some((pos2 * pos6).to_string())
    }
}
