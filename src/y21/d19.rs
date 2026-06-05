//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/19
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D19;

type Coordinate = (i64, i64, i64);
type Scanner = Vec<Coordinate>;

/// Orient the scanner in all possible ways.
fn get_orientations(scanner: &Scanner) -> Vec<Scanner> {
    let mut orientations = Vec::new();
    let signs = [1i64, -1i64];
    let perms: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    for &s1 in &signs {
        for &s2 in &signs {
            for &s3 in &signs {
                for p in &perms {
                    let mut oriented = Vec::with_capacity(scanner.len());
                    for beacon in scanner {
                        let b = [beacon.0, beacon.1, beacon.2];
                        let x = b[p[0]] * s1;
                        let y = b[p[1]] * s2;
                        let z = b[p[2]] * s3;
                        oriented.push((x, y, z));
                    }
                    orientations.push(oriented);
                }
            }
        }
    }
    orientations
}

/// Return all possible scanners when aligning all possible beacon pairs.
/// Each item: (delta, shifted_scanner).
fn get_shifts(shifts: &Scanner, scanner: &Scanner) -> Vec<(Coordinate, Scanner)> {
    let mut new_shifts = Vec::new();
    for &(x1, y1, z1) in shifts {
        for &(x2, y2, z2) in scanner {
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;

            let shifted = scanner
                .iter()
                .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
                .collect();
            new_shifts.push(((dx, dy, dz), shifted));
        }
    }
    new_shifts
}

/// Return the number of overlapping beacons for the scanners.
fn count_overlapping_beacons(s1: &Scanner, s2: &Scanner) -> usize {
    s1.iter()
        .map(|a| s2.iter().filter(|b| *b == a).count())
        .sum()
}

/// Find the correct orientation of the incorrect scanner, given a correct one.
/// Return its delta and shifted scanner.
fn find_correct_orientation(
    correct_scanner: &Scanner,
    incorrect_scanner: &Scanner,
) -> Option<(Coordinate, Scanner)> {
    for oriented_scanner in get_orientations(incorrect_scanner) {
        for (delta, shifted_scanner) in get_shifts(correct_scanner, &oriented_scanner) {
            if count_overlapping_beacons(correct_scanner, &shifted_scanner) >= 12 {
                return Some((delta, shifted_scanner));
            }
        }
    }
    None
}

fn parse(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    for block in input.trim().split("\n\n") {
        let mut scanner: Scanner = Vec::new();
        for line in block.lines().skip(1) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(',');
            let x: i64 = parts.next().unwrap().trim().parse().unwrap();
            let y: i64 = parts.next().unwrap().trim().parse().unwrap();
            let z: i64 = parts.next().unwrap().trim().parse().unwrap();
            scanner.push((x, y, z));
        }
        scanners.push(scanner);
    }
    scanners
}

/// Resolve all scanners; returns (scanners, positions).
fn resolve(input: &str) -> (Vec<Scanner>, Vec<Coordinate>) {
    let mut scanners = parse(input);
    let n = scanners.len();

    let mut oriented = vec![false; n];
    oriented[0] = true;

    let mut positions: Vec<Coordinate> = vec![(0, 0, 0); n];
    positions[0] = (0, 0, 0);

    let mut tested: FxHashSet<(usize, usize)> = FxHashSet::default();

    while !oriented.iter().all(|&b| b) {
        for i in 0..n {
            if !oriented[i] {
                continue;
            }

            for j in 0..n {
                if oriented[j] {
                    continue;
                }

                if tested.contains(&(i, j)) {
                    continue;
                }

                let correct_scanner = scanners[i].clone();
                let incorrect_scanner = scanners[j].clone();

                let result = find_correct_orientation(&correct_scanner, &incorrect_scanner);

                if let Some(((x, y, z), scanner)) = result {
                    oriented[j] = true;
                    scanners[j] = scanner;
                    positions[j] = (x, y, z);
                } else {
                    tested.insert((i, j));
                }
            }
        }
    }

    (scanners, positions)
}

fn distance(c1: Coordinate, c2: Coordinate) -> i64 {
    (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs() + (c1.2 - c2.2).abs()
}

impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (scanners, _positions) = resolve(input);

        let beacons: FxHashSet<Coordinate> = scanners.iter().flatten().copied().collect();

        Some(beacons.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (_scanners, positions) = resolve(input);

        let n = positions.len();
        let max_distance = (0..n)
            .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
            .map(|(i, j)| distance(positions[i], positions[j]))
            .max()
            .unwrap_or(0);

        Some(max_distance.to_string())
    }
}
