//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/23
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D23;

const PROPOSITION_DIRECTIONS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn parse(input: &str) -> FxHashSet<(i64, i64)> {
    let mut elves = FxHashSet::default();
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }
    elves
}

fn adjacent_elves(elves: &FxHashSet<(i64, i64)>, x: i64, y: i64) -> usize {
    let dirs = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    dirs.iter()
        .filter(|(dx, dy)| elves.contains(&(x + dx, y + dy)))
        .count()
}

fn get_proposition(
    elves: &FxHashSet<(i64, i64)>,
    x: i64,
    y: i64,
    proposition_index: usize,
) -> (i64, i64) {
    for i in 0..PROPOSITION_DIRECTIONS.len() {
        let (dx, dy) = PROPOSITION_DIRECTIONS[(i + proposition_index) % PROPOSITION_DIRECTIONS.len()];

        let blocked = (-1..2).any(|j| {
            let nx = x + if dx == 0 { j } else { dx };
            let ny = y + if dy == 0 { j } else { dy };
            elves.contains(&(nx, ny))
        });
        if !blocked {
            return (x + dx, y + dy);
        }
    }

    // if there are elves everywhere, don't move
    (x, y)
}

/// Runs the simulation. If `max_rounds` is Some(n), runs at most n rounds and returns
/// the final elf set. If None, runs until no elf moves and returns the round number.
fn step(
    elves: &FxHashSet<(i64, i64)>,
    proposition_index: usize,
) -> (FxHashSet<(i64, i64)>, bool) {
    let mut do_nothing: Vec<(i64, i64)> = Vec::new();
    let mut moves: Vec<(i64, i64)> = Vec::new();

    for &elf in elves.iter() {
        if adjacent_elves(elves, elf.0, elf.1) == 0 {
            do_nothing.push(elf);
        } else {
            moves.push(elf);
        }
    }

    let no_move = moves.is_empty();

    let mut propositions: FxHashMap<(i64, i64), Vec<(i64, i64)>> = FxHashMap::default();
    for &elf in moves.iter() {
        let p = get_proposition(elves, elf.0, elf.1, proposition_index);
        propositions.entry(p).or_default().push(elf);
    }

    let mut new_elves: FxHashSet<(i64, i64)> = do_nothing.into_iter().collect();
    for (p, group) in propositions {
        if group.len() != 1 {
            for e in group {
                new_elves.insert(e);
            }
        } else {
            new_elves.insert(p);
        }
    }

    (new_elves, no_move)
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut elves = parse(input);
        let mut proposition_index = 0usize;

        for _ in 0..10 {
            let (new_elves, _) = step(&elves, proposition_index);
            elves = new_elves;
            proposition_index += 1;
        }

        let min_x = elves.iter().map(|e| e.0).min().unwrap();
        let min_y = elves.iter().map(|e| e.1).min().unwrap();
        let max_x = elves.iter().map(|e| e.0).max().unwrap();
        let max_y = elves.iter().map(|e| e.1).max().unwrap();

        let answer = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64;
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut elves = parse(input);
        let mut proposition_index = 0usize;

        loop {
            let (new_elves, no_move) = step(&elves, proposition_index);
            if no_move {
                break;
            }
            elves = new_elves;
            proposition_index += 1;
        }

        let answer = proposition_index + 1;
        Some(answer.to_string())
    }
}
