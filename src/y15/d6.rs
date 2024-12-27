use crate::util::Day;
use regex::Regex;

pub struct D6;

static WIDTH: usize = 1_000;

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut grid = vec![vec![false; WIDTH]; WIDTH];
        let re = Regex::new(r"^.*?(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

        input.lines().for_each(|l| {
            let m = re.captures(l).unwrap();

            let command = &m[1];
            let nums: Vec<_> = [&m[2], &m[3], &m[4], &m[5]]
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for x in nums[0]..=nums[2] {
                for y in nums[1]..=nums[3] {
                    match command {
                        "on" => grid[x][y] = true,
                        "off" => grid[x][y] = false,
                        "toggle" => grid[x][y] = !grid[x][y],
                        _ => {}
                    }
                }
            }
        });

        let mut total = 0;
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                if grid[x][y] {
                    total += 1;
                }
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut grid = vec![vec![0usize; WIDTH]; WIDTH];
        let re = Regex::new(r"^.*?(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

        input.lines().for_each(|l| {
            let m = re.captures(l).unwrap();

            let command = &m[1];
            let nums: Vec<_> = [&m[2], &m[3], &m[4], &m[5]]
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            for x in nums[0]..=nums[2] {
                for y in nums[1]..=nums[3] {
                    match command {
                        "on" => grid[x][y] += 1,
                        "off" => {
                            if grid[x][y] > 0 {
                                grid[x][y] -= 1;
                            }
                        }
                        "toggle" => grid[x][y] += 2,
                        _ => {}
                    }
                }
            }
        });

        let mut total = 0;
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                total += grid[x][y];
            }
        }

        Option::from(total.to_string())
    }
}
