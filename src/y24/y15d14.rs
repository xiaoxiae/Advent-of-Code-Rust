use crate::util::Day;
use regex::Regex;

pub struct Y15D14;

static FLIGHT_TIME: usize = 2503;

#[derive(Debug)]
struct Reindeer {
    name: String,
    distance: usize,
    flight_time: usize,
    pause_time: usize,
}

fn parse(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(r"(?P<name>\w+) can fly (?P<distance>\d+) km/s for (?P<flight_time>\d+) seconds, but then must rest for (?P<pause>\d+) seconds.").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Reindeer {
                name: captures["name"].parse().unwrap(),
                distance: captures["distance"].parse::<usize>().unwrap(),
                flight_time: captures["flight_time"].parse::<usize>().unwrap(),
                pause_time: captures["pause"].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Reindeer>>()
}

fn solve(reindeer: Vec<Reindeer>) -> (Vec<usize>, Vec<usize>) {
    let mut distances: Vec<_> = vec![0; reindeer.len()];
    let mut scores: Vec<_> = vec![0; reindeer.len()];

    for time in 0..FLIGHT_TIME {
        for (i, r) in reindeer.iter().enumerate() {
            if time % (r.flight_time + r.pause_time) < r.flight_time {
                distances[i] += r.distance;
            }
        }

        let max = *distances.iter().max().unwrap();

        for i in 0..reindeer.len() {
            if max == distances[i] {
                scores[i] += 1;
            }
        }
    }

    (distances, scores)
}

impl Day for Y15D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let reindeer = parse(input);
        
        let (distances, _) = solve(reindeer);

        Option::from(distances.iter().max().unwrap().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let reindeer = parse(input);

        let (_, scores) = solve(reindeer);

        Option::from(scores.iter().max().unwrap().to_string())
    }
}
