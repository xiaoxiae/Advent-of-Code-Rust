//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/20
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D20;

fn direction_to_step(c: char) -> (i64, i64) {
    match c {
        'E' => (1, 0),
        'N' => (0, -1),
        'S' => (0, 1),
        'W' => (-1, 0),
        _ => unreachable!(),
    }
}

/// Build the move graph from the regex (without surrounding ^ and $).
fn build_move_dict(regex: &str) -> FxHashMap<(i64, i64), Vec<(i64, i64)>> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut coordinate_stack: Vec<(i64, i64)> = Vec::new();
    let mut move_dict: FxHashMap<(i64, i64), Vec<(i64, i64)>> = FxHashMap::default();

    for char in regex.chars() {
        match char {
            'E' | 'N' | 'S' | 'W' => {
                let direction = direction_to_step(char);
                let start = (x, y);
                x += direction.0;
                y += direction.1;
                let end = (x, y);
                move_dict.entry(start).or_default().push(end);
            }
            '(' => {
                coordinate_stack.push((x, y));
            }
            '|' => {
                let coordinate = *coordinate_stack.last().unwrap();
                x = coordinate.0;
                y = coordinate.1;
            }
            ')' => {
                let coordinate = coordinate_stack.pop().unwrap();
                x = coordinate.0;
                y = coordinate.1;
            }
            _ => {}
        }
    }

    move_dict
}

/// Run the DFS used by both parts; returns the `explored` distance map.
fn explore(move_dict: &FxHashMap<(i64, i64), Vec<(i64, i64)>>) -> FxHashMap<(i64, i64), i64> {
    let mut stack: Vec<(i64, i64)> = vec![(0, 0)];
    let mut explored: FxHashMap<(i64, i64), i64> = FxHashMap::default();
    explored.insert((0, 0), 0);

    while let Some(coord) = stack.pop() {
        if let Some(neighbours) = move_dict.get(&coord) {
            let coord_dist = explored[&coord];
            for &neighbour in neighbours {
                if !explored.contains_key(&neighbour) {
                    explored.insert(neighbour, coord_dist + 1);
                    stack.push(neighbour);
                }
            }
        }
    }

    explored
}

fn regex_body(input: &str) -> &str {
    let line = input.lines().next().unwrap();
    // strip leading ^ and trailing $ (Python: [1:-1])
    &line[1..line.len() - 1]
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let regex = regex_body(input);
        let move_dict = build_move_dict(regex);
        let explored = explore(&move_dict);
        let answer = explored.values().copied().max().unwrap();
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let regex = regex_body(input);
        let move_dict = build_move_dict(regex);
        let explored = explore(&move_dict);
        let answer = explored.values().filter(|&&v| v >= 1000).count();
        Some(answer.to_string())
    }
}
