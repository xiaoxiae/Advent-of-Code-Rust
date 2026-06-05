//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/12
use crate::util::Day;
use std::collections::VecDeque;

pub struct D12;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().collect())
        .collect()
}

fn at(scan: &[Vec<u8>], x: usize, y: usize) -> i32 {
    let c = scan[y][x];
    let val = match c {
        b'S' => b'a',
        b'E' => b'z',
        other => other,
    };
    val as i32 - 97
}

fn neighbours(scan: &[Vec<u8>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let h = scan.len() as i32;
    let w = scan[0].len() as i32;
    let mut n = Vec::new();
    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < w && ny >= 0 && ny < h {
            let (nxu, nyu) = (nx as usize, ny as usize);
            if at(scan, x, y) + 1 >= at(scan, nxu, nyu) {
                n.push((nxu, nyu));
            }
        }
    }
    n
}

fn bfs(scan: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let h = scan.len();
    let w = scan[0].len();
    let mut visited = vec![vec![-1i32; w]; h];
    let mut queue = VecDeque::new();
    visited[start.1][start.0] = 0;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let cur_dist = visited[current.1][current.0];
        for (nx, ny) in neighbours(scan, current.0, current.1) {
            if visited[ny][nx] == -1 {
                visited[ny][nx] = cur_dist + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    let v = visited[end.1][end.0];
    (v != -1).then_some(v)
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let scan = parse(input);
        let mut start = (0, 0);
        let mut end = (0, 0);
        for y in 0..scan.len() {
            for x in 0..scan[0].len() {
                if scan[y][x] == b'S' {
                    start = (x, y);
                }
                if scan[y][x] == b'E' {
                    end = (x, y);
                }
            }
        }
        let answer = bfs(&scan, start, end).unwrap();
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let scan = parse(input);
        let mut end = (0, 0);
        for y in 0..scan.len() {
            for x in 0..scan[0].len() {
                if scan[y][x] == b'E' {
                    end = (x, y);
                }
            }
        }

        let mut best = i32::MAX;
        for y in 0..scan.len() {
            for x in 0..scan[0].len() {
                let start = (x, y);
                if at(&scan, x, y) != 0 {
                    continue;
                }
                if let Some(d) = bfs(&scan, start, end) {
                    best = best.min(d);
                }
            }
        }
        Some(best.to_string())
    }
}
