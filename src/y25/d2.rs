use crate::util::Day;
use itertools::Itertools;
use std::collections::HashMap;

pub struct D2;

fn is_invalid(s: usize) -> bool {
    let d = s.ilog10();
    let ten = 10_u32.pow((d + 1) / 2) as usize;

    return s % ten == s / ten;
}

fn is_really_invalid(mut s: usize) -> bool {
    let d = s.ilog10();

    for i in 1..=((d + 1) / 2) {
        let ten = 10_u32.pow(i) as usize;
        let v = s % ten;
        let mut ns = s;

        while ns > 0 {
            if ns % ten != v {
                break;
            }

            ns /= ten;
        }

        // the divisibility check is important; prevents stuff like 70707
        if ns == 0 && (d + 1) % i == 0 {
            return true;
        }
    }

    false
}

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut invalid_id_sum = 0;

        for part in input.trim().split(",") {
            let (a, b) = part.split_once('-').unwrap();

            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            for i in a..=b {
                if is_invalid(i) {
                    invalid_id_sum += i;
                }
            }
        }

        Some(invalid_id_sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut invalid_id_sum = 0;

        for part in input.trim().split(",") {
            let (a, b) = part.split_once('-').unwrap();

            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            for i in a..=b {
                if is_really_invalid(i) {
                    invalid_id_sum += i;
                }
            }
        }

        Some(invalid_id_sum.to_string())
    }
}
