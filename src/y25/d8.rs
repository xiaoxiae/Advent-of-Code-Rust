use std::collections::HashMap;

use itertools::Itertools;

use crate::util::Day;

pub struct D8;

#[derive(Debug, Clone, Copy)]
enum UFValue {
    None,
    Parent(usize),
    Root,
}

fn parent(idx: usize, union_find: &Vec<UFValue>) -> Option<usize> {
    match union_find[idx] {
        UFValue::Parent(i) => parent(i, union_find),
        UFValue::None => None,
        UFValue::Root => Some(idx),
    }
}

fn parse_and_sort(input: &str) -> (Vec<(usize, usize, usize)>, Vec<UFValue>, Vec<(usize, (usize, usize))>) {
    let mut coordinates = vec![];
    let mut union_find = vec![];

    for line in input.lines() {
        let parts = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize, usize)>()
            .unwrap();

        coordinates.push(parts);
        union_find.push(UFValue::None);
    }

    let mut pair_distance = (0..coordinates.len())
        .tuple_combinations()
        .map(|(i, j)| {
            let (x1, y1, z1) = coordinates[i];
            let (x2, y2, z2) = coordinates[j];

            let distance =
                x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2);

            (distance, (i, j))
        })
        .collect::<Vec<_>>();

    pair_distance.sort_unstable();

    (coordinates, union_find, pair_distance)
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (coordinates, mut union_find, pair_distance) = parse_and_sort(input);

        let mut added = 0;
        for (_distance, (i, j)) in pair_distance {
            let i_val = parent(i, &union_find);
            let j_val = parent(j, &union_find);

            match (i_val, j_val) {
                (None, None) => {
                    union_find[i] = UFValue::Root;
                    union_find[j] = UFValue::Parent(i);
                }
                (Some(i_parent), None) => {
                    union_find[j] = UFValue::Parent(i_parent);
                }
                (None, Some(j_parent)) => {
                    union_find[i] = UFValue::Parent(j_parent);
                }
                (Some(i_parent), Some(j_parent)) => {
                    if i_parent != j_parent {
                        union_find[j_parent] = UFValue::Parent(i_parent);
                    }
                }
            }
            added += 1;

            if (added == 1000) {
                break;
            }
        }

        let mut counts = HashMap::new();

        for i in 0..coordinates.len() {
            match parent(i, &union_find) {
                Some(i_val) => {
                    counts
                        .entry(i_val)
                        .and_modify(|count| *count += 1)
                        .or_insert(1 as usize);
                }
                None => continue,
            }
        }

        let mut values = counts.values().map(|&count| count).collect::<Vec<_>>();
        values.sort_unstable();

        Option::from(values.iter().rev().take(3).product::<usize>().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (coordinates, mut union_find, pair_distance) = parse_and_sort(input);

        let mut components = 0;
        let mut unconnected = coordinates.len();

        for (_distance, (i, j)) in pair_distance {
            let i_val = parent(i, &union_find);
            let j_val = parent(j, &union_find);

            match (i_val, j_val) {
                (None, None) => {
                    union_find[i] = UFValue::Root;
                    union_find[j] = UFValue::Parent(i);
                    components += 1;
                    unconnected -= 2;
                }
                (Some(i_parent), None) => {
                    union_find[j] = UFValue::Parent(i_parent);
                    unconnected -= 1;
                }
                (None, Some(j_parent)) => {
                    union_find[i] = UFValue::Parent(j_parent);
                    unconnected -= 1;
                }
                (Some(i_parent), Some(j_parent)) => {
                    if i_parent != j_parent {
                        union_find[j_parent] = UFValue::Parent(i_parent);
                        components -= 1;
                    }
                }
            }

            if unconnected == 0 && components == 1 {
                let (x1, _, _) = coordinates[i];
                let (x2, _, _) = coordinates[j];

                return Option::from((x1 * x2).to_string());
            }
        }

        unreachable!()
    }
}
