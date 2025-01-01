use crate::util::Day;
use rustc_hash::FxHashSet as HashSet;

pub struct D6;

fn solve(packet: &str, width: usize) -> Option<String> {
    for i in 0..=(packet.len() - width) {
        let window: HashSet<char> = packet[i..i + width].chars().collect();
        if window.len() == width {
            return Option::from((i + width).to_string());
        }
    }

    None
}

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        solve(input, 4)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        solve(input, 14)
    }
}
