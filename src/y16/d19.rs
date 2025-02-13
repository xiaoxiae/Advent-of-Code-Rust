use crate::util::Day;

pub struct D19;

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut remaining = input.trim().parse::<usize>().unwrap();
        let mut round = 0;

        let mut last = 0;

        while remaining != 1 {
            round += 1;

            // even-sized remainings don't change first living elf
            // odd-sized remainings do, in this manner (draw it out and think about why)
            if remaining % 2 == 1 {
                last += 2usize.pow(round);
            }

            remaining /= 2;
        }

        Option::from((last + 1).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut i = input.trim().parse::<usize>().unwrap();

        // printed out and reverse-engineered the pattern
        // can't explain this shit
        let pow = i.ilog(3usize);
        let offset_pow = 3usize.pow(pow);

        let diff = i - offset_pow;
        let last;

        if diff == 0 {
            last = offset_pow;
        } else if diff < offset_pow {
            last = diff;
        } else {
            last = offset_pow + (diff - offset_pow) * 2;
        }

        Option::from(last.to_string())
    }
}
