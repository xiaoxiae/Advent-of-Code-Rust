use crate::util::Day;
use regex::Regex;

pub struct D20;

struct Particle {
    position: [i64; 3],
    velocity: [i64; 3],
    acceleration: [i64; 3],
}

fn parse(input: &str) -> Vec<Particle> {
    let mut particles = Vec::new();
    let re = Regex::new(r"-?\d+").unwrap();

    for line in input.lines() {
        let n: Vec<_> = re
            .find_iter(line)
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();

        particles.push(Particle {
            position: [n[0], n[1], n[2]],
            velocity: [n[3], n[4], n[5]],
            acceleration: [n[6], n[7], n[8]],
        })
    }

    particles
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut particles = parse(input);

        // calculate this algebraically
        let n = 1_000;
        let mut closest = i64::MAX;
        let mut closest_i = 0;

        for (i, particle) in particles.iter_mut().enumerate() {
            for j in 0..particle.acceleration.len() {
                particle.velocity[j] += n * particle.acceleration[j];
                particle.position[j] +=
                    n * particle.velocity[j] - (n * (n - 1) / 2) * particle.acceleration[j];
            }

            let mut distance = 0;

            for j in 0..particle.acceleration.len() {
                distance += particle.position[j].abs();
            }

            if closest > distance {
                closest_i = i;
                closest = distance;
            }
        }

        Option::from(closest_i.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut particles = parse(input);
        let mut collided = vec![false; particles.len()];

        let n = 1_000;

        for _ in 0..n {
            for i in 0..particles.len() {
                if collided[i] {
                    continue;
                }

                'outer: for j in i + 1..particles.len() {
                    if collided[j] {
                        continue;
                    }

                    for k in 0..particles[i].position.len() {
                        if particles[i].position[k] != particles[j].position[k] {
                            break 'outer;
                        }
                    }

                    collided[i] = true;
                    collided[j] = true;
                }
            }

            for particle in particles.iter_mut() {
                for k in 0..particle.position.len() {
                    particle.velocity[k] += particle.acceleration[k];
                    particle.position[k] += particle.velocity[k];
                }
            }
        }

        Option::from(collided.iter().filter(|&&v| !v).count().to_string())
    }
}
