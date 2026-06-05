//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/12
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D12;

/// Parse the puzzle input into the (extended) initial state and the rule set.
fn parse(input: &str) -> (Vec<bool>, FxHashSet<[bool; 5]>) {
    let data: Vec<&str> = input.lines().collect();

    // get the input from line and extend it
    let initial: Vec<bool> = data[0].split(' ').nth(2).unwrap().chars().map(|c| c == '#').collect();
    let mut state: Vec<bool> = Vec::new();
    state.extend([false; 3]);
    state.extend(initial);
    state.extend([false; 3]);

    // convert rules that produce plants to a set
    let mut rules: FxHashSet<[bool; 5]> = FxHashSet::default();
    for rule in &data[2..] {
        if rule.is_empty() {
            continue;
        }
        let parts: Vec<&str> = rule.split(" => ").collect();
        // ignore rules that don't produce plants and add those that do to the set
        if parts[1] == "." {
            continue;
        }
        let config: Vec<bool> = parts[0].chars().map(|c| c == '#').collect();
        let arr: [bool; 5] = [config[0], config[1], config[2], config[3], config[4]];
        rules.insert(arr);
    }

    (state, rules)
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut state, rules) = parse(input);

        // iterate over 20 generations
        let mut beginning_index: i64 = -3;
        for _ in 0..20 {
            // resize the input array if necessary
            if state[2] {
                state.insert(0, false);
                beginning_index -= 1;
            }
            if state[state.len() - 3] {
                state.push(false);
            }

            // create the next generation
            let mut new_state = vec![false; state.len()];
            for j in 2..state.len() - 1 {
                // Python's input[j-2:j+3] silently shrinks at the right edge,
                // so a full 5-window only exists while j + 2 < len.
                if j + 2 >= state.len() {
                    continue;
                }
                let window: [bool; 5] = [state[j - 2], state[j - 1], state[j], state[j + 1], state[j + 2]];
                if rules.contains(&window) {
                    new_state[j] = true;
                }
            }

            state = new_state;
        }

        // sum the result
        let total: i64 = state
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(i, _)| i as i64 + beginning_index)
            .sum();

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut state, rules) = parse(input);

        // iterate until we find the generation that repeats forever
        let mut beginning_index: i64 = -3;
        let mut generation: i64 = 0;
        let mut previous_number: Option<String> = None;
        loop {
            // resize the input array if necessary
            if state[2] {
                state.insert(0, false);
                beginning_index -= 1;
            }
            if state[state.len() - 3] {
                state.push(false);
            }

            // create the next generation
            let mut new_state = vec![false; state.len()];
            for j in 2..state.len() - 1 {
                // Python's input[j-2:j+3] silently shrinks at the right edge,
                // so a full 5-window only exists while j + 2 < len.
                if j + 2 >= state.len() {
                    continue;
                }
                let window: [bool; 5] = [state[j - 2], state[j - 1], state[j], state[j + 1], state[j + 2]];
                if rules.contains(&window) {
                    new_state[j] = true;
                }
            }

            state = new_state;
            generation += 1;

            // the numerical representation of the generation:
            // join bits as "1"/"0", drop the last two characters ([:-2]) and
            // interpret as a binary number. We compare the trimmed bit-string,
            // which is equivalent to comparing the integer values.
            let bits: String = state.iter().map(|&v| if v { '1' } else { '0' }).collect();
            // drop last two chars (Python's [:-2])
            let trimmed = &bits[..bits.len().saturating_sub(2)];
            // strip leading zeros to mimic int(..., 2) canonical comparison
            let canonical = trimmed.trim_start_matches('0');
            let number = canonical.to_string();

            // check whether the numeric representation doesn't repeat
            if Some(&number) == previous_number.as_ref() {
                // fast-forward the 50000000000 generations
                beginning_index += 50_000_000_000 - generation;
                break;
            }

            previous_number = Some(number);
        }

        // sum the result
        let total: i64 = state
            .iter()
            .enumerate()
            .filter(|(_, &v)| v)
            .map(|(i, _)| i as i64 + beginning_index)
            .sum();

        Some(total.to_string())
    }
}
