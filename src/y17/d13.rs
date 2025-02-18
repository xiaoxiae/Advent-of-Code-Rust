use crate::util::Day;
use itertools::Itertools;

pub struct D13;

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let firewall = input.trim().split_terminator('\n')
            .map(|l| l.split(": ").map(|v| v.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>().unwrap())
            .collect::<Vec<_>>();

        let mut severity = 0;
        for (depth, range) in firewall {
            if depth % ((range - 1) * 2) == 0 {
                severity += depth * range;
            }
        }

        Option::from(severity.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let firewall = input.trim().split_terminator('\n')
            .map(|l| l.split(": ").map(|v| v.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>().unwrap())
            .collect::<Vec<_>>();

        // TODO: solve me analytically, this is slow!
        let mut delay = 0;
        'outer: loop {
            for (depth, range) in &firewall {
                if (depth + delay) % ((range - 1) * 2) == 0 {
                    delay += 1;
                    continue 'outer;
                }
            }

            break;
        }

        Option::from(delay.to_string())
    }
}
