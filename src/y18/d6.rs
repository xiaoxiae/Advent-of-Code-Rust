use crate::util::Day;
use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

pub struct D6;

impl Day for D6 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let points: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let (x, y) = line.split(", ").collect_tuple().unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let max_x = points.iter().map(|p| p.0).max().unwrap();
        let max_y = points.iter().map(|p| p.1).max().unwrap();

        let mut point_frequency = vec![0; points.len()];
        let mut infinite_ids = HashSet::default();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let mut min_dist = i32::MAX;
                let mut min_dist_id = None;

                for (idx, &(px, py)) in points.iter().enumerate() {
                    let dist = (x - px).abs() + (y - py).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        min_dist_id = Some(idx);
                    } else if dist == min_dist {
                        min_dist_id = None;
                    }
                }

                if let Some(id) = min_dist_id {
                    if x == 0 || y == 0 || x == max_x || y == max_y {
                        infinite_ids.insert(id);
                    } else if !infinite_ids.contains(&id) {
                        point_frequency[id] += 1;
                    }
                }
            }
        }

        let max_area = point_frequency
            .iter()
            .enumerate()
            .filter(|(i, _)| !infinite_ids.contains(i))
            .map(|(_, &count)| count)
            .max();

        max_area.map(|v| v.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let points: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let (x, y) = line.split(", ").collect_tuple().unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let max_x = points.iter().map(|p| p.0).max().unwrap();
        let max_y = points.iter().map(|p| p.1).max().unwrap();

        let mut total = 0;
        for x in 0..=max_x {
            for y in 0..=max_y {
                let distance_sum: i32 = points.iter().map(|&(px, py)| (x - px).abs() + (y - py).abs()).sum();
                if distance_sum < 10000 {
                    total += 1;
                }
            }
        }

        Some(total.to_string())
    }
}
