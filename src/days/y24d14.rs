use crate::util::Day;
use rayon::prelude::*;
use regex::Regex;

static WIDTH: usize = 101;
static HEIGHT: usize = 103;

static STEPS: usize = 100;

#[derive(Debug, Clone)]
struct Bot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Bot {
    fn step(&mut self, width: usize, height: usize) {
        self.position.0 = (self.position.0 + self.velocity.0 + width as i64) % (width as i64);
        self.position.1 = (self.position.1 + self.velocity.1 + height as i64) % (height as i64);
    }

    fn step_by(&mut self, steps: usize, width: usize, height: usize) {
        self.position.0 =
            (self.position.0 + self.velocity.0 * (steps as i64)).rem_euclid(width as i64);
        self.position.1 =
            (self.position.1 + self.velocity.1 * (steps as i64)).rem_euclid(height as i64);
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

fn bot_position_variance(bots: &Vec<Bot>) -> i64 {
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

pub struct Y24D14;

impl Day for Y24D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut bots = parse_input(input);

        let indicators: Vec<_> = bots
            .par_iter_mut()
            .map(|bot| {
                for _ in 0..STEPS {
                    bot.step(WIDTH, HEIGHT);
                }

                return (
                    (bot.position.0 < (WIDTH / 2) as i64) as i64,
                    (bot.position.0 > (WIDTH / 2) as i64) as i64,
                    (bot.position.1 < (HEIGHT / 2) as i64) as i64,
                    (bot.position.1 > (HEIGHT / 2) as i64) as i64,
                );
            })
            .collect();

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
        let bots = parse_input(input);

        const BATCH_SIZE: usize = 512;

        let mut iteration = 0;
        let mut min_variance = i64::MAX;
        let mut min_steps;

        loop {
            let batch: Vec<_> = (0..BATCH_SIZE)
                .into_par_iter()
                .map(|thread_idx| {
                    let mut local_bots = bots.clone();

                    let steps = thread_idx + iteration * BATCH_SIZE;

                    local_bots.iter_mut().for_each(|bot| {
                        bot.step_by(steps, WIDTH, HEIGHT);
                    });

                    let variance = bot_position_variance(&local_bots);

                    variance
                })
                .collect();

            let min_batch_variance = *batch.iter().min().unwrap();

            if min_batch_variance < min_variance {
                min_variance = min_batch_variance;
                min_steps = iteration * BATCH_SIZE
                    + batch.iter().position(|&x| x == min_batch_variance).unwrap();

                // we assume that the variance of the image will be much less than that of random robots
                // i.e. that it improves by a large amount (>25%; a bit arbitrary but fuck it we ball)
                if iteration != 0 && (min_batch_variance as f64) < (min_variance as f64) * 1.25 {
                    break;
                }
            }

            iteration += 1;
        }

        Option::from(min_steps.to_string())
    }
}
