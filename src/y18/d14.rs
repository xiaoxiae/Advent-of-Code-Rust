//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/14
use crate::util::Day;

pub struct D14;

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let number_of_chocolates: usize = input.lines().next()?.trim().parse().ok()?;

        // chocolates and elfs
        let mut chocolates: Vec<u8> = vec![3, 7];
        let mut elf1: usize = 0;
        let mut elf2: usize = 1;

        while chocolates.len() < number_of_chocolates + 10 {
            let sum = chocolates[elf1] + chocolates[elf2];
            if sum >= 10 {
                chocolates.push(sum / 10);
                chocolates.push(sum % 10);
            } else {
                chocolates.push(sum);
            }
            elf1 = (elf1 + chocolates[elf1] as usize + 1) % chocolates.len();
            elf2 = (elf2 + chocolates[elf2] as usize + 1) % chocolates.len();
        }

        let answer: String = chocolates[chocolates.len() - 10..]
            .iter()
            .map(|d| (b'0' + d) as char)
            .collect();
        Some(answer)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let recipes_str = input.lines().next()?.trim();
        let recipes: Vec<u8> = recipes_str.bytes().map(|b| b - b'0').collect();
        let rl = recipes.len();

        // chocolates and elfs
        let mut chocolates: Vec<u8> = vec![3, 7];
        let mut elf1: usize = 0;
        let mut elf2: usize = 1;

        // Faithful to the Python: after each iteration check whether `recipes`
        // is contained in the last `len(recipes)` chocolates, then report the
        // first index of `recipes` in the full sequence.
        loop {
            let sum = chocolates[elf1] + chocolates[elf2];
            if sum >= 10 {
                chocolates.push(sum / 10);
                chocolates.push(sum % 10);
            } else {
                chocolates.push(sum);
            }
            elf1 = (elf1 + chocolates[elf1] as usize + 1) % chocolates.len();
            elf2 = (elf2 + chocolates[elf2] as usize + 1) % chocolates.len();

            // Equivalent of `recipes not in chocolates[-len(recipes):]`:
            // the suffix of length `rl` contains `recipes` iff it equals it.
            if chocolates.len() >= rl && chocolates[chocolates.len() - rl..] == recipes[..] {
                break;
            }
        }

        // chocolates.index(recipes) -> first occurrence
        let idx = chocolates
            .windows(rl)
            .position(|w| w == recipes.as_slice())?;
        Some(idx.to_string())
    }
}
