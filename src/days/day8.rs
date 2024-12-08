use crate::util::Day;
use std::collections::HashMap;

pub struct Day8;

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>) {
    let map = input
        .trim()
        .split_terminator("\n")
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut coordinates: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            match char {
                '.' => continue,
                other => coordinates.entry(*other).or_insert(Vec::new()).push((x, y)),
            }
        }
    }

    (map, coordinates)
}

fn create_antinodes(
    map: &mut Vec<Vec<char>>,
    coordinates: &HashMap<char, Vec<(usize, usize)>>,
    min_jumps: usize,
    max_jumps: usize,
) {
    for (_, coordinates) in coordinates {
        for p1 in coordinates {
            for p2 in coordinates {
                if p1 == p2 {
                    continue;
                }

                let (dx, dy) = (p1.0 as i32 - p2.0 as i32, p1.1 as i32 - p2.1 as i32);

                let mut i = min_jumps as i32;
                loop {
                    let (x, y) = (p1.0 as i32 + i * dx, p1.1 as i32 + i * dy);

                    match map.get(y as usize).and_then(|row| row.get(x as usize)) {
                        Some(_) => {
                            map[y as usize][x as usize] = '#';
                            i += 1;
                        }
                        _ => break,
                    }

                    if i >= max_jumps as i32 {
                        break;
                    }
                }
            }
        }
    }
}

impl Day for Day8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut map, coordinates) = parse_input(input);

        create_antinodes(&mut map, &coordinates, 1, 1);

        Option::from(map.iter()
            .flatten()
            .filter(|x| **x == '#')
            .count()
            .to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut map, coordinates) = parse_input(input);

        create_antinodes(&mut map, &coordinates, 0, usize::MAX);

        Option::from(map.iter()
            .flatten()
            .filter(|x| **x == '#')
            .count()
            .to_string())
    }
}
