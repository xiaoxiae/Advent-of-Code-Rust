use crate::util::Day;
use std::collections::{HashMap, VecDeque};

pub struct D17;

static ROOM_SIZE: usize = 4;


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    position: (isize, isize),
    hash: String,
}

impl State {
    fn neighbours(&self) -> Vec<State> {
        let mut states = vec![];

        let md5 = md5::compute(&self.hash);
        let doors = format!("{:x}", md5);

        for (i, (dx, dy, d)) in [(0, -1, 'U'), (0, 1, 'D'), (-1, 0, 'L'), (1, 0, 'R')].iter().enumerate() {
            if doors.chars().nth(i).unwrap() <= 'a' {
                continue;
            }

            let position = (self.position.0 + dx, self.position.1 + dy);

            if position.0 < 0 || position.0 >= ROOM_SIZE as isize || position.1 < 0 || position.1 >= ROOM_SIZE as isize {
                continue;
            }

            let mut hash = self.hash.clone();
            hash.push(*d);

            states.push(State { hash, position });
        }

        states
    }
}


impl Day for D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let mut queue: VecDeque<(State, usize)> = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back((State { hash: String::from(id), position: (0, 0) }, 0));

        while let Some((state, dist)) = queue.pop_front() {
            if state.position == (3, 3) {
                return Some(state.hash[id.len()..].to_string());
            }

            for neighbor in state.neighbours() {
                distances.insert(neighbor.clone(), dist + 1);
                queue.push_back((neighbor.clone(), dist + 1));
            }
        }

        unreachable!()
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let mut queue: VecDeque<(State, usize)> = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back((State { hash: String::from(id), position: (0, 0) }, 0));

        let mut longest = usize::MAX;
        
        while let Some((state, dist)) = queue.pop_front() {
            if state.position == (3, 3) {
                longest = dist;
                continue;
            }

            for neighbor in state.neighbours() {
                distances.insert(neighbor.clone(), dist + 1);
                queue.push_back((neighbor.clone(), dist + 1));
            }
        }
        
        Option::from(longest.to_string())
    }
}
