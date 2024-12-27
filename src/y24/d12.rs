use crate::util::Day;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

pub struct D12;

/// Return the indexes of the positions of the region at (x, y)
fn get_region(map: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut deque = VecDeque::from([(x, y)]);

    let mut explored = HashSet::default();
    explored.insert((x, y));

    let char = map[y][x];

    while !deque.is_empty() {
        let (x, y) = deque.pop_front().unwrap();

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if explored.contains(&(nx, ny)) {
                continue;
            }

            match map.get(ny).and_then(|row| row.get(nx)) {
                Some(&c) if c == char => {
                    explored.insert((nx, ny));
                    deque.push_back((nx, ny));
                }
                _ => {}
            }
        }
    }

    explored
}


impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let map = input.trim().split_terminator('\n')
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut explored: HashSet<(usize, usize)> = HashSet::default();
        let mut total = 0;

        for (y, line) in map.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if explored.contains(&(x, y)) {
                    continue;
                }

                let region = get_region(&map, x, y);
                explored.extend(&region);

                let mut area = 0;

                // We count perimeter as follows:
                // - each piece of land adds 4 perimeter
                // - however, if there is an adjacent one to the left or top, we subtract 2
                //   (one for the left/top piece and one for itself)
                let mut perimeter = 0;
                for (rx, ry) in &region {
                    area += 1;
                    perimeter += 4;

                    for (dx, dy) in [(-1, 0), (0, -1)] {
                        let nx = (*rx as isize + dx) as usize;
                        let ny = (*ry as isize + dy) as usize;

                        match map.get(ny).and_then(|row| row.get(nx)) {
                            Some(c) if c == char => perimeter -= 2,
                            _ => {}
                        }
                    }
                }

                total += area * perimeter;
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let map = input.trim().split_terminator('\n')
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut explored: HashSet<(usize, usize)> = HashSet::default();
        let mut total = 0;

        for (y, line) in map.iter().enumerate() {
            for (x, _char) in line.iter().enumerate() {
                if explored.contains(&(x, y)) {
                    continue;
                }

                let region = get_region(&map, x, y);
                explored.extend(&region);

                let mut area = 0;

                // We count the sides as follows:
                // - 'positive' top-left and bottom-right corners add 2
                // |  .          a
                // | .Aa   or   aA.   (. has to be empty)
                // |  a          .
                //
                // - 'negative' top-right and bottom-left corners add 2
                // |             aA
                // | a.    or    .a   (. has to be empty)
                // | Aa
                let mut sides = 0;
                for &(rx, ry) in &region {
                    area += 1;

                    let neighbors = |dx: isize, dy: isize|
                        region.contains(&((rx as isize + dx) as usize, (ry as isize + dy) as usize));

                    // Positive corners
                    if !neighbors(-1, 0) && !neighbors(0, -1) {
                        sides += 2;
                    }

                    if !neighbors(1, 0) && !neighbors(0, 1) {
                        sides += 2;
                    }

                    // Negative corners
                    if neighbors(1, 0) && neighbors(0, -1) && !neighbors(1, -1) {
                        sides += 2;
                    }

                    if neighbors(-1, 0) && neighbors(0, 1) && !neighbors(-1, 1) {
                        sides += 2;
                    }
                }

                total += area * sides;
            }
        }

        Option::from(total.to_string())
    }
}
