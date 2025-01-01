use crate::util::Day;

pub struct D5;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(&str, usize, usize, usize)>) {
    let (crates_str, commands_str) = input.split_once("\n\n").unwrap();
    let crates_rows: Vec<&str> = crates_str.lines().collect();
    let stack_count = crates_rows
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    for row in crates_rows.iter().rev().skip(1) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let index = 4 * i + 1;
            if let Some(char) = row.chars().nth(index) {
                if char != ' ' {
                    stack.push(char);
                }
            }
        }
    }

    let commands = commands_str
        .lines()
        .map(|command| {
            let parts: Vec<&str> = command.split_whitespace().collect();
            let count = parts[1].parse::<usize>().unwrap();
            let i_from = parts[3].parse::<usize>().unwrap() - 1;
            let i_to = parts[5].parse::<usize>().unwrap() - 1;
            (command, count, i_from, i_to)
        })
        .collect();

    (stacks, commands)
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut stacks, commands) = parse(input);

        for (_, count, i_from, i_to) in commands {
            for _ in 0..count {
                if let Some(item) = stacks[i_from].pop() {
                    stacks[i_to].push(item);
                }
            }
        }

        let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
        Some(result)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut stacks, commands) = parse(input);

        for (_, count, i_from, i_to) in commands {
            let mut tmp_stack = Vec::new();
            for _ in 0..count {
                if let Some(item) = stacks[i_from].pop() {
                    tmp_stack.push(item);
                }
            }
            for _ in 0..count {
                if let Some(item) = tmp_stack.pop() {
                    stacks[i_to].push(item);
                }
            }
        }

        let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
        Some(result)
    }
}
