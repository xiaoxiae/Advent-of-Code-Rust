use crate::util::Day;

type Map = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = (isize, isize);
type Instructions = Vec<char>;

fn parse_input(input: &str) -> (Map, Position, Instructions) {
    let parts = input.trim().split("\n\n").collect::<Vec<&str>>();

    let mut map = parts[0]
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect::<Map>();

    let mut position: Position = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '@' => {
                    position = (x, y);
                    map[y][x] = '.';
                }
                _ => continue,
            }
        }
    }

    let instructions = parts[1]
        .split_whitespace()
        .flat_map(|l| l.chars())
        .collect::<Vec<char>>();

    (map, position, instructions)
}

/// Helper function for getting positions of a given barrel
fn get_barrel_positions((x, y): Position, ch: char) -> Vec<Position> {
    match ch {
        'O' => vec![(x, y)],
        ']' => vec![(x, y), ((x as isize - 1) as usize, y)],
        '[' => vec![(x, y), ((x as isize + 1) as usize, y)],
        _ => panic!("Unknown barrel character {}", ch),
    }
}

fn can_push_barrels(map: &Map, (x, y): Position, (dx, dy): Direction) -> bool {
    let nx = (x as isize + dx) as usize;
    let ny = (y as isize + dy) as usize;

    match map.get(y).and_then(|row| row.get(x)) {
        Some('.') => true,

        // Horizontal push and small barrels don't require recursion
        Some('O') => can_push_barrels(map, (nx, ny), (dx, dy)),
        Some('[') | Some(']') if dx != 0 => can_push_barrels(map, (nx, ny), (dx, dy)),

        // Vertical push recurses
        Some(ch @ '[' | ch @ ']') => get_barrel_positions((x, y), *ch).iter().all(|(x, y)| {
            can_push_barrels(
                map,
                ((*x as isize + dx) as usize, (*y as isize + dy) as usize),
                (dx, dy),
            )
        }),

        _ => false,
    }
}

/// Push a thing at the given coordinate
/// Only barrels move, everything else doesn't change
fn push_barrels(map: &mut Map, (x, y): Position, (dx, dy): Direction) {
    let push = |map: &mut Map, (x, y): Position, (dx, dy): Direction| {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        push_barrels(map, (nx, ny), (dx, dy));

        map[ny][nx] = map[y][x];
        map[y][x] = '.';
    };

    match map.get(y).and_then(|row| row.get(x)) {
        Some('O') => push(map, (x, y), (dx, dy)),

        // Horizontal push and small barrels don't require recursion
        Some('[') | Some(']') if dx != 0 => push(map, (x, y), (dx, dy)),

        // Vertical push recurses
        Some(ch @ '[' | ch @ ']') => {
            get_barrel_positions((x, y), *ch)
                .iter()
                .for_each(|&p| push(map, p, (dx, dy)));
        }

        _ => return,
    }
}

fn try_move(map: &mut Map, position: &mut Position, delta: Direction) {
    let nx = (position.0 as isize + delta.0) as usize;
    let ny = (position.1 as isize + delta.1) as usize;

    match map.get(ny).and_then(|row| row.get(nx)) {
        Some('.') => {
            position.0 = nx;
            position.1 = ny;
        }
        Some('O') | Some('[') | Some(']') => {
            if can_push_barrels(map, (nx, ny), delta) {
                push_barrels(map, (nx, ny), delta);

                position.0 = nx;
                position.1 = ny;
            }
        }
        _ => {}
    }
}

fn solve(map: &mut Map, position: &mut Position, instructions: Instructions) -> usize {
    instructions.iter().for_each(|c| {
        let delta = match c {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => panic!("Unknown instruction character '{}'", *c as u32),
        };

        try_move(map, position, delta);
    });

    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' || map[y][x] == '[' {
                total += 100 * y + x;
            }
        }
    }

    return total;
}

pub struct D15;

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut map, mut position, instructions) = parse_input(input);

        let total = solve(&mut map, &mut position, instructions);

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (map, mut position, instructions) = parse_input(input);

        let mut map: Map = map
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|c| {
                        match c {
                            '#' => "##",
                            '.' => "..",
                            'O' => "[]",
                            _ => panic!("Unknown instruction character '{}'", *c as u32),
                        }
                        .chars()
                    })
                    .collect::<Vec<char>>()
            })
            .collect();

        position.0 *= 2;

        let total = solve(&mut map, &mut position, instructions);

        Option::from(total.to_string())
    }
}
