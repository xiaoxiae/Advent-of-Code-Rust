use regex::Regex;
use crate::util::Day;

pub struct D25;

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"\d+").unwrap();

        let cordinates = re.find_iter(input)
            .filter_map(|mat| mat.as_str().parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let row = cordinates[0];
        let column = cordinates[1];

        let n = column + row;
        let triangle = (n * (n - 1)) / 2;

        let count = triangle - n + column;

        let mut code: usize = 20151125;
        for _ in 0..count {
            code = (code * 252533) % 33554393;
        }

        Option::from(code.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        match input.parse::<usize>() {
            Ok(49) => Option::from("<3".to_string()),
            _ => None,
        }
    }
}
