use crate::util::Day;

pub struct D18;

static STEPS: usize = 100;

fn simulate(map: &mut Vec<Vec<bool>>, steps: usize, corners: bool) {
    for _ in 0..steps {
        if corners {
            let w = map[0].len();
            let h = map.len();

            map[0][0] = true;
            map[0][w - 1] = true;
            map[h - 1][0] = true;
            map[h - 1][w - 1] = true;
        }

        let mut original_map = map.clone();

        for y in 0..original_map.len() {
            for x in 0..original_map[0].len() {
                let mut neighbours = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if original_map
                            .get(ny as usize)
                            .and_then(|row| row.get(nx as usize))
                            == Some(&true)
                        {
                            neighbours += 1;
                        }
                    }
                }

                match original_map[y][x] {
                    true if neighbours == 2 || neighbours == 3 => map[y][x] = true,
                    false if neighbours == 3 => map[y][x] = true,
                    _ => map[y][x] = false,
                }
            }
        }
    }

    if corners {
        let w = map[0].len();
        let h = map.len();

        map[0][0] = true;
        map[0][w - 1] = true;
        map[h - 1][0] = true;
        map[h - 1][w - 1] = true;
    }
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

impl Day for D18 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut map = parse(input);

        simulate(&mut map, STEPS, false);

        let on = map
            .iter()
            .map(|row| row.iter().filter(|&&c| c).count())
            .sum::<usize>();

        Option::from(on.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut map = parse(input);

        simulate(&mut map, STEPS, true);

        let on = map
            .iter()
            .map(|row| row.iter().filter(|&&c| c).count())
            .sum::<usize>();

        Option::from(on.to_string())
    }
}
