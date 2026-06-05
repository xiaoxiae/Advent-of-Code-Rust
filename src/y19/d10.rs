//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/10
use crate::util::Day;

pub struct D10;

/// Greatest common divisor matching Python's `math.gcd` (non-negative result).
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Returns true if asteroid at (x2, y2) is in sight of the one at (x1, y1).
fn in_sight(x1: i64, y1: i64, x2: i64, y2: i64, area: &[Vec<char>]) -> bool {
    // skip itself
    if x1 == x2 && y1 == y2 {
        return false;
    }

    // go to the coordinate
    let x_d = x2 - x1;
    let y_d = y2 - y1;
    let multiple = gcd(x_d, y_d);

    let x_step = x_d / multiple;
    let y_step = y_d / multiple;

    let mut cx = x1 + x_step;
    let mut cy = y1 + y_step;

    // jump to x2, y2 until we hit something
    while cx != x2 || cy != y2 {
        if area[cy as usize][cx as usize] == '#' {
            return false;
        }

        cx += x_step;
        cy += y_step;
    }

    // if we didn't hit something, the position is valid!
    true
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Finds the best station position and its sight count.
fn best_station(area: &[Vec<char>]) -> (i64, i64, i64) {
    let height = area.len() as i64;
    let width = area[0].len() as i64;

    let mut max_sight = 0;
    let mut bx = 0;
    let mut by = 0;

    for y1 in 0..height {
        for x1 in 0..width {
            let mut subtotal = 0;

            if area[y1 as usize][x1 as usize] == '#' {
                for y2 in 0..height {
                    for x2 in 0..width {
                        if area[y2 as usize][x2 as usize] == '#'
                            && in_sight(x1, y1, x2, y2, area)
                        {
                            subtotal += 1;
                        }
                    }
                }

                if max_sight < subtotal {
                    max_sight = subtotal;
                    bx = x1;
                    by = y1;
                }
            }
        }
    }

    (bx, by, max_sight)
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let area = parse(input);
        let (_, _, max_sight) = best_station(&area);
        Some(max_sight.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let area = parse(input);
        let height = area.len() as i64;
        let width = area[0].len() as i64;

        let (x, y, _) = best_station(&area);

        // get the coordinates of all asteroids
        let mut asteroids: Vec<(i64, i64)> = Vec::new();
        for y1 in 0..height {
            for x1 in 0..width {
                if area[y1 as usize][x1 as usize] == '#' {
                    asteroids.push((x1, y1));
                }
            }
        }

        // sort their positions on their angles with the base station
        // key: (atan2(y - cy, x - cx) + pi / 2) % pi * 2
        let pi = std::f64::consts::PI;
        let key = |c: &(i64, i64)| -> f64 {
            let raw = ((y - c.1) as f64).atan2((x - c.0) as f64) + pi / 2.0;
            // Python modulo: result has same sign as divisor (always non-negative here)
            let m = raw.rem_euclid(pi);
            m * 2.0
        };
        asteroids.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());

        let mut asteroids_to_remove: Vec<(i64, i64)> = Vec::new();
        let mut removed_asteroids = 0;

        while !asteroids.is_empty() {
            for asteroid in &asteroids {
                // mark each asteroid in sight to be removed
                if in_sight(x, y, asteroid.0, asteroid.1, &area) {
                    asteroids_to_remove.push(*asteroid);
                    removed_asteroids += 1;

                    // if we're on the 200th, hooray!
                    if removed_asteroids == 200 {
                        return Some((asteroid.0 * 100 + asteroid.1).to_string());
                    }
                }
            }

            // remove marked asteroids
            while let Some(rem) = asteroids_to_remove.pop() {
                if let Some(pos) = asteroids.iter().position(|&a| a == rem) {
                    asteroids.remove(pos);
                }
            }
        }

        None
    }
}
