use crate::util::Day;

pub struct D4;

fn eat(src: &Vec<Vec<bool>>, dst: &mut Vec<Vec<bool>>, h: usize, w: usize) -> usize {
    let mut eaten = 0;

    for y in 1..=h {
        for x in 1..=w {
            if !src[y][x] {
                dst[y][x] = false;
                continue;
            }

            let neighbours = src[y - 1][x - 1] as usize
                + src[y - 1][x] as usize
                + src[y - 1][x + 1] as usize
                + src[y][x - 1] as usize
                + src[y][x + 1] as usize
                + src[y + 1][x - 1] as usize
                + src[y + 1][x] as usize
                + src[y + 1][x + 1] as usize;

            if neighbours >= 4 {
                dst[y][x] = true;
            } else {
                dst[y][x] = false;
                eaten += 1;
            }
        }
    }

    eaten
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();
        let h = lines.len();
        let w = lines[0].len();

        let mut a = vec![vec![false; w + 2]; h + 2];
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                a[y + 1][x + 1] = c == '@';
            }
        }
        let mut b = vec![vec![false; w + 2]; h + 2];

        let eaten = eat(&a, &mut b, h, w);

        Option::from(eaten.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();
        let h = lines.len();
        let w = lines[0].len();

        let mut a = vec![vec![false; w + 2]; h + 2];
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                a[y + 1][x + 1] = c == '@';
            }
        }
        let mut b = vec![vec![false; w + 2]; h + 2];

        let mut total_eaten = 0;

        loop {
            let eaten = eat(&a, &mut b, h, w);
            std::mem::swap(&mut a, &mut b);
            total_eaten += eaten;

            if eaten == 0 {
                break;
            }
        }

        Option::from(total_eaten.to_string())
    }
}
