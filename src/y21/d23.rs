//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/23
//!
//! ⚠️ SLOW (~6.7s, part 1): faithful Dijkstra over amphipod states. Correct
//! (13495 / 53767) but flagged for manual optimization.
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::util::Day;

pub struct D23;

// ----------------------------------------------------------------------------
// Part 1: grid-based Dijkstra
// ----------------------------------------------------------------------------

type Diagram = Vec<Vec<char>>;

fn is_outside_room(x: i32, y: i32) -> bool {
    y == 1 && (x == 3 || x == 5 || x == 7 || x == 9)
}

fn is_hallway(x: i32, y: i32) -> bool {
    y == 1 && (1..12).contains(&x)
}

fn is_free(x: i32, y: i32, diagram: &Diagram) -> bool {
    diagram[y as usize][x as usize] == '.'
}

fn is_correct_room(amphipod: char, x: i32, _y: i32) -> bool {
    (amphipod == 'A' && x == 3)
        || (amphipod == 'B' && x == 5)
        || (amphipod == 'C' && x == 7)
        || (amphipod == 'D' && x == 9)
}

fn can_move_to(amphipod: char, x: i32, y: i32, xn: i32, yn: i32, diagram: &Diagram) -> bool {
    // can never move to outside the room
    if is_outside_room(xn, yn) {
        return false;
    }

    // can move to the correct room (but not to block the entrance)
    if is_correct_room(amphipod, xn, yn) {
        // NOTE: the original Python calls is_correct_room(xn, yn+1, diagram), which
        // (due to a quirky call) always evaluates to False, so this reduces to the
        // emptiness check below.
        return diagram[yn as usize][xn as usize] == '.';
    }

    // can move to the hallway, only if it's not in it already
    if is_hallway(xn, yn) && !is_hallway(x, y) {
        return true;
    }

    false
}

fn in_bounds(x: i32, y: i32, diagram: &Diagram) -> bool {
    y >= 0
        && (y as usize) < diagram.len()
        && x >= 0
        && (x as usize) < diagram[y as usize].len()
}

fn reachable_positions(x: i32, y: i32, diagram: &Diagram) -> HashMap<(i32, i32), i32> {
    let mut reachable: HashMap<(i32, i32), i32> = HashMap::new();
    let mut stack: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
    stack.push(Reverse((0, x, y)));

    while let Some(Reverse((d, x, y))) = stack.pop() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let xn = x + dx;
            let yn = y + dy;

            if in_bounds(xn, yn, diagram)
                && is_free(xn, yn, diagram)
                && !reachable.contains_key(&(xn, yn))
            {
                stack.push(Reverse((d + 1, xn, yn)));
                reachable.insert((xn, yn), d + 1);
            }
        }
    }

    reachable
}

