use crate::util::Day;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let instructions = input.trim().lines().collect::<Vec<&str>>();
        let mapping = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let mut x = 1;
        let mut y = 1;
        let mut result = String::new();

        for inst in instructions {
            for char in inst.chars() {
                match char {
                    'U' => y = if y > 0 { y - 1 } else { y },
                    'D' => y = if y < 2 { y + 1 } else { y },
                    'L' => x = if x > 0 { x - 1 } else { x },
                    'R' => x = if x < 2 { x + 1 } else { x },
                    _ => {}
                }
            }
            result.push_str(&mapping[y][x].to_string());
        }

        Some(result)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let instructions = input.trim().lines().collect::<Vec<&str>>();
        let mapping = [
            ["", "", "1", "", ""],
            ["", "2", "3", "4", ""],
            ["5", "6", "7", "8", "9"],
            ["", "A", "B", "C", ""],
            ["", "", "D", "", ""],
        ];
        let mut x = 0;
        let mut y = 2;
        let mut result = String::new();

        for inst in instructions {
            for char in inst.chars() {
                match char {
                    'U' => {
                        if y > 0 && !mapping[y - 1][x].is_empty() {
                            y -= 1;
                        }
                    }
                    'D' => {
                        if y < 4 && !mapping[y + 1][x].is_empty() {
                            y += 1;
                        }
                    }
                    'L' => {
                        if x > 0 && !mapping[y][x - 1].is_empty() {
                            x -= 1;
                        }
                    }
                    'R' => {
                        if x < 4 && !mapping[y][x + 1].is_empty() {
                            x += 1;
                        }
                    }
                    _ => {}
                }
            }
            result.push_str(mapping[y][x]);
        }

        Some(result)
    }
}
