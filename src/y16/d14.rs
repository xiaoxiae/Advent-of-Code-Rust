use crate::util::Day;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

pub struct D14;


static WINDOW: usize = 1000;
static KEY_COUNT: usize = 64;


fn solve(input: &str, iterations: usize) -> usize {
    let salt = input.trim();

    let mut triples: HashMap<char, VecDeque<usize>> = HashMap::new();
    let mut quintuples = HashMap::new();

    let mut id = 0;
    let mut keys = HashSet::new();

    // since we find keys by finding quintuples first, we need to do some extra iterations
    // to make sure we didn't skip over the 64th key
    let mut remaining_counter = WINDOW as isize;

    loop {
        let mut digest = md5::compute(format!("{}{}", salt, id));

        for _ in 0..iterations {
            digest = md5::compute(format!("{:x}", digest));
        }

        let hash = format!("{:x}", digest);

        // find triples/quintuples
        let mut prev_c = hash.chars().nth(0).unwrap();
        let mut count = 1;

        let mut quintuple_added = false;
        let mut triple_added = false;

        for c in hash[1..].chars() {
            if c == prev_c {
                count += 1;
            } else {
                prev_c = c;
                count = 1;
            }

            if count == 3 && !triple_added {
                triples.entry(c).or_insert(VecDeque::new()).push_back(id);
                triple_added = true;
            }

            if count == 5 {
                quintuples.insert(c, id);
                quintuple_added = true;
            }
        }

        // only check when something happens
        if quintuple_added {
            // nuke stuff that is too old
            triples.iter_mut().for_each(|(_, last_seen_ids)| last_seen_ids.retain(|last_seen_id| id - *last_seen_id <= WINDOW));
            quintuples.retain(|_, last_seen_id| (id - *last_seen_id) <= WINDOW);

            for (quintuple, position) in &quintuples {
                match triples.get_mut(quintuple) {
                    None => {}
                    Some(positions) => {
                        while positions.len() != 0 && positions[0] != *position {
                            let key = positions.pop_front();
                            keys.insert(key);
                        }
                    }
                }
            }
        }

        if keys.len() >= KEY_COUNT {
            if remaining_counter <= 0 {
                let last_key = keys.iter().sorted().nth(KEY_COUNT - 1).unwrap().unwrap();
                return last_key;
            }

            remaining_counter -= 1;
        }

        id += 1;
    }
}


impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 0).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 2016).to_string())
    }
}
