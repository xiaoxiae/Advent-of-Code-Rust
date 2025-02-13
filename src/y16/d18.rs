use crate::util::Day;
use std::mem::swap;

pub struct D18;


fn solve(input: &str, iterations: usize) -> usize {
    let state = input.trim().chars().map(|c| c == '.').collect::<Vec<_>>();

    // pad so we don't run out
    let mut padded_state = Vec::with_capacity(state.len() + 2);
    padded_state.push(true);
    padded_state.extend_from_slice(&state);
    padded_state.push(true);

    let mut new_state = padded_state.clone();

    let mut safe_states = state.iter().filter(|&&b| b).count();

    for _ in 0..iterations - 1 {
        for i in 0..state.len() {
            let neighbours = (padded_state[i], padded_state[i + 1], padded_state[i + 2]);

            new_state[i + 1] = match neighbours {
                (false, false, true) => false,
                (true, false, false) => false,
                (false, true, true) => false,
                (true, true, false) => false,
                _ => {
                    safe_states += 1;

                    true
                }
            }
        }

        swap(
            &mut new_state,
            &mut padded_state,
        )
    }

    safe_states
}


impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 40).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 400_000).to_string())
    }
}
