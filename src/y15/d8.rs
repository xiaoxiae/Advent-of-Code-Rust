use crate::util::Day;

pub struct D8;

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let total: usize = input
            .lines()
            .map(|l| {
                let mut i: usize = 0;

                let mut chars = 0;
                let l = l.chars().collect::<Vec<char>>();

                loop {
                    match l[i..] {
                        ['\\', '\\', ..] | ['\\', '"', ..] => i += 2,
                        ['\\', 'x', _, _, ..] => i += 4,
                        [] => break,
                        _ => i += 1,
                    }

                    chars += 1;
                }

                l.len() - (chars - 2)
            })
            .sum();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let total: usize = input
            .lines()
            .map(|l| {
                let mut i: usize = 0;

                let mut chars = 0;
                let l = l.chars().collect::<Vec<char>>();

                let mut i = 0;
                while i < l.len() {
                    match l[i] {
                        '\\' | '"' => chars += 2,
                        _ => chars += 1,
                    }

                    i += 1
                }

                (chars + 2) - l.len()
            })
            .sum();

        Option::from(total.to_string())
    }
}
