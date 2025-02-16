use crate::util::Day;
use std::collections::HashSet;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for line in input.lines() {
            let mid = line.len() / 2;
            let (a, b) = line.split_at(mid);

            if let Some(&c) = a.chars().collect::<HashSet<_>>()
                .intersection(&b.chars().collect::<HashSet<_>>())
                .next()
            {
                total += if c.is_lowercase() {
                    (c as u8 - b'a' + 1) as i32
                } else {
                    (c as u8 - b'A' + 27) as i32
                };
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total = 0;
        let lines: Vec<&str> = input.lines().collect();

        for chunk in lines.chunks(3) {
            if let [a, b, c] = chunk {
                let set_a: HashSet<char> = a.chars().collect();
                let set_b: HashSet<char> = b.chars().collect();
                let set_c: HashSet<char> = c.chars().collect();

                if let Some(&char) = set_a.intersection(&set_b)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&set_c)
                    .next()
                {
                    total += if char.is_lowercase() {
                        (char as u8 - b'a' + 1) as i32
                    } else {
                        (char as u8 - b'A' + 27) as i32
                    };
                }
            }
        }

        Some(total.to_string())
    }
}
