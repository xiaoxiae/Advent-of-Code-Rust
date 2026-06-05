//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2023/tree/master/12
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D12;

fn count_ways(
    record: &[u8],
    groups: &[usize],
    cache: &mut FxHashMap<(Vec<u8>, Vec<usize>), i64>,
) -> i64 {
    let key = (record.to_vec(), groups.to_vec());
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let result = compute(record, groups, cache);
    cache.insert(key, result);
    result
}

fn can_be_eaten(record: &[u8], g: usize) -> bool {
    // "." not in record[:g] and (len(record) == g or record[g] in ("?", "."))
    if record[..g].contains(&b'.') {
        return false;
    }
    record.len() == g || record[g] == b'?' || record[g] == b'.'
}

fn compute(
    record: &[u8],
    groups: &[usize],
    cache: &mut FxHashMap<(Vec<u8>, Vec<usize>), i64>,
) -> i64 {
    if groups.is_empty() {
        if record.contains(&b'#') {
            return 0;
        }
        return 1;
    }

    if record.len() < groups[0] {
        return 0;
    }

    match record[0] {
        b'.' => {
            // record.lstrip(".")
            let i = record.iter().take_while(|&&c| c == b'.').count();
            count_ways(&record[i..], groups, cache)
        }
        b'#' => {
            if can_be_eaten(record, groups[0]) {
                let next = (groups[0] + 1).min(record.len());
                count_ways(&record[next..], &groups[1..], cache)
            } else {
                0
            }
        }
        _ => {
            let dot_result = count_ways(&record[1..], groups, cache);
            let mut hash_result = 0;
            if can_be_eaten(record, groups[0]) {
                let next = (groups[0] + 1).min(record.len());
                hash_result = count_ways(&record[next..], &groups[1..], cache);
            }
            dot_result + hash_result
        }
    }
}

impl D12 {
    fn run(&self, input: &str, unfold: bool) -> i64 {
        let mut total: i64 = 0;
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            let records = parts.next().unwrap();
            let groups_str = parts.next().unwrap();
            let groups: Vec<usize> = groups_str
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            let (rec, grp): (String, Vec<usize>) = if unfold {
                let r = vec![records; 5].join("?");
                let g = groups.repeat(5);
                (r, g)
            } else {
                (records.to_string(), groups)
            };

            let mut cache: FxHashMap<(Vec<u8>, Vec<usize>), i64> = FxHashMap::default();
            total += count_ways(rec.as_bytes(), &grp, &mut cache);
        }
        total
    }
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(self.run(input, false).to_string())
    }
    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(self.run(input, true).to_string())
    }
}
