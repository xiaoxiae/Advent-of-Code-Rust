use crate::util::Day;

pub struct D15;

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut lines = input.lines();

        // the unholy triple-unwrap in a single line; they said it couldn't be done
        let mut a = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let mut b = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

        let mut count = 0;

        for _ in 0..40_000_000 {
            a = (a * 16807) % 2147483647;
            b = (b * 48271) % 2147483647;

            if (a & 0xFFFF) == (b & 0xFFFF) {
                count += 1;
            }
        }

        Option::from(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut lines = input.lines();

        // the unholy triple-unwrap in a single line; they said it couldn't be done
        let mut a = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();
        let mut b = lines.next().unwrap().split_whitespace().last().unwrap().parse::<usize>().unwrap();

        let mut count = 0;

        for _ in 0..5_000_000 {
            loop {
                a = (a * 16807) % 2147483647;

                if a % 4 == 0 {
                    break;
                }
            }

            loop {
                b = (b * 48271) % 2147483647;

                if b % 8 == 0 {
                    break;
                }
            }

            if (a & 0xFFFF) == (b & 0xFFFF) {
                count += 1;
            }
        }

        Option::from(count.to_string())
    }
}
