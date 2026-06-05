//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/10
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub struct D10;

fn connection_types(c: char) -> &'static [(i32, i32)] {
    match c {
        '-' => &[(-1, 0), (1, 0)],
        '|' => &[(0, 1), (0, -1)],
        'L' => &[(0, -1), (1, 0)],
        'J' => &[(0, -1), (-1, 0)],
        'F' => &[(0, 1), (1, 0)],
        '7' => &[(0, 1), (-1, 0)],
        'S' => &[(1, 0), (0, 1), (-1, 0), (0, -1)],
        _ => &[],
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect())
        .filter(|l: &Vec<char>| !l.is_empty())
        .collect()
}

fn build_graph(
    grid: &[Vec<char>],
) -> (
    FxHashMap<(i32, i32), Vec<(i32, i32)>>,
    (i32, i32),
) {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut graph: FxHashMap<(i32, i32), Vec<(i32, i32)>> = FxHashMap::default();
    let mut start = (0, 0);

    for y in 0..height {
        for x in 0..width {
            let c = grid[y as usize][x as usize];
            if c == '.' {
                continue;
            }

            if c == 'S' {
                start = (x, y);
            }

            for &(dx, dy) in connection_types(c) {
                let nx = x + dx;
                let ny = y + dy;

                if !(0 <= nx && nx < width && 0 <= ny && ny < height) {
                    continue;
                }

                let nc = grid[ny as usize][nx as usize];
                if nc == '.' {
                    continue;
                }

                // matching connection
                if !connection_types(nc).contains(&(-dx, -dy)) {
                    continue;
                }

                graph.entry((x, y)).or_default().push((nx, ny));
            }
        }
    }

    // strip non-loop nodes
    loop {
        let mut deleted = false;

        let keys: Vec<(i32, i32)> = graph.keys().copied().collect();
        for k in keys {
            if graph.get(&k).is_some_and(|v| v.len() == 1) {
                deleted = true;
                graph.remove(&k);
            }
        }

        if !deleted {
            break;
        }
    }

    (graph, start)
}

fn bfs(
    graph: &FxHashMap<(i32, i32), Vec<(i32, i32)>>,
    start: (i32, i32),
) -> (i32, FxHashSet<(i32, i32)>) {
    let mut queue: VecDeque<(i32, (i32, i32))> = VecDeque::new();
    queue.push_back((0, start));
    let mut explored: FxHashMap<(i32, i32), i32> = FxHashMap::default();
    explored.insert(start, 0);
    let mut max_distance = 0;

    while let Some((d, current)) = queue.pop_front() {
        max_distance = max_distance.max(d);

        if let Some(neighbours) = graph.get(&current) {
            for &neighbour in neighbours {
                if explored.contains_key(&neighbour) {
                    continue;
                }
                explored.insert(neighbour, d + 1);
                queue.push_back((d + 1, neighbour));
            }
        }
    }

    let explored_set: FxHashSet<(i32, i32)> = explored.keys().copied().collect();
    (max_distance, explored_set)
}

impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let (graph, start) = build_graph(&grid);
        let (max_distance, _) = bfs(&graph, start);
        Some(max_distance.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);
        let height = grid.len();
        let width = grid[0].len();
        let (graph, start) = build_graph(&grid);
        let (_, explored) = bfs(&graph, start);

        // count the crossing numbers of the pipes to determine inside/outside
        let mut tiles = 0i64;
        for y in 0..height {
            let mut crossing_number = 0;
            let mut last_complex: Option<char> = None;

            for x in 0..width {
                let mut c = grid[y][x];

                // we need to know the type of S to count correctly
                if c == 'S' {
                    let deltas: FxHashSet<(i32, i32)> = graph
                        .get(&(x as i32, y as i32))
                        .map(|v| {
                            v.iter()
                                .map(|&(nx, ny)| (nx - x as i32, ny - y as i32))
                                .collect()
                        })
                        .unwrap_or_default();

                    for &t in &['-', '|', 'L', 'J', 'F', '7', 'S'] {
                        let set: FxHashSet<(i32, i32)> =
                            connection_types(t).iter().copied().collect();
                        if deltas == set {
                            c = t;
                            break;
                        }
                    }
                }

                if explored.contains(&(x as i32, y as i32)) {
                    // always increases for |
                    if c == '|' {
                        crossing_number += 1;
                    }
                    // start of vertical pipe
                    else if c == 'F' || c == 'L' {
                        last_complex = Some(c);
                    }
                    // end of vertical pipe
                    else if c == '7' || c == 'J' {
                        if (last_complex == Some('F') && c == 'J')
                            || (last_complex == Some('L') && c == '7')
                        {
                            crossing_number += 1;
                        }
                    }
                } else if crossing_number % 2 == 1 {
                    tiles += 1;
                }
            }
        }

        Some(tiles.to_string())
    }
}
