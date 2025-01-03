use crate::util::Day;
use regex::Regex;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

const FONT_WIDTH: usize = 5;
const FONT_HEIGHT: usize = 6;

const LETTERS: &str = " ##  ###   ##  ###  #### ####  ##  #  # ##### ### #  # #    #  # #  #  ##  ###   ##  ###   ### #### #  # #  # #  # #  # #   ##### |
#  # #  # #  # #  # #    #    #  # #  #   #     # # #  #    #### ## # #  # #  # #  # #  # #     #   #  # #  # #  # #  # #   #  #  |
#  # ###  #    #  # ###  ###  #    ####   #     # ##   #    #  # # ## #  # #  # #  # #  #  ##   #   #  # #  # #  #  ##   # #  #   |
#### #  # #    #  # #    #    # ## #  #   #     # # #  #    #  # #  # #  # ###  #  # ###     #  #   #  # #  # #  # #  #   #   #   |
#  # #  # #  # #  # #    #    #  # #  #   #  #  # #  # #    #  # #  # #  # #    # #  #  #    #  #   #  #  # # #### #  #   #  #    |
#  # ###   ##  ###  #### #     ### #  # ##### ##  #  # #### #  # #  #  ##  #     # # #  # ###   #    ##    #  #  # #  #   #  #### |";

pub struct D8;

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

fn run_commands(input: &str, screen: &mut Vec<Vec<bool>>) {
    let re = Regex::new(r".*?(?P<type>rect|row|column).*?(?P<a>\d+).*?(?P<b>\d+)").unwrap();
    let mut commands = Vec::new();

    for line in input.trim().lines() {
        let captures = re.captures(line).unwrap();

        let a = captures["a"].parse().unwrap();
        let b = captures["b"].parse().unwrap();

        match captures["type"].as_ref() {
            "rect" => commands.push(Command::Rect(a, b)),
            "column" => commands.push(Command::RotateColumn(a, b)),
            "row" => commands.push(Command::RotateRow(a, b)),
            _ => panic!("Unknown command {}!", line),
        }
    }

    for command in commands {
        match command {
            Command::Rect(w, h) => {
                for x in 0..w {
                    for y in 0..h {
                        screen[y][x] = true;
                    }
                }
            }
            Command::RotateRow(y, n) => {
                let row = screen[y].clone();

                for x in 0..WIDTH {
                    screen[y][(x + n) % WIDTH] = row[x];
                }
            }
            Command::RotateColumn(x, n) => {
                let column: Vec<bool> = (0..HEIGHT).map(|y| screen[y][x]).collect();

                for y in 0..HEIGHT {
                    screen[(y + n) % HEIGHT][x] = column[y];
                }
            }
        }
    }
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut screen = vec![vec![false; WIDTH]; HEIGHT];

        run_commands(input, &mut screen);

        let mut total = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if screen[y][x] {
                    total += 1;
                }
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut screen = vec![vec![false; WIDTH]; HEIGHT];

        run_commands(input, &mut screen);

        let reference_letters = LETTERS
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut result = vec![];

        for x_offset in 0..WIDTH / FONT_WIDTH {
            let mut letter_matches = vec![0; 26];

            for letter in 0..26 {
                for y in 0..FONT_HEIGHT {
                    for x in 0..FONT_WIDTH {
                        match (
                            screen[y][x + x_offset * FONT_WIDTH],
                            reference_letters[y][x + letter * FONT_WIDTH],
                        ) {
                            (true, true) => letter_matches[letter] += 1,
                            (false, true) => letter_matches[letter] -= 1,
                            (true, false) => letter_matches[letter] -= 1,
                            _ => continue,
                        }
                    }
                }
            }

            let best_match_value = letter_matches.iter().max().unwrap();
            let best_letter = letter_matches
                .iter()
                .position(|&v| v == *best_match_value)
                .unwrap();

            result.push((best_letter as u8 + b'A') as char);
        }

        Option::from(result.iter().collect::<String>())
    }
}
