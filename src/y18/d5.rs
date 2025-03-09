use crate::util::Day;

pub struct D5;

impl D5 {
    fn react(polymer: &str) -> String {
        let mut stack = Vec::new();
        for c in polymer.chars() {
            if let Some(&last) = stack.last() {
                if (last as u8 ^ 32) as char == c {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.into_iter().collect()
    }
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(D5::react(input.trim()).len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let polymer = D5::react(input.trim());
        let min_length = (b'a'..=b'z')
            .map(|c| {
                let filtered: String = polymer
                    .chars()
                    .filter(|&ch| ch.to_ascii_lowercase() as u8 != c)
                    .collect();
                D5::react(&filtered).len()
            })
            .min()?;
        Some(min_length.to_string())
    }
}