fn move_cost(amphipod: char) -> i64 {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn is_correct(diagram: &Diagram) -> bool {
    diagram[2][3] == 'A'
        && diagram[3][3] == 'A'
        && diagram[2][5] == 'B'
        && diagram[3][5] == 'B'
        && diagram[2][7] == 'C'
        && diagram[3][7] == 'C'
        && diagram[2][9] == 'D'
        && diagram[3][9] == 'D'
}

fn diagram_key(diagram: &Diagram) -> String {
    let mut s = String::new();
    for row in diagram {
        for &c in row {
            s.push(c);
        }
        s.push('\n');
    }
    s
}

fn return_next_states(cost: i64, diagram: &Diagram) -> Vec<(i64, Diagram)> {
    let mut states = Vec::new();

    let mut amphipods: Vec<(char, i32, i32)> = Vec::new();
    let width = diagram[0].len();
    for y in 0..diagram.len() {
        for x in 0..width {
            if x < diagram[y].len() {
                let c = diagram[y][x];
                if c == 'A' || c == 'B' || c == 'C' || c == 'D' {
                    amphipods.push((c, x as i32, y as i32));
                }
            }
        }
    }

    for (amphipod, x, y) in amphipods {
        let positions = reachable_positions(x, y, diagram);

        for (&(xn, yn), &dist) in &positions {
            if can_move_to(amphipod, x, y, xn, yn, diagram) {
                let mut new_diagram: Diagram = diagram.clone();
                new_diagram[y as usize][x as usize] = '.';
                new_diagram[yn as usize][xn as usize] = amphipod;
                let new_cost = cost + dist as i64 * move_cost(amphipod);
                states.push((new_cost, new_diagram));
            }
        }
    }

    states
}

fn part1(input: &str) -> i64 {
    let raw_lines: Vec<&str> = input.lines().collect();

    let mut diagram: Diagram = Vec::new();
    for (i, line) in raw_lines.iter().enumerate() {
        let mut row: Vec<char> = line.chars().collect();
        while i != 0 && row.len() != diagram[i - 1].len() {
            row.push(' ');
        }
        diagram.push(row);
    }

    let mut states: BinaryHeap<Reverse<(i64, String, Diagram)>> = BinaryHeap::new();
    states.push(Reverse((0, diagram_key(&diagram), diagram.clone())));

    let mut explored: HashMap<String, i64> = HashMap::new();

    while let Some(Reverse((cost, key, diagram))) = states.pop() {
        if explored.contains_key(&key) {
            continue;
        }
        explored.insert(key, cost);

        if is_correct(&diagram) {
            return cost;
        }

        for (new_cost, new_diagram) in return_next_states(cost, &diagram) {
            let k = diagram_key(&new_diagram);
            states.push(Reverse((new_cost, k, new_diagram)));
        }
    }

    -1
}

// ----------------------------------------------------------------------------
// Part 2: abstracted "rooms" Dijkstra
// ----------------------------------------------------------------------------

type Rooms = Vec<Vec<char>>;

const ROOM_COUNT: usize = 11;
const ROOM_SIZES: [usize; 11] = [1, 1, 4, 1, 4, 1, 4, 1, 4, 1, 1];
const START_END_COST: [i64; 11] = [0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0];

fn empty_spaces(i: usize, rooms: &Rooms) -> i64 {
    ROOM_SIZES[i] as i64 - rooms[i].len() as i64
}

fn is_full(i: usize, rooms: &Rooms) -> bool {
    empty_spaces(i, rooms) == 0
}

fn is_passthrough(i: usize) -> bool {
    matches!(i, 2 | 4 | 6 | 8)
}

fn is_room_idx(i: usize) -> bool {
    i % 2 == 0 && i != 0 && i != ROOM_COUNT - 1
}

fn is_hallway_idx(i: usize) -> bool {
    !is_room_idx(i)
}

fn correct_room_amphipod(i: usize) -> char {
    match i {
        2 => 'A',
        4 => 'B',
        6 => 'C',
        8 => 'D',
        _ => unreachable!(),
    }
}

fn contains_only_valid_amphipods(i: usize, rooms: &Rooms) -> bool {
    let target = correct_room_amphipod(i);
    rooms[i].iter().all(|&v| v == target)
}

fn possible_moves(amphipod: char, i: usize, rooms: &Rooms) -> Vec<usize> {
    let mut moves = Vec::new();

    for dx in [-1i32, 1i32] {
        let mut j = i as i32 + dx;
        while j >= 0 && (j as usize) < rooms.len() {
            let ju = j as usize;
            if is_full(ju, rooms) {
                if !is_passthrough(ju) {
                    break;
                }
            } else {
                if is_hallway_idx(i)
                    && is_room_idx(ju)
                    && correct_room_amphipod(ju) == amphipod
                    && contains_only_valid_amphipods(ju, rooms)
                {
                    moves.push(ju);
                }

                if is_room_idx(i) && is_hallway_idx(ju) {
                    moves.push(ju);
                }
            }

            j += dx;
        }
    }

    moves
}

fn is_correct_p2(rooms: &Rooms) -> bool {
    for i in 0..4 {
        let c = i * 2 + 2;
        if !is_full(c, rooms) || !contains_only_valid_amphipods(c, rooms) {
            return false;
        }
    }
    true
}

fn rooms_key(rooms: &Rooms) -> String {
    let mut s = String::new();
    for room in rooms {
        for &c in room {
            s.push(c);
        }
        s.push('|');
    }
    s
}

fn return_next_states_p2(cost: i64, rooms: &Rooms) -> Vec<(i64, Rooms)> {
    let mut states = Vec::new();

    for i in 0..rooms.len() {
        if !rooms[i].is_empty() {
            let amphipod = *rooms[i].last().unwrap();
            for j in possible_moves(amphipod, i, rooms) {
                let mut cost_delta =
                    (START_END_COST[i] + empty_spaces(i, rooms)) * move_cost(amphipod);

                let lo = i.min(j);
                let hi = i.max(j);
                cost_delta += (hi - lo - 1) as i64 * move_cost(amphipod);

                cost_delta += (START_END_COST[j] + empty_spaces(j, rooms)) * move_cost(amphipod);

                let mut new_rooms: Rooms = rooms.clone();
                let v = new_rooms[i].pop().unwrap();
                new_rooms[j].push(v);

                states.push((cost + cost_delta, new_rooms));
            }
        }
    }

    states
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let mut rooms: Rooms = vec![Vec::new(); ROOM_COUNT];

    // parse the input
    for line in &lines {
        for (x, v) in line.chars().enumerate() {
            if v == 'A' || v == 'B' || v == 'C' || v == 'D' {
                let idx = (((x as i32 - 3) / 2 + 1) * 2) as usize;
                rooms[idx].insert(0, v);
            }
        }
    }

    for line in [['D', 'C', 'B', 'A'], ['D', 'B', 'A', 'C']] {
        for (i, &c) in line.iter().enumerate() {
            rooms[i * 2 + 2].insert(1, c);
        }
    }

    let mut states: BinaryHeap<Reverse<(i64, String, Rooms)>> = BinaryHeap::new();
    states.push(Reverse((0, rooms_key(&rooms), rooms.clone())));

    let mut explored: HashMap<String, i64> = HashMap::new();

    while let Some(Reverse((cost, key, rooms))) = states.pop() {
        if explored.contains_key(&key) {
            continue;
        }
        explored.insert(key, cost);

        if is_correct_p2(&rooms) {
            return cost;
        }

        for (new_cost, new_rooms) in return_next_states_p2(cost, &rooms) {
            let k = rooms_key(&new_rooms);
            states.push(Reverse((new_cost, k, new_rooms)));
        }
    }

    -1
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(part1(input).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(part2(input).to_string())
    }
}
