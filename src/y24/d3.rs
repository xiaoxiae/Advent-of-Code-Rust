use regex::Regex;
use crate::util::Day;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;

        for line in input.trim().lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

            total += re.captures_iter(line)
                .map(|cap|
                    {
                        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

                        return x * y;
                    }
                )
                .sum::<i32>();
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total = 0;
        let mut enabled = true;

        for line in input.trim().lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

            for cap in re.captures_iter(line) {
                let whole = cap.get(0).unwrap().as_str();

                match whole {
                    "do()" => {
                        enabled = true;
                    }
                    "don't()" => {
                        enabled = false;
                    }
                    _ if enabled => {
                        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                        total += x * y;
                    }
                    _ => {}
                }
            }
        }

        Option::from(total.to_string())
    }

    /// --- Tom's Part 3 ---
    /// Actually, we would like to match things in the form "mul(a, b)...mul(c, d)"
    /// What we want to do for each match is to calculate a * d + b...c, where b...c
    /// is a concatenation of b, all digits in ... and c. Do and Don't still work.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let mut total = 0;
        let mut enabled = true;

        for line in input.trim().lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)(.+?)(\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

            for cap in re.captures_iter(line) {
                let whole = cap.get(0).unwrap().as_str();

                match whole {
                    "do()" => {
                        enabled = true;
                    }
                    "don't()" => {
                        enabled = false;
                    }
                    _ if enabled => {
                        let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                        let d = cap.get(5).unwrap().as_str().parse::<i64>().unwrap();

                        let b = cap.get(2).unwrap().as_str();
                        let c = cap.get(4).unwrap().as_str();
                        let middle = cap.get(3).unwrap().as_str()
                            .chars().filter(|c| c.is_digit(10)).collect::<String>();

                        total += a * d + format!("{}{}{}", b, middle, c).parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
        }

        Option::from(total.to_string())
    }
}
