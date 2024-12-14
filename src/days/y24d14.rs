use regex::Regex;
use crate::util::Day;
use rayon::prelude::*;

static WIDTH: usize = 101;
static HEIGHT: usize = 103;

static STEPS: usize = 100;


#[derive(Debug)]
struct Bot {
    position: (i64, i64),
    velocity: (i64, i64),
}


impl Bot {
    fn step(&mut self, width: usize, height: usize) {
        self.position.0 = (self.position.0 + self.velocity.0 + width as i64) % (width as i64);
        self.position.1 = (self.position.1 + self.velocity.1 + height as i64) % (height as i64);
    }
}

fn parse_input(input: &str) -> Vec<Bot> {
    let re = Regex::new(r"[-+]?\d+").unwrap();

    let numbers = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<i64>().ok())
        .collect::<Vec<i64>>();

    numbers
        .chunks(4)
        .map(|parts| Bot {
            position: (parts[0], parts[1]),
            velocity: (parts[2], parts[3]),
        })
        .collect()
}


fn bot_position_variances(bots: &Vec<Bot>) -> i64 {
    // Extract x and y coordinates into separate vectors
    let mut x_coords: Vec<i64> = bots.iter().map(|bot| bot.position.0).collect();
    let mut y_coords: Vec<i64> = bots.iter().map(|bot| bot.position.1).collect();

    // Sort the coordinates
    x_coords.sort_unstable();
    y_coords.sort_unstable();

    // Calculate the sum of pairwise differences for both x and y
    let calculate_component_sum = |coords: &Vec<i64>| -> i64 {
        coords
            .iter()
            .enumerate()
            .map(|(i, &val)| val * (2 * i as i64 + 1 - coords.len() as i64))
            .sum()
    };

    let x_distance = calculate_component_sum(&x_coords);
    let y_distance = calculate_component_sum(&y_coords);

    x_distance * y_distance
}

fn _print_robots(bots: &Vec<Bot>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut count = 0;
            for bot in bots {
                if bot.position == (x as i64, y as i64) {
                    count += 1;
                }
            }

            match count {
                0 => print!("{}", "."),
                v => print!("{}", v)
            }
        }
        println!()
    }
}

pub struct Y24D14;

impl Day for Y24D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut bots = parse_input(input);

        let indicators: Vec<_> = bots.par_iter_mut().map(|bot| {
            for _ in 0..STEPS {
                bot.step(WIDTH, HEIGHT);
            }

            return (
                (bot.position.0 < (WIDTH / 2) as i64) as i64,
                (bot.position.0 > (WIDTH / 2) as i64) as i64,
                (bot.position.1 < (HEIGHT / 2) as i64) as i64,
                (bot.position.1 > (HEIGHT / 2) as i64) as i64,
            );
        }).collect();

        let mut quadrants = vec![0, 0, 0, 0];
        for indicator in indicators {
            // clarity > smartness
            match indicator {
                (1, 0, 1, 0) => quadrants[0] += 1,
                (0, 1, 1, 0) => quadrants[1] += 1,
                (1, 0, 0, 1) => quadrants[2] += 1,
                (0, 1, 0, 1) => quadrants[3] += 1,
                _ => continue,
            }
        }

        Option::from(quadrants.iter().product::<i64>().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut bots = parse_input(input);

        let mut min_dist = i64::MAX;
        let mut steps = 0;
        loop {
            bots.iter_mut().for_each(|bot| {
                bot.step(WIDTH, HEIGHT);
            });

            steps += 1;

            let distances = bot_position_variances(&bots);
            if distances < min_dist {
                min_dist = distances;

                // this probably a bit of a shit assumption, but it's not a well-formed problem
                // anyway and works well for my input, so we take those
                if steps > STEPS {
                    break;
                }
            }
        }

        Option::from(steps.to_string())
    }
}
