//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/05
use crate::util::Day;

pub struct D5;

/// Parse the input into:
/// - the list of seed numbers (first part)
/// - an ordered chain of maps. Each map is (from_category, to_category, ranges)
///   where ranges is a list of (fs, fs+d, ts) tuples, matching the Python
///   dict keyed by (fs, fs+d) with value ts (insertion order preserved).
fn parse(input: &str) -> (Vec<i64>, Vec<(String, String, Vec<(i64, i64, i64)>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut maps: Vec<(String, String, Vec<(i64, i64, i64)>)> = Vec::new();

    for part in &parts[1..] {
        let part = part.trim_end_matches('\n');
        if part.trim().is_empty() {
            continue;
        }
        let lines: Vec<&str> = part.lines().collect();

        // first token of header e.g. "seed-to-soil" -> split on '-'
        let header_first = lines[0].split_whitespace().next().unwrap();
        let hp: Vec<&str> = header_first.split('-').collect();
        let f = hp[0].to_string();
        let t = hp[2].to_string();

        let mut ranges: Vec<(i64, i64, i64)> = Vec::new();
        for line in &lines[1..] {
            if line.trim().is_empty() {
                continue;
            }
            let nums: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let (ts, fs, d) = (nums[0], nums[1], nums[2]);
            ranges.push((fs, fs + d, ts));
        }

        maps.push((f, t, ranges));
    }

    (seeds, maps)
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut seeds, maps) = parse(input);

        for (_f, _t, ranges) in maps.iter() {
            let mut new_seeds: Vec<i64> = Vec::new();
            for &s in seeds.iter() {
                let mut mapped = false;
                for &(l, h, ts) in ranges.iter() {
                    if l <= s && s <= h {
                        new_seeds.push(ts + (s - l));
                        mapped = true;
                        break;
                    }
                }
                if !mapped {
                    new_seeds.push(s);
                }
            }
            seeds = new_seeds;
        }

        let answer = seeds.iter().min().unwrap();
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (seeds, maps) = parse(input);

        let mut seed_ranges: Vec<(i64, i64)> = seeds
            .chunks(2)
            .map(|c| (c[0], c[0] + c[1]))
            .collect();

        for (_f, _t, ranges) in maps.iter() {
            let mut new_seed_ranges: Vec<(i64, i64)> = Vec::new();

            // NOTE: faithfully mirrors the Python which appends to seed_ranges
            // while iterating with an index, so the list grows during iteration.
            let mut idx = 0;
            while idx < seed_ranges.len() {
                let (sl, sh) = seed_ranges[idx];

                let mut matched = false;
                for &(l, h, ts) in ranges.iter() {
                    // no overlap - continue
                    if sh < l || h < sl {
                        continue;
                    }

                    // seeds are all within range - no split, just map
                    if l <= sl && sl <= sh && sh <= h {
                        new_seed_ranges.push((ts + (sl - l), ts + (sh - l)));
                        matched = true;
                        break;
                    } else {
                        // otherwise split into parts, add to seed ranges and continue
                        if sl < l {
                            seed_ranges.push((sl, l - 1));
                        }
                        if h < sh {
                            seed_ranges.push((h + 1, sh));
                        }
                        seed_ranges.push((sl.max(l), sh.min(h)));
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    new_seed_ranges.push((sl, sh));
                }

                idx += 1;
            }

            seed_ranges = new_seed_ranges;
        }

        let answer = seed_ranges.iter().map(|&(l, _)| l).min().unwrap();
        Some(answer.to_string())
    }
}
