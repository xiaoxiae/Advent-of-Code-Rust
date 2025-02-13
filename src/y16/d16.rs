use crate::util::Day;
use itertools::Itertools;

pub struct D16;

fn get_dragon_curve(mut initial: &Vec<bool>, disk_size: usize) -> Vec<bool> {
    let mut initial = initial.clone();

    while initial.len() < disk_size {
        let mut new_initial = Vec::with_capacity(initial.len() * 2 + 1);
        new_initial.extend_from_slice(&initial);
        new_initial.push(false);
        new_initial.extend(initial.iter().rev().map(|&b| !b));

        initial = new_initial;
    }

    initial[..disk_size].to_vec()
}

fn get_checksum(curve: &Vec<bool>) -> Vec<bool> {
    let mut checksum = curve.clone();

    while checksum.len() % 2 != 1 {
        let mut new_checksum = Vec::with_capacity(checksum.len() / 2);

        for i in 0..checksum.len() / 2 {
            new_checksum.push(checksum[i * 2] == checksum[i * 2 + 1]);
        }

        checksum = new_checksum;
    }

    checksum
}

fn solve(input: &str, disk_size: usize) -> String {
    let initial = input.trim().chars().map(|c| c == '1').collect();

    let curve = get_dragon_curve(&initial, disk_size);
    let checksum = get_checksum(&curve);

    checksum.iter().map(|&b| if b { '1' } else { '0' }).collect::<String>()
}


impl Day for D16 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 272))
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 35651584))
    }
}
