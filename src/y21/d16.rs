//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/16
use crate::util::Day;

pub struct D16;

fn hex_to_bits(packet: &str) -> Vec<u8> {
    let mut bits = Vec::with_capacity(packet.len() * 4);
    for c in packet.chars() {
        let v = c.to_digit(16).unwrap() as u8;
        for i in (0..4).rev() {
            bits.push((v >> i) & 1);
        }
    }
    bits
}

fn get_int(bits: &[u8], pos: &mut usize, n: usize) -> u64 {
    let mut value: u64 = 0;
    for _ in 0..n {
        value = (value << 1) | bits[*pos] as u64;
        *pos += 1;
    }
    value
}

fn parse_literal(bits: &[u8], pos: &mut usize) -> u64 {
    let mut num: u64 = 0;
    loop {
        let prefix = bits[*pos];
        *pos += 1;
        for _ in 0..4 {
            num = (num << 1) | bits[*pos] as u64;
            *pos += 1;
        }
        if prefix == 0 {
            break;
        }
    }
    num
}

// Part 1: accumulate version sum
fn parse_packet_versions(bits: &[u8], pos: &mut usize, version_sum: &mut u64) {
    let version = get_int(bits, pos, 3);
    let type_id = get_int(bits, pos, 3);
    *version_sum += version;

    if type_id == 4 {
        let _ = parse_literal(bits, pos);
    } else {
        let length_type_id = get_int(bits, pos, 1);
        if length_type_id == 0 {
            let subpacket_bits = get_int(bits, pos, 15) as usize;
            let start = *pos;
            while *pos - start != subpacket_bits {
                parse_packet_versions(bits, pos, version_sum);
            }
        } else {
            let subpacket_count = get_int(bits, pos, 11);
            for _ in 0..subpacket_count {
                parse_packet_versions(bits, pos, version_sum);
            }
        }
    }
}

// Part 2: evaluate expression
fn parse_packet_eval(bits: &[u8], pos: &mut usize) -> u64 {
    let _version = get_int(bits, pos, 3);
    let type_id = get_int(bits, pos, 3);

    if type_id == 4 {
        return parse_literal(bits, pos);
    }

    let length_type_id = get_int(bits, pos, 1);
    let mut results: Vec<u64> = Vec::new();

    if length_type_id == 0 {
        let subpacket_bits = get_int(bits, pos, 15) as usize;
        let start = *pos;
        while *pos - start != subpacket_bits {
            results.push(parse_packet_eval(bits, pos));
        }
    } else {
        let subpacket_count = get_int(bits, pos, 11);
        for _ in 0..subpacket_count {
            results.push(parse_packet_eval(bits, pos));
        }
    }

    match type_id {
        0 => results.iter().sum(),
        1 => results.iter().product(),
        2 => *results.iter().min().unwrap(),
        3 => *results.iter().max().unwrap(),
        5 => (results[0] > results[1]) as u64,
        6 => (results[0] < results[1]) as u64,
        7 => (results[0] == results[1]) as u64,
        _ => unreachable!(),
    }
}

impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let packet = input.trim().lines().next().unwrap();
        let bits = hex_to_bits(packet);
        let mut pos = 0;
        let mut version_sum = 0;
        parse_packet_versions(&bits, &mut pos, &mut version_sum);
        Some(version_sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let packet = input.trim().lines().next().unwrap();
        let bits = hex_to_bits(packet);
        let mut pos = 0;
        let result = parse_packet_eval(&bits, &mut pos);
        Some(result.to_string())
    }
}
