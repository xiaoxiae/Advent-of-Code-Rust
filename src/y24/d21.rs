use crate::util::Day;
use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::iter::{once, repeat};

static NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

static PADPAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

type Pad = Vec<Vec<char>>;
type PadPaths = HashMap<char, HashMap<char, Vec<Sequence>>>;
type Sequence = Vec<char>;

fn pad_paths(pad: Pad) -> PadPaths {
    let mut map: PadPaths = HashMap::default();

    for y in 0..pad.len() {
        for x in 0..pad[y].len() {
            if pad[y][x] == ' ' {
                continue;
            }

            let mut local_map: HashMap<char, Vec<Sequence>> = HashMap::default();

            for ny in 0..pad.len() {
                for nx in 0..pad[y].len() {
                    if pad[ny][nx] == ' ' {
                        continue;
                    }

                    let mut vec = Vec::new();

                    let dx = nx as isize - x as isize;
                    let dy = ny as isize - y as isize;

                    for _ in 0..dx.abs() {
                        vec.push(if dx < 0 { '<' } else { '>' })
                    }

                    for _ in 0..dy.abs() {
                        vec.push(if dy < 0 { '^' } else { 'v' })
                    }

                    let mut shortest_sequences: HashSet<Sequence> = HashSet::default();

                    for p in vec.iter().permutations(vec.len()) {
                        // ensure that we don't step over a gap
                        let (mut tx, mut ty) = (x, y);
                        let mut gap = false;
                        for c in &p {
                            match c {
                                '^' => ty -= 1,
                                '<' => tx -= 1,
                                'v' => ty += 1,
                                '>' => tx += 1,
                                _ => panic!("Unknown movement symbol!"),
                            }

                            if pad[ty][tx] == ' ' {
                                gap = true;
                                break;
                            }
                        }

                        if !gap {
                            let mut p = p.into_iter().cloned().collect::<Sequence>();
                            p.push('A');

                            shortest_sequences.insert(p);
                        }
                    }

                    local_map.insert(
                        pad[ny][nx],
                        shortest_sequences.into_iter().collect::<Vec<Sequence>>(),
                    );
                }
            }

            map.insert(pad[y][x], local_map);
        }
    }

    map
}

/// This function returns the number of keypresses at the given 'level',
/// starting at 'start' and typing in the 'sequence' using 'pads'.
fn recursive<'a>(
    start: char,
    sequence: &'a Sequence,
    pads: &Vec<&'a HashMap<char, HashMap<char, Vec<Sequence>>>>,
    level: usize,
    cache: &mut HashMap<(char, &'a Sequence, usize), usize>,
) -> usize {
    if cache.contains_key(&(start, sequence, level)) {
        return *cache.get(&(start, sequence, level)).unwrap();
    }

    if pads.len() == level {
        return 1;
    }

    let mut total = 0;
    let mut s = start;

    // every pad starts at A
    for (&c1, &c2) in once(&'A').chain(sequence.iter()).tuple_windows() {
        let mut t = usize::MAX;

        // go through all shortest paths between characters on a numpad and pick the minimum
        for p in pads[level].get(&c1).unwrap().get(&c2).unwrap() {
            let l = recursive(s, p, pads, level + 1, cache);
            t = t.min(l);
        }

        s = c2;

        total += t;
    }

    // cache go brrr
    cache.insert((start, sequence, level), total);

    total
}

fn solve(input: &str, pads: &Vec<&PadPaths>) -> usize {
    let complexities = input
        .trim()
        .split_whitespace()
        .map(|r| {
            let s: Sequence = r.chars().collect();

            let a: usize = s
                .iter()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();

            let mut cache = HashMap::default();
            let total = recursive('A', &s, &pads, 0, &mut cache);

            a * total
        })
        .sum::<usize>();

    complexities
}

pub struct D21;

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let numpad = pad_paths(NUMPAD.map(|x| x.to_vec()).to_vec());
        let padpad = pad_paths(PADPAD.map(|x| x.to_vec()).to_vec());

        let pads = once(&numpad)
            .chain(repeat(&padpad).take(2 + 1))
            .collect::<Vec<_>>();

        let result = solve(input, &pads);

        Option::from(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let numpad = pad_paths(NUMPAD.map(|x| x.to_vec()).to_vec());
        let padpad = pad_paths(PADPAD.map(|x| x.to_vec()).to_vec());

        let pads = once(&numpad)
            .chain(repeat(&padpad).take(25 + 1))
            .collect::<Vec<_>>();

        let result = solve(input, &pads);

        Option::from(result.to_string())
    }
}
