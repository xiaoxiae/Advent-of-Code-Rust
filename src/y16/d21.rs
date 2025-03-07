use crate::util::Day;
use std::collections::VecDeque;

pub struct D21;

#[derive(Debug, PartialEq)]
enum Command {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetter(char),
    ReversePositions(usize, usize),
    MovePosition(usize, usize),
}

impl Command {
    fn apply(&self, password: &mut VecDeque<char>) {
        match *self {
            Command::SwapPosition(i, j) => password.swap(i, j),
            Command::SwapLetter(c, d) => {
                let i = password.iter().position(|&x| x == c).unwrap();
                let j = password.iter().position(|&x| x == d).unwrap();

                password.swap(i, j);
            }
            Command::RotateLeft(n) => {
                for _ in 0..n {
                    let v = password.pop_front().unwrap();
                    password.push_back(v);
                }
            }
            Command::RotateRight(n) => {
                for _ in 0..n {
                    let v = password.pop_back().unwrap();
                    password.push_front(v);
                }
            }
            Command::RotateBasedOnLetter(c) => {
                let mut n = password.iter().position(|&x| x == c).unwrap();

                if n >= 4 {
                    n += 1;
                }

                n += 1;

                for _ in 0..n {
                    let v = password.pop_back().unwrap();
                    password.push_front(v);
                }
            }
            Command::ReversePositions(from, to) => {
                for i in 0..(to - from + 1) / 2 {
                    password.swap(from + i, to - i)
                }
            }
            Command::MovePosition(from, to) => {
                let v = password.remove(from).unwrap();
                password.insert(to, v);
            }
        }
    }

    fn apply_inverse(&self, password: &mut VecDeque<char>) {
        match *self {
            Command::SwapPosition(_, _) => self.apply(password),
            Command::SwapLetter(_, _) => self.apply(password),
            Command::RotateLeft(n) => Command::RotateRight(n).apply(password),
            Command::RotateRight(n) => Command::RotateLeft(n).apply(password),
            Command::ReversePositions(_, _) => self.apply(password),
            Command::MovePosition(from, to) => Command::MovePosition(to, from).apply(password),
            Command::RotateBasedOnLetter(c) => {
                // since this command is specially designed to be a bijection for n=8,
                // I just hard code it using the following pattern
                //
                // 01234567
                // .o
                //  . o
                //   .  o
                //    .   o
                //   o .     >= 4, so +1
                //     o.
                //       o
                // o      .

                let n = password.iter().position(|&x| x == c).unwrap();

                let command = if n % 2 == 1 {
                    match n {
                        1 => Command::RotateLeft(1),
                        3 => Command::RotateLeft(2),
                        5 => Command::RotateLeft(3),
                        7 => Command::RotateLeft(4),
                        _ => panic!(),
                    }
                    // Command::RotateLeft(n / 2)
                } else {
                    match n {
                        0 => Command::RotateLeft(1),
                        2 => Command::RotateRight(2),
                        4 => Command::RotateRight(1),
                        6 => Command::RotateRight(0),
                        _ => panic!(),
                    }
                };

                command.apply(password);
            }
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    let mut commands = vec![];

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let command = match parts.as_slice() {
            ["swap", "position", x, "with", "position", y] => {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                Command::SwapPosition(x, y)
            }
            ["swap", "letter", x, "with", "letter", y] => {
                let x = x.chars().next().unwrap();
                let y = y.chars().next().unwrap();
                Command::SwapLetter(x, y)
            }
            ["rotate", "left", x, "steps"] | ["rotate", "left", x, "step"] => {
                let x = x.parse::<usize>().unwrap();
                Command::RotateLeft(x)
            }
            ["rotate", "right", x, "steps"] | ["rotate", "right", x, "step"] => {
                let x = x.parse::<usize>().unwrap();
                Command::RotateRight(x)
            }
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                let x = x.chars().next().unwrap();
                Command::RotateBasedOnLetter(x)
            }
            ["reverse", "positions", x, "through", y] => {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                Command::ReversePositions(x, y)
            }
            ["move", "position", x, "to", "position", y] => {
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                Command::MovePosition(x, y)
            }
            _ => panic!("{}", format!("invalid command: {}", line)),
        };

        commands.push(command);
    }

    commands
}


impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let commands = parse(input);
        let mut password = "abcdefgh".chars().collect::<VecDeque<_>>();

        for command in commands {
            command.apply(&mut password);
        }

        Option::from(password.iter().collect::<String>())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let commands = parse(input);
        let mut password = "fbgdceah".chars().collect::<VecDeque<_>>();

        for command in commands.iter().rev() {
            command.apply_inverse(&mut password);
        }

        Option::from(password.iter().collect::<String>())
    }
}
