use crate::util::Day;
use std::collections::HashSet;

pub struct D9;

fn distance(head: (i32, i32), tail: (i32, i32)) -> i32 {
    (head.0 - tail.0).abs().max((head.1 - tail.1).abs())
}

fn move_towards(a: i32, b: i32) -> i32 {
    if a < b { 1 } else if a > b { -1 } else { 0 }
}

fn move_tail(head: (i32, i32), tail: &mut (i32, i32)) {
    if distance(head, *tail) > 1 {
        tail.0 += move_towards(tail.0, head.0);
        tail.1 += move_towards(tail.1, head.1);
    }
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut visited = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);
        visited.insert(tail);

        let deltas = [("R", (1, 0)), ("L", (-1, 0)), ("U", (0, 1)), ("D", (0, -1))]
            .into_iter().collect::<std::collections::HashMap<_, _>>();

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let dir = parts.next()?;
            let count: i32 = parts.next()?.parse().ok()?;
            let (dx, dy) = deltas[dir];

            for _ in 0..count {
                head.0 += dx;
                head.1 += dy;
                move_tail(head, &mut tail);
                visited.insert(tail);
            }
        }
        Option::from(visited.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut visited = HashSet::new();
        let mut tails = vec![(0, 0); 10];
        visited.insert(tails[9]);

        let deltas = [("R", (1, 0)), ("L", (-1, 0)), ("U", (0, 1)), ("D", (0, -1))]
            .into_iter().collect::<std::collections::HashMap<_, _>>();

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let dir = parts.next()?;
            let count: i32 = parts.next()?.parse().ok()?;
            let (dx, dy) = deltas[dir];

            for _ in 0..count {
                tails[0].0 += dx;
                tails[0].1 += dy;
                for i in 0..9 {
                    let head = tails[i];
                    let tail = &mut tails[i + 1];
                    move_tail(head, tail);
                }
                visited.insert(tails[9]);
            }
        }
        Option::from(visited.len().to_string())
    }
}
