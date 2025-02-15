use std::collections::HashSet;
use crate::util::Day;

pub struct D8;

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let trees: Vec<Vec<i32>> = input.lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(|d| d as i32)).collect())
            .collect();

        let h = trees.len();
        let w = trees[0].len();
        let mut visible = HashSet::new();

        for y in 0..h {
            for dir in [Box::new(0..w) as Box<dyn Iterator<Item = usize>>, Box::new((0..w).rev())] {
                let mut highest = i32::MIN;
                for x in dir {
                    if trees[y][x] > highest {
                        visible.insert((x, y));
                        highest = trees[y][x];
                    }
                }
            }
        }

        for x in 0..w {
            for dir in [Box::new(0..h) as Box<dyn Iterator<Item = usize>>, Box::new((0..h).rev())] {
                let mut highest = i32::MIN;
                for y in dir {
                    if trees[y][x] > highest {
                        visible.insert((x, y));
                        highest = trees[y][x];
                    }
                }
            }
        }

        Option::from(visible.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let trees: Vec<Vec<i32>> = input.lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(|d| d as i32)).collect())
            .collect();

        let w = trees[0].len();
        let h = trees.len();
        let mut max_scenic_score = 0;

        for y in 0..h {
            for x in 0..w {
                let mut visible = vec![];

                for &(dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                    let (mut nx, mut ny) = (x as isize, y as isize);

                    visible.push(0);

                    loop {
                        nx += dx;
                        ny += dy;

                        if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
                            break;
                        }

                        let n = visible.len();
                        visible[n - 1] += 1;

                        if trees[y][x] <= trees[ny as usize][nx as usize] {
                            break;
                        }
                    }
                }

                let scenic_score = visible.iter().product::<i32>();
                max_scenic_score = max_scenic_score.max(scenic_score);
            }
        }

        Option::from(max_scenic_score.to_string())
    }
}
