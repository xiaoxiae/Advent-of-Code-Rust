use crate::util::Day;

pub struct D19;

fn solve(input: &str) -> (String, i32) {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_x = map[0].iter().position(|&c| c == '|').unwrap();

    let mut current = (start_x, 0);
    let mut direction: (isize, isize) = (0, 1);

    let mut password = String::new();
    let mut steps = 0;

    loop {
        let char = map.get(current.1).and_then(|l| l.get(current.0));
        steps += 1;

        match char {
            None | Some(' ') => break,
            Some(char) => {
                match char {
                    &c if c.is_alphabetic() => password.push(c),
                    '+' => {
                        let left_turn = (direction.1, -direction.0);

                        if match map
                            .get((current.1 as isize + left_turn.1) as usize)
                            .and_then(|l| l.get((current.0 as isize + left_turn.0) as usize))
                        {
                            None | Some(' ') => false,
                            _ => true,
                        } {
                            direction = left_turn;
                        } else {
                            direction = (-direction.1, direction.0);
                        }
                    }
                    _ => {}
                }

                current.0 = (current.0 as isize + direction.0) as usize;
                current.1 = (current.1 as isize + direction.1) as usize;
            }
        }
    }

    (password, steps - 1)
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input).0)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input).1.to_string())
    }
}
