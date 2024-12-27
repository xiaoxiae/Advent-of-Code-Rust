use crate::util::Day;
use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;

pub struct D7;

#[derive(Debug)]
enum Command {
    Unary {
        operator: String,
        input: String,
        output: String,
    },
    Binary {
        left: String,
        operator: String,
        right: String,
        output: String,
    },
    Assign {
        input: String,
        output: String,
    },
}

fn resolve_value_or_error(
    value_or_input: &str,
    wires: &HashMap<String, u16>,
) -> Result<u16, String> {
    if let Ok(value) = value_or_input.parse::<u16>() {
        Ok(value)
    } else if let Some(&value) = wires.get(value_or_input) {
        Ok(value)
    } else {
        Err(format!("Could not resolve value: {}", value_or_input))
    }
}

fn solve(input: &str, b_value: Option<u16>) -> u16 {
    let mut commands: Vec<Command> = Vec::new();

    let binary_re = Regex::new(r"(\w+) (AND|OR|LSHIFT|RSHIFT) (\w+) -> (\w+)").unwrap();
    let unary_re = Regex::new(r"(\w+?) (\w+?) -> (\w+)").unwrap();
    let assign_re = Regex::new(r"(\d+|[a-z]+) -> (\w+)").unwrap();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(captures) = binary_re.captures(line) {
            commands.push(Command::Binary {
                left: captures[1].to_string(),
                operator: captures[2].to_string(),
                right: captures[3].to_string(),
                output: captures[4].to_string(),
            });
        } else if let Some(captures) = unary_re.captures(line) {
            commands.push(Command::Unary {
                operator: captures[1].to_string(),
                input: captures[2].to_string(),
                output: captures[3].to_string(),
            });
        } else if let Some(captures) = assign_re.captures(line) {
            commands.push(Command::Assign {
                input: captures[1].to_string(),
                output: captures[2].to_string(),
            });
        } else {
            eprintln!("Unknown command: {}", line);
        }
    }

    let mut wires: HashMap<String, u16> = HashMap::new();

    if let Some(v) = b_value {
        wires.insert("b".to_string(), v);
    }

    let mut iter = 0;

    while commands.len() != 0 {
        iter += 1;

        let mut i = 0;
        while i < commands.len() {
            let command = &commands[i];

            match command {
                Command::Assign {
                    input: value_or_input,
                    output,
                } => match resolve_value_or_error(value_or_input, &wires) {
                    Ok(value) => {
                        if output != "b" || b_value.is_none() {
                            wires.insert(output.to_string(), value);
                        }

                        commands.remove(i);
                    }
                    _ => i += 1,
                },
                Command::Unary {
                    operator,
                    input,
                    output,
                } => match resolve_value_or_error(input, &wires) {
                    Ok(value) => {
                        wires.insert(
                            output.to_string(),
                            match operator.as_str() {
                                "NOT" => !value,
                                _ => panic!(),
                            },
                        );
                        commands.remove(i);
                    }
                    _ => i += 1,
                },
                Command::Binary {
                    left,
                    operator,
                    right,
                    output,
                } => {
                    match (
                        resolve_value_or_error(left, &wires),
                        resolve_value_or_error(right, &wires),
                    ) {
                        (Ok(left), Ok(right)) => {
                            wires.insert(
                                output.to_string(),
                                match operator.as_str() {
                                    "AND" => left & right,
                                    "OR" => left | right,
                                    "LSHIFT" => left << right,
                                    "RSHIFT" => left >> right,
                                    _ => panic!(),
                                },
                            );
                            commands.remove(i);
                        }
                        _ => i += 1,
                    }
                }
            }
        }
    }

    wires["a"]
}

impl Day for D7 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, None).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let a = solve(input, None);

        Option::from(solve(input, Option::from(a)).to_string())
    }
}
