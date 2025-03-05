use crate::util::Day;

pub struct D21;

fn parse_pattern(pattern: &str) -> usize {
    pattern
        .chars()
        .filter(|&c| c != '/')
        .rev()
        .fold(0, |acc, c| (acc << 1) | (c == '#') as usize)
}

fn rotate_pattern(pattern: usize, size: usize) -> usize {
    let mut rotated = 0;

    for y in 0..size {
        for x in 0..size {
            let bit = (pattern >> ((size - 1 - y) * size + x)) & 1;
            let new_pos = (x * size) + y;
            rotated |= bit << new_pos;
        }
    }

    rotated
}

fn flip_pattern(pattern: usize, size: usize) -> usize {
    let mut flipped = 0;

    for y in 0..size {
        let col = (pattern >> (y * size)) & ((1 << size) - 1);
        let new_pos = (size - 1 - y) * size;
        flipped |= col << new_pos;
    }

    flipped
}

fn from_map(map: &Vec<Vec<bool>>, dx: usize, dy: usize, size: usize) -> usize {
    let mut pattern = 0;

    for y in (0..size).rev() {
        for x in (0..size).rev() {
            pattern <<= 1;

            if map[y + dy][x + dx] {
                pattern |= 1;
            }
        }
    }

    pattern
}

fn to_map(pattern: usize, size: usize, map: &mut Vec<Vec<bool>>, dx: usize, dy: usize) {
    for y in 0..size {
        for x in 0..size {
            if (pattern >> (x + y * size)) & 1 == 1 {
                map[y + dy][x + dx] = true;
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![
        vec![],
        vec![],
        vec![0; 2usize.pow(4)],
        vec![0; 2usize.pow(9)],
    ];

    for line in input.lines() {
        let parts = line.split(" => ").collect::<Vec<_>>();

        let from = parts[0];
        let to = parts[1];

        // a little ew but we know the input data
        let size;
        if from.len() == 5 {
            size = 2;
        } else {
            size = 3;
        }

        let mut from_pattern = parse_pattern(from);
        let to_pattern = parse_pattern(to);
        for _ in 0..4 {
            result[size][from_pattern] = to_pattern;
            from_pattern = rotate_pattern(from_pattern, size);
        }

        from_pattern = flip_pattern(from_pattern, size);

        for _ in 0..4 {
            result[size][from_pattern] = to_pattern;
            from_pattern = rotate_pattern(from_pattern, size);
        }
    }

    result
}

fn solve(input: &str, iterations: usize) -> usize {
    let patterns = parse(input);

    let mut width = 3;
    let mut map = vec![vec![false; width]; width];
    to_map(parse_pattern(".#./..#/###"), width, &mut map, 0, 0);

    for iteration in 0..iterations {
        let size = match width % 2 {
            0 => 2,
            _ => 3,
        };

        let new_width = width + (width / size);
        let mut new_map = vec![vec![false; new_width]; new_width];

        for dx in 0..(width / size) {
            for dy in 0..(width / size) {
                let mut pattern = from_map(&map, dx * size, dy * size, size);
                pattern = patterns[size][pattern];

                to_map(
                    pattern,
                    size + 1,
                    &mut new_map,
                    dx * (size + 1),
                    dy * (size + 1),
                );
            }
        }

        map = new_map;
        width = new_width;
    }

    map.iter().flatten().filter(|&&b| b).count()
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 5).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 18).to_string())
    }
}
