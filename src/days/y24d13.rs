use crate::util::Day;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

static BUTTON_A_PRICE: usize = 3;
static BUTTON_B_PRICE: usize = 1;

static OOPSIE_DOOPSIE_FUCKY_WUCKY: usize = 10000000000000;

static MAX_STEPS: usize = 100;

pub struct Y24D13;

#[derive(Debug)]
struct Contraption {
    button_a: (usize, usize),
    button_b: (usize, usize),
    goal: (usize, usize),
}

fn parse_input(input: &str) -> Vec<Contraption> {
    let re = Regex::new(r"[-+]?\d+").unwrap();

    let numbers = re
        .find_iter(input)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    numbers
        .chunks(6)
        .map(|parts| Contraption {
            button_a: (parts[0], parts[1]),
            button_b: (parts[2], parts[3]),
            goal: (parts[4], parts[5]),
        })
        .collect()
}

/// My second solution (first one was Dijkstra because I expected some state space search shenanigans)
fn find_path_bruteforce(contraption: &Contraption, max_steps: usize) -> Option<usize> {
    let mut min = usize::MAX;
    for i in 0..max_steps {
        let x = contraption.button_a.0 * i;
        let y = contraption.button_a.1 * i;

        let (rx, ry) = (contraption.goal.0 - x, contraption.goal.1 - y);

        if (rx % contraption.button_b.0 == 0 && ry % contraption.button_b.1 == 0)
            && rx / contraption.button_b.0 == ry / contraption.button_b.1
        {
            let dist = i * BUTTON_A_PRICE + (rx / contraption.button_b.0) * BUTTON_B_PRICE;
            min = usize::min(min, dist);
        }
    }

    match min {
        usize::MAX => None,
        other => Some(other),
    }
}

/// Faster second solution, skipping over unnecessary checks
fn find_path_faster(contraption: &Contraption) -> Option<usize> {
    let mut min = usize::MAX;
    let mut i = 0;

    loop {
        let x = contraption.button_a.0 * i;
        let y = contraption.button_a.1 * i;

        if x > contraption.goal.0 || y > contraption.goal.1 {
            break;
        }

        let rx = contraption.goal.0 - x;
        let ry = contraption.goal.1 - y;

        let mx = rx / contraption.button_b.0;
        let my = ry / contraption.button_b.1;

        if mx == my && rx % contraption.button_b.0 == 0 && ry % contraption.button_b.1 == 0 {
            let dist = i * BUTTON_A_PRICE + (rx / contraption.button_b.0) * BUTTON_B_PRICE;
            min = usize::min(min, dist);
        }

        // mx and my give us the values by which to multiply button_b to obtain equality
        // if there is a difference, we can skip a lot of checks by incrementing i based on it
        //
        // we can skip at least their difference, divided by the largest button_a value
        // the division essentially acts as if incrementing i has no difference on the overall result
        if mx != my {
            i += usize::max(
                (usize::max(mx, my) - usize::min(mx, my))
                    / usize::max(contraption.button_a.0, contraption.button_a.1),
                1,
            );
        } else {
            i += 1;
        }
    }

    match min {
        usize::MAX => None,
        other => Some(other),
    }
}

/// Math is OP
fn find_path_math(contraption: &Contraption, max_steps: usize) -> Option<usize> {
    // if we cast a ray from start (0, 0) with button presses A
    // and one from the end (goal) with button presses B,
    // they will either collide at one point, or not at all (in our case)
    //
    // the price is a red herring since it doesn't affect the calculation
    //
    // i [ax ay] + j [bx by] = [cx cy]
    //
    // i ax + j bx = cx
    // i ay + j by = cy
    //
    // i ax + j by (ax / ay) = cy (ax / ay)
    //        j by (ax / ay) - j bx = cy (ax / ay) - cx
    //        j (by (ax / ay) - bx) = cy (ax / ay) - cx
    //        j = (cy (ax / ay) - cx) / (by (ax / ay) - bx)
    //
    // i = (cx - j bx) / ax

    let (ax, ay) = (contraption.button_a.0 as f64, contraption.button_a.1 as f64);
    let (bx, by) = (contraption.button_b.0 as f64, contraption.button_b.1 as f64);
    let (cx, cy) = (contraption.goal.0 as f64, contraption.goal.1 as f64);

    let j = (cy * (ax / ay) - cx) / (by * (ax / ay) - bx);
    let i = (cx - j * bx) / ax;

    if i > max_steps as f64 || j > max_steps as f64 {
        return None;
    }

    // NOTE: this is a bit finicky - both 1e-1 and 1e-4 give incorrect results!!!
    if i > 0f64 && j > 0f64 && (i.round() - i).abs() < 1e-3 && (j.round() - j).abs() < 1e-3 {
        return Some((i.round() as usize) * BUTTON_A_PRICE + (j.round() as usize) * BUTTON_B_PRICE);
    }

    None
}

impl Day for Y24D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(
            parse_input(input)
                .iter()
                .map(|c| find_path_math(c, MAX_STEPS))
                .filter_map(|x| x)
                .sum::<usize>()
                .to_string(),
        )
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(
            parse_input(input)
                .par_iter()
                .map(|c| {
                    find_path_math(
                        &Contraption {
                            button_a: c.button_a,
                            button_b: c.button_b,
                            goal: (
                                c.goal.0 + OOPSIE_DOOPSIE_FUCKY_WUCKY,
                                c.goal.1 + OOPSIE_DOOPSIE_FUCKY_WUCKY,
                            ),
                        },
                        usize::MAX,
                    )
                })
                .filter_map(|x| x)
                .sum::<usize>()
                .to_string(),
        )
    }
}
