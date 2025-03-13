use crate::util::Day;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

pub struct D15;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Unit {
        health: isize,
        attack: isize,
        good: bool,
    },
    Wall,
    Empty,
}

fn find_lowest_health_target(
    map: &Vec<Vec<Tile>>,
    (x, y): (isize, isize),
    source_good: bool,
) -> Option<(isize, isize)> {
    let mut lowest = isize::MAX;
    let mut lowest_position = None;

    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let nx = x + dx;
        let ny = y + dy;

        match map[ny as usize][nx as usize] {
            Tile::Unit { good, health, .. } if good != source_good => {
                if lowest > health {
                    lowest = health;
                    lowest_position.replace((nx, ny));
                }
            }
            _ => continue,
        }
    }

    lowest_position
}

fn find_closest_targets(
    map: &Vec<Vec<Tile>>,
    start: (isize, isize),
    source_good: bool,
) -> Vec<(isize, isize)> {
    let mut queue = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited: HashSet<(isize, isize)> = HashSet::default();
    visited.insert(start);

    let mut closest_targets = vec![];
    let mut closest = isize::MAX;

    while !queue.is_empty() {
        let (x, y, distance) = queue.pop_front().unwrap();

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;

            if visited.contains(&(nx, ny)) {
                continue;
            }

            visited.insert((nx, ny));

            match &map[ny as usize][nx as usize] {
                Tile::Empty => {
                    queue.push_back((nx, ny, distance + 1));
                }
                Tile::Unit { good, .. } if *good != source_good => {
                    if distance < closest {
                        closest = distance;
                        closest_targets = vec![(nx, ny)];
                    } else if distance == closest {
                        closest_targets.push((nx, ny));
                    }
                }
                _ => continue,
            }
        }
    }

    closest_targets
}

fn find_closest_target_direction(
    map: &Vec<Vec<Tile>>,
    start: (isize, isize),
    source_good: bool,
) -> Option<(isize, isize)> {
    let mut closest_targets = find_closest_targets(map, start, source_good);

    if closest_targets.is_empty() {
        return None;
    }

    closest_targets.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

    let target = closest_targets[0];

    let mut queue = VecDeque::from([start]);
    let mut visited: HashMap<(isize, isize), (isize, isize)> = HashMap::default();
    visited.insert(start, start);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let mut nx = x + dx;
            let mut ny = y + dy;

            if visited.contains_key(&(nx, ny)) {
                continue;
            }

            visited.insert((nx, ny), (x, y));

            match &map[ny as usize][nx as usize] {
                Tile::Empty => {
                    queue.push_back((nx, ny));
                }
                _ => {
                    if (nx, ny) == target {
                        while visited.get(&(nx, ny)).unwrap() != &start {
                            (nx, ny) = *visited.get(&(nx, ny)).unwrap();
                        }

                        return Some((nx - start.0, ny - start.1));
                    }
                }
            }
        }
    }

    unreachable!()
}

fn solve(input: &str, elf_power: isize) -> (isize, isize) {
    let mut map = vec![];

    let mut initial_elves = 0;
    let mut counts = (0, 0);

    for line in input.lines() {
        let mut row: Vec<Tile> = vec![];

        for char in line.chars() {
            row.push(match char {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'E' => {
                    counts.0 += 1;
                    initial_elves += 1;

                    Tile::Unit {
                        health: 200,
                        attack: elf_power,
                        good: true,
                    }
                }
                'G' => {
                    counts.1 += 1;

                    Tile::Unit {
                        health: 200,
                        attack: 3,
                        good: false,
                    }
                }
                _ => unreachable!(),
            })
        }

        map.push(row);
    }

    let mut round = 0;
    'outer: loop {
        let mut units = vec![];

        let mut moved = false;

        for (y, line) in map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                match tile {
                    Tile::Unit { .. } => units.push((x as isize, y as isize)),
                    _ => {}
                }
            }
        }

        for (mut x, mut y) in units {
            let unit = &map[y as usize][x as usize];

            // if the unit already died, we skip it
            let current_good;
            let current_attack;
            match unit {
                Tile::Wall | Tile::Empty => continue,
                Tile::Unit { good, attack, .. } => {
                    current_attack = *attack;
                    current_good = *good
                }
            }

            // if the unit has no targets, the combat ended
            if counts.0 == 0 || counts.1 == 0 {
                break 'outer;
            }

            // move, if not already in range
            if let Some((dx, dy)) = find_closest_target_direction(&map, (x, y), current_good) {
                moved = true;

                let nx = x + dx;
                let ny = y + dy;

                match &map[ny as usize][nx as usize] {
                    Tile::Unit { .. } => {} // yay we can reach!
                    Tile::Empty => {
                        map[ny as usize][nx as usize] = unit.clone();
                        map[y as usize][x as usize] = Tile::Empty;

                        x += dx;
                        y += dy;
                    }
                    _ => unreachable!(),
                }
            }

            // attack, if in range
            if let Some((nx, ny)) = find_lowest_health_target(&map, (x, y), current_good) {
                match map[ny as usize][nx as usize] {
                    Tile::Unit {
                        good,
                        health,
                        attack,
                    } => {
                        // death
                        if (health - current_attack) <= 0 {
                            map[ny as usize][nx as usize] = Tile::Empty;

                            if good {
                                counts.0 -= 1;
                            } else {
                                counts.1 -= 1;
                            }
                        } else {
                            map[ny as usize][nx as usize] = Tile::Unit {
                                good,
                                health: health - current_attack,
                                attack,
                            };
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        // after no moves have been made, terminate
        if !moved {
            break;
        }

        round += 1;
    }

    let mut health = 0;
    for row in map {
        for item in row {
            health += match item {
                Tile::Unit { health, .. } => health,
                _ => 0,
            }
        }
    }

    (health * round, initial_elves - counts.0)
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 3).0.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut min = 4;
        let mut max = 4;

        while solve(input, max).1 > 0 {
            min = max;
            max *= 2;
        }

        let mut result = None;

        while min <= max {
            let mid = (min + max) / 2;
            let current_result = solve(input, mid);

            if current_result.1 == 0 {
                result = Some(current_result.0.to_string());
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }

        result
    }
}
