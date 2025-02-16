use std::cmp::Ordering;
use crate::util::Day;

pub struct D22;

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct NodeData {
    used: usize,
    avail: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    space: Position,
    data: Position,
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        vec![
            Position { x: self.x + 1, y: self.y },
            Position { x: self.x - 1, y: self.y },
            Position { x: self.x, y: self.y + 1 },
            Position { x: self.x, y: self.y - 1 },
        ]
    }

    fn distance(&self, other: &Position) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

impl State {
    fn neighbours(&self, free: &HashSet<Position>) -> HashMap<State, usize> {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back((self.space, 0));
        distances.insert(self.space, 0);

        while let Some((pos, dist)) = queue.pop_front() {
            for neighbor in pos.neighbours() {
                if free.contains(&neighbor) && !distances.contains_key(&neighbor) && neighbor != self.data {
                    distances.insert(neighbor, dist + 1);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        distances.retain(|pos, _| self.data.neighbours().contains(pos));

        let mut neighbors = HashMap::new();
        for (pos, dist) in distances.iter() {
            let new_state = State { space: self.data, data: *pos };
            neighbors.insert(new_state, dist + 1);
        }

        neighbors
    }
}

fn parse_line(line: &str) -> Option<(isize, isize, usize, usize)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 { return None; }

    let path_parts: Vec<&str> = parts[0].split('-').collect();
    if path_parts.len() < 3 { return None; }

    let x = path_parts[1].strip_prefix("x")?.parse().ok()?;
    let y = path_parts[2].strip_prefix("y")?.parse().ok()?;
    let used = parts[2].strip_suffix("T")?.parse().ok()?;
    let avail = parts[3].strip_suffix("T")?.parse().ok()?;

    Some((x, y, used, avail))
}

fn parse(input: &str) -> HashMap<Position, NodeData> {
    let mut grid = HashMap::new();

    for line in input.lines() {
        if let Some((x, y, used, avail)) = parse_line(line) {
            grid.insert(Position { x, y }, NodeData { used, avail });
        }
    }

    grid
}


#[derive(Debug, Eq, PartialEq)]
struct QueueEntry {
    cost: usize,
    heuristic: usize,
    state: State,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic)) // Reverse order for min-heap
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(start: State, end: Position, free: &HashSet<Position>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    heap.push(QueueEntry { cost: 0, heuristic: start.data.distance(&end), state: start });
    distances.insert(start, 0);

    while let Some(QueueEntry { cost, state, .. }) = heap.pop() {
        if state.data == end {
            return Some(cost);
        }

        for (neighbor, move_cost) in state.neighbours(free) {
            let new_cost = cost + move_cost;
            if !distances.contains_key(&neighbor) || new_cost < distances[&neighbor] {
                // the best case for moving data is to run around to the other size,
                // which requires 5 moves; 3 when it's turning a corner but that's a special case
                let heuristic = neighbor.data.distance(&end) * 5;

                distances.insert(neighbor, new_cost);
                heap.push(QueueEntry { cost: new_cost, heuristic, state: neighbor });
            }
        }
    }

    None
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let grid = parse(input);

        let mut free_pairs = 0;

        for (&k1, n1) in &grid {
            for (&k2, n2) in &grid {
                if n1.used == 0 {
                    continue;
                }

                if k1 == k2 {
                    continue;
                }

                if n1.used <= n2.avail {
                    free_pairs += 1;
                }
            }
        }

        Option::from(free_pairs.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let grid = parse(input);

        // this solution makes the assumptions that are outlined in the problem description:
        // - only 1 free space at a time (no double merging)

        let mut max_x = 0;

        let mut space = Position { x: 0, y: 0 };
        let mut space_avail = 0;

        for (&key, node) in &grid {
            max_x = max_x.max(key.x);

            if node.used == 0 {
                space = key;
                space_avail = node.avail;
            }
        }

        let mut free = HashSet::new();
        for (&key, node) in &grid {
            if node.used <= space_avail {
                free.insert(key);
            }
        }

        let start = State { space, data: Position { x: max_x, y: 0 } };

        let result = solve(start, Position {x: 0, y: 0}, &free);

        Option::from(result.unwrap().to_string())
    }
}
