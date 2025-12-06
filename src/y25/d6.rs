use crate::util::Day;

pub struct D6;

fn solve(problems: Vec<Vec<usize>>, symbols: Vec<char>) -> usize {
    let mut total = 0;

    for (problem, symbol) in problems.iter().zip(symbols.iter()) {
        let mut solution = problem[0];

        for num in &problem[1..] {
            match symbol {
                '*' => solution *= num,
                '+' => solution += num,
                _ => unreachable!(),
            }
        }

        total += solution;
    }

    total
}

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut problems = Vec::new();
        let mut symbols = Vec::new();

        for line in input.lines() {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            if parts[0] == "+" || parts[0] == "*" {
                symbols = parts.iter().map(|s| s.chars().next().unwrap()).collect();
            } else {
                for (i, part) in parts.iter().enumerate() {
                    if i < parts.len() {
                        problems.push(Vec::new())
                    }

                    problems[i].push(part.parse::<usize>().unwrap());
                }
            }
        }

        Option::from(solve(problems, symbols).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut lines = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut symbols = lines
            .pop()
            .unwrap()
            .iter()
            .filter(|c| !c.is_whitespace())
            .map(|c| *c)
            .rev()
            .collect::<Vec<char>>();

        let mut problems = vec![vec![]; symbols.len()];

        let mut problem = 0;
        for i in (0..lines[0].len()).rev() {
            let mut num: usize = 0;
            let mut matched = false;

            for j in 0..lines.len() {
                match lines[j][i] {
                    c if c.is_digit(10) => {
                        num = (num * 10 + c.to_digit(10).unwrap() as usize);
                        matched = true;
                    }
                    _ => continue,
                }
            }

            if matched {
                problems[problem].push(num);
            } else {
                problem += 1;
            }
        }

        Option::from(solve(problems, symbols).to_string())
    }
}
