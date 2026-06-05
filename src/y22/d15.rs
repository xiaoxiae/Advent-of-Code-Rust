//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2022/tree/master/15
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D15;

fn d(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for row in input.lines() {
        let row = row.trim();
        if row.is_empty() {
            continue;
        }

        let parts: Vec<&str> = row.split_whitespace().collect();

        // parts[2] = "x=...," -> [2:-1]
        let sx: i64 = parts[2][2..parts[2].len() - 1].parse().unwrap();
        // parts[3] = "y=...:" -> [2:-1]
        let sy: i64 = parts[3][2..parts[3].len() - 1].parse().unwrap();
        // parts[-2] = "x=...," -> [2:-1]
        let bx_part = parts[parts.len() - 2];
        let bx: i64 = bx_part[2..bx_part.len() - 1].parse().unwrap();
        // parts[-1] = "y=..." -> [2:]
        let by_part = parts[parts.len() - 1];
        let by: i64 = by_part[2..].parse().unwrap();

        sensors.push((sx, sy));
        beacons.push((bx, by));
    }

    (sensors, beacons)
}

/// Get the interval of values at level y which is covered by the sensor.
fn get_beacon_interval(beacon: (i64, i64), sensor: (i64, i64), y: i64) -> Option<(i64, i64)> {
    let dy = (y - sensor.1).abs();
    let dist = d(beacon, sensor);

    let interval = (sensor.0 - dist + dy, sensor.0 + dist - dy);

    if interval.0 > interval.1 {
        None
    } else {
        Some(interval)
    }
}

/// Get the bordering values of the sensor (one _after_ its range).
fn get_bordering_values(beacon: (i64, i64), sensor: (i64, i64)) -> Vec<(i64, i64)> {
    let dist = d(beacon, sensor) + 1;

    let mut values = Vec::new();
    for i in -dist..=0 {
        values.push((sensor.0 - dist + i, sensor.1 + i));
        values.push((sensor.0 - dist + i, sensor.1 - i));
    }

    for i in 0..=dist {
        values.push((sensor.0 + i, sensor.1 + dist - i));
        values.push((sensor.0 + i, sensor.1 - dist + i));
    }

    values
}

/// Is a point covered by any of the sensors?
fn is_covered(point: (i64, i64), beacons: &[(i64, i64)], sensors: &[(i64, i64)]) -> bool {
    for (beacon, sensor) in beacons.iter().zip(sensors.iter()) {
        if d(point, *sensor) <= d(*beacon, *sensor) {
            return true;
        }
    }
    false
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (sensors, beacons) = parse(input);
        let y: i64 = 2000000;

        let intervals = beacons
            .iter()
            .zip(sensors.iter())
            .filter_map(|(&beacon, &sensor)| get_beacon_interval(beacon, sensor, y));

        // slow and disgusting but I'm too lazy
        // NOTE: faithfully replicates Python `set(range(*interval))` which is
        // half-open [interval.0, interval.1), excluding the endpoint.
        let mut values: FxHashSet<i64> = FxHashSet::default();
        for interval in intervals {
            for v in interval.0..interval.1 {
                values.insert(v);
            }
        }

        Some(values.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (sensors, beacons) = parse(input);
        let max_values: i64 = 4000000;

        for (&beacon, &sensor) in beacons.iter().zip(sensors.iter()) {
            for value in get_bordering_values(beacon, sensor) {
                if !(0 <= value.0 && value.0 <= max_values && 0 <= value.1 && value.1 <= max_values)
                {
                    continue;
                }

                if !is_covered(value, &beacons, &sensors) {
                    return Some((value.0 * 4000000 + value.1).to_string());
                }
            }
        }

        None
    }
}
