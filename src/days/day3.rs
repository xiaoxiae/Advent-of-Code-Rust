use regex::Regex;
use crate::util::Day;

pub struct Day3;

impl Day for Day3 {
    fn solve_part1(&self, input: &str) -> String {
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

        total.to_string()
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut total = 0;
        let mut enabled = true;

        for line in input.trim().lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

            for cap in re.captures_iter(line) {
                let whole = cap.get(0).unwrap().as_str();

                if whole == "do()" {
                    enabled = true;
                } else if  whole == "don't()" {
                    enabled = false;
                } else if enabled {
                    let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    total += x * y;
                }
            }
        }

        total.to_string()
    }
}
