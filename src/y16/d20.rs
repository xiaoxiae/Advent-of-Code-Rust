use crate::util::Day;
use rangemap::RangeInclusiveSet;

pub struct D20;


fn get_rangeset(input: &str) -> RangeInclusiveSet<u32, u32> {
    let mut blocked = RangeInclusiveSet::new();

    for line in input.lines() {
        let parts = line.trim().split('-').collect::<Vec<&str>>();
        blocked.insert(parts[0].parse::<u32>().unwrap()..=parts[1].parse::<u32>().unwrap());
    }

    blocked
}


impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let blocked = get_rangeset(input);

        // note that this assumes that 0 is blocked :)
        Option::from((blocked.iter().next().unwrap().end() + 1).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let blocked = get_rangeset(input);

        let mut start = 0;
        let mut allowed = 0;
        for range in blocked.iter() {
            allowed += range.start() - start;
            start = range.end().wrapping_add(1);
        }

        Option::from(allowed.to_string())
    }
}
