use crate::util::Day;

pub struct D6;

fn solve(input: &str, find_function: fn(&Vec<usize>) -> usize) -> String {
    let columns = input.lines().next().unwrap().len();

    let mut occurrences = vec![ vec![0usize; 26]; columns];

    for line in input.lines() {
        for (column, char) in line.chars().enumerate() {
            occurrences[column][(char as u8 - b'a') as usize] += 1;
        }
    }

    let mut message = String::new();
    for column in occurrences {
        let value = find_function(&column);

        message += &*(((column.iter().position(|&x| x == value).unwrap() as u8) + b'a') as char).to_string();
    }

    message
}


impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, |v| *v.iter().max().unwrap()))
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, |v| *v.iter().min().unwrap()))
    }
}
