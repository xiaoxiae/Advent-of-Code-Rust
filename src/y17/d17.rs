use crate::util::Day;

pub struct D17;

impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let steps = input.trim().parse::<usize>().unwrap();
        let insertions = 2017;

        let mut buffer = vec![0];
        let mut position = 0;

        for element in 1..insertions {
            buffer.insert(position + 1, element);
            position = (position + steps + 1) % buffer.len();
        }

        Option::from(buffer[(position + 1) % buffer.len()].to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let steps = input.trim().parse::<usize>().unwrap();
        let insertions = 50_000_000;

        let mut zero_position = 0;
        let mut after_zero = 1;

        let mut position = 1;

        for size in 2..insertions {
            position = (position + steps) % size;

            // hit zero -- insert after it
            if position == zero_position {
                after_zero = size;
            } else if position < zero_position {
                zero_position += 1;
            }

            position += 1;
        }

        Option::from(after_zero.to_string())
    }
}
