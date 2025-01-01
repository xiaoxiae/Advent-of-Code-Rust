use crate::util::Day;

pub struct D4;

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a_s, a_e) = a.split_once('-').unwrap();
            let (b_s, b_e) = b.split_once('-').unwrap();
            (
                (a_s.parse().unwrap(), a_e.parse().unwrap()),
                (b_s.parse().unwrap(), b_e.parse().unwrap()),
            )
        })
        .collect()
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let ranges = parse(input);
        let total = ranges
            .iter()
            .filter(|&((a_s, a_e), (b_s, b_e))| {
                (a_s <= b_s && b_e <= a_e) || (b_s <= a_s && a_e <= b_e)
            })
            .count();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let ranges = parse(input);
        let total = ranges
            .iter()
            .filter(|&((a_s, a_e), (b_s, b_e))| {
                (a_s <= b_s && b_s <= a_e) || (b_s <= a_s && a_s <= b_e)
            })
            .count();

        Some(total.to_string())
    }
}
