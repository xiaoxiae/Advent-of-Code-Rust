use crate::util::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct D23;

struct Robot {
    x: isize,
    y: isize,
    z: isize,
    r: usize,
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<isize> = line
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter_map(|s| s.parse().ok())
                .collect();

            Robot {
                x: nums[0],
                y: nums[1],
                z: nums[2],
                r: nums[3] as usize,
            }
        })
        .collect::<Vec<Robot>>()
}

fn neighbouring(r1: &Robot, r2: &Robot) -> bool {
    let distance = r1.x.abs_diff(r2.x) + r1.y.abs_diff(r2.y) + r1.z.abs_diff(r2.z);
    distance <= r1.r + r2.r
}

fn reachable(r1: &Robot, r2: &Robot) -> bool {
    let distance = r1.x.abs_diff(r2.x) + r1.y.abs_diff(r2.y) + r1.z.abs_diff(r2.z);
    distance <= r2.r
}

fn get_largest_clique(
    clique: Vec<usize>,
    remaining: &Vec<usize>,
    graph: &HashMap<usize, HashSet<usize>>,
) -> Vec<usize> {
    if remaining.len() == 0 {
        return clique;
    }

    let u = remaining[0];
    let u_neighbours = graph.get(&u).unwrap();

    let mut valid = true;
    for &v in &clique {
        if !u_neighbours.contains(&v) {
            valid = false;
            break;
        }
    }

    let mut o1 = vec![];
    if valid {
        let mut new = clique.clone();
        new.push(u);

        let new_remaining: Vec<_> = remaining
            .iter()
            .filter(|&r| u_neighbours.contains(&r))
            .map(|&v| v)
            .collect();

        o1 = get_largest_clique(new, &new_remaining, graph);
    }

    let o2 = get_largest_clique(clique, &remaining[1..].to_vec(), graph);

    std::cmp::max_by_key(o1, o2, |o| o.len())
}

fn approximate_max_clique(graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    let mut clique = HashSet::new();
    let mut candidate_vertices: Vec<_> = graph.keys().collect();

    candidate_vertices.sort_by_key(|&v| std::cmp::Reverse(graph[v].len()));

    for &v in candidate_vertices {
        if clique.iter().all(|&u| graph[&u].contains(&v)) {
            clique.insert(v);
        }
    }

    clique
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bots = parse(input);

        let strongest_bot = bots.iter().max_by(|a, b| a.r.cmp(&b.r)).unwrap();

        let total = bots
            .iter()
            .filter(|bot| reachable(bot, strongest_bot))
            .count();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bots = parse(input);

        let mut graph = HashMap::new();

        for (i, b1) in bots.iter().enumerate() {
            let mut neighbours = HashSet::new();

            for (j, b2) in bots.iter().enumerate() {
                if i == j {
                    continue;
                }

                if neighbouring(b1, b2) {
                    neighbours.insert(j);
                }
            }

            graph.insert(i, neighbours);
        }

        let clique: Vec<_> = approximate_max_clique(&graph).iter().map(|&v| v).collect();
        
        // to get the closest point, we want to find the most restrictive half-plane,
        // as all points on it will be the same
        let mut closest_half_plane = isize::MAX;
        for i in clique {
            let bot = &bots[i];
            
            closest_half_plane = closest_half_plane.min(bot.x + bot.y + bot.z + bot.r as isize);
        }
        
        Option::from(closest_half_plane.to_string())
    }
}
