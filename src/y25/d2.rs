use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D2;

fn digit_count(n: usize) -> usize {
    (n.ilog10() + 1) as usize
}

///
/// Build a number from repeating a part.
///
fn build_repeated(part: usize, count: usize, d: u32) -> usize {
    let ten = 10_usize.pow(d);
    let mut result = part;
    for _ in 1..count {
        result = result * ten + part;
    }
    result
}

///
/// Given a number, return smallest build_repeated number smaller than it.
///
fn smallest(mut number: usize, parts: usize) -> usize {
    number = number.max(10);
    let part_width = (digit_count(number) / parts) as u32;
    let ten = 10_usize.pow(part_width);

    let mut min = usize::MAX;
    for _ in 0..parts {
        min = min.min(number % ten);
        number /= ten;
    }

    build_repeated(min.max(1), parts, part_width)
}

///
/// Return the next repeated number.
///
fn next(s: usize, parts: usize) -> usize {
    let part_width = (digit_count(s) / parts) as u32;
    let cp = s % 10_usize.pow(part_width) + 1;
    build_repeated(cp, parts, cp.ilog10() + 1)
}

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut sum = 0;

        for part in input.trim().split(',') {
            let (a, b) = part.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            let mut cur = smallest(a, 2);
            while cur <= b {
                if a <= cur {
                    sum += cur;
                }
                cur = next(cur, 2);
            }
        }

        Some(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut sum = 0;

        for part in input.trim().split(',') {
            let (a, b) = part.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            let mut seen = FxHashSet::default();

            for parts in 2..=digit_count(b) {
                let mut cur = smallest(a, parts);
                while cur <= b {
                    if a <= cur && seen.insert(cur) {
                        sum += cur;
                    }
                    cur = next(cur, parts);
                }
            }
        }

        Some(sum.to_string())
    }
}
