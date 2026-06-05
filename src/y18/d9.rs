//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/09
use crate::util::Day;

pub struct D9;

fn parse(input: &str) -> (usize, usize) {
    let nums: Vec<usize> = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1])
}

/// Doubly-linked list over a flat arena, indexed by usize.
/// next[i] / prev[i] hold the neighbours of marble stored at slot i.
fn play(players: usize, last_marble: usize) -> u64 {
    let mut scores = vec![0u64; players];

    // Pre-allocate the arena: one slot per marble value (0..=last_marble).
    let mut next = vec![0usize; last_marble + 1];
    let mut prev = vec![0usize; last_marble + 1];

    // marble 0 links onto itself
    next[0] = 0;
    prev[0] = 0;
    let mut current = 0usize; // slot index == marble value

    for i in 1..=last_marble {
        if i % 23 == 0 {
            // go 7 marbles back (Python: 6 prev steps after the implicit current)
            for _ in 0..6 {
                current = prev[current];
            }

            // increment player score: (i - 1) % players
            scores[(i - 1) % players] += current as u64 + i as u64;

            // remove the marble at `current` by relinking its neighbours.
            // Note we deliberately leave next[current]/prev[current] untouched:
            // the Python keeps `currentMarble` pointing at the removed node,
            // whose stale `.next` still points to its old successor. The next
            // non-23 step then does current.next.next off this stale pointer,
            // which our arena reproduces because next[current] is preserved.
            let p = prev[current];
            let n = next[current];
            next[p] = n;
            prev[n] = p;
            // `current` stays as the (now-detached) removed slot index.
        } else {
            // go by 2 forward (current.next.next)
            current = next[next[current]];

            // create the new marble (value i, stored at slot i) and link it
            // between `current` and current.next
            // Insert the new marble between `current` and current.next.
            // Python does NOT reassign `current` to the new node here: it stays
            // pointing at the node two-forward, with the new marble right after
            // it. We mirror that by leaving `current` unchanged.
            let after = next[current];
            next[i] = after;
            prev[i] = current;
            next[current] = i;
            prev[after] = i;
        }
    }

    scores.into_iter().max().unwrap_or(0)
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (players, last_marble) = parse(input);
        Some(play(players, last_marble).to_string())
    }
    fn solve_part2(&self, input: &str) -> Option<String> {
        let (players, last_marble) = parse(input);
        Some(play(players, last_marble * 100).to_string())
    }
}
