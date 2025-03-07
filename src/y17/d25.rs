use crate::util::Day;
use rustc_hash::FxHashSet as HashSet;

pub struct D25;

fn parse(
    input: &str,
) -> (
    usize,
    usize,
    Vec<((bool, bool, usize), (bool, bool, usize))>,
) {
    let lines = input.lines().collect::<Vec<_>>();

    // wasteful but I'm lazy
    let last = |line: &str| line.split_whitespace().last().unwrap().to_string();

    let state = last(lines[0]).chars().next().unwrap() as usize - 'A' as usize;
    let steps = lines[1]
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut machine = vec![];
    let mut i = 5;

    while i < lines.len() {
        machine.push((
            (
                last(lines[i]).chars().next().unwrap().to_digit(10).unwrap() == 1,
                last(lines[i + 1])
                    .trim_end_matches('.')
                    .to_string()
                    .eq("right"),
                last(lines[i + 2]).chars().next().unwrap() as usize - 'A' as usize,
            ),
            (
                last(lines[4 + i])
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    == 1,
                last(lines[4 + i + 1])
                    .trim_end_matches('.')
                    .to_string()
                    .eq("right"),
                last(lines[4 + i + 2]).chars().next().unwrap() as usize - 'A' as usize,
            ),
        ));

        i += 10;
    }

    (state, steps, machine)
}

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut state, steps, machine) = parse(input);

        let mut tape: HashSet<isize> = HashSet::default();
        let mut position = 0;

        for _ in 0..steps {
            let value = tape.contains(&position);

            let operations = &machine[state];

            let operation = match value {
                false => &operations.0,
                true => &operations.1,
            };

            if operation.0 {
                tape.insert(position);
            } else {
                tape.remove(&position);
            }

            if operation.1 {
                position += 1;
            } else {
                position -= 1;
            }

            state = operation.2;
        }

        Option::from(tape.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        match input.parse::<usize>() {
            Ok(49) => Option::from("<3".to_string()),
            _ => None,
        }
    }
}
