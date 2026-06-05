//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/04
use crate::util::Day;

pub struct D4;

/// Replicates Python's get_input(): strip whole text, then splitlines.
fn get_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

const CONDITIONS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn check_passport_part1(passport: &[&str]) -> bool {
    let mut conditions: Vec<&str> = CONDITIONS.to_vec();

    let joined = passport.join(" ");
    for kv in joined.split(' ') {
        if kv.is_empty() {
            // mimic empty passport: " ".join([]) == "" -> split(" ") == [""],
            // "".split(":") -> ValueError in Python; but here passport is
            // never empty when called in practice for part1's logic since
            // check_passport([]) would error. Skip safely.
            continue;
        }
        let key = kv.split(':').next().unwrap();
        if let Some(pos) = conditions.iter().position(|&c| c == key) {
            conditions.remove(pos);
        }
    }

    conditions.is_empty() || conditions == ["cid"]
}

fn check_passport_part2(passport: &[&str]) -> bool {
    let mut conditions: Vec<&str> = CONDITIONS.to_vec();

    let joined = passport.join(" ");
    for kv in joined.split(' ') {
        if kv.is_empty() {
            continue;
        }
        let mut it = kv.splitn(2, ':');
        let key = it.next().unwrap();
        let value = it.next().unwrap();

        if let Some(pos) = conditions.iter().position(|&c| c == key) {
            conditions.remove(pos);
        }

        match key {
            "byr" => {
                let Ok(v) = value.parse::<i64>() else {
                    return false;
                };
                if !(1920..=2002).contains(&v) {
                    return false;
                }
            }
            "iyr" => {
                let Ok(v) = value.parse::<i64>() else {
                    return false;
                };
                if !(2010..=2020).contains(&v) {
                    return false;
                }
            }
            "eyr" => {
                let Ok(v) = value.parse::<i64>() else {
                    return false;
                };
                if !(2020..=2030).contains(&v) {
                    return false;
                }
            }
            "hgt" => {
                if value.len() >= 2 && &value[value.len() - 2..] == "cm" {
                    let Ok(num) = value[..value.len() - 2].parse::<i64>() else {
                        return false;
                    };
                    if !(150..=193).contains(&num) {
                        return false;
                    }
                } else if value.len() >= 2 && &value[value.len() - 2..] == "in" {
                    let Ok(num) = value[..value.len() - 2].parse::<i64>() else {
                        return false;
                    };
                    if !(59..=76).contains(&num) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let bytes = value.as_bytes();
                if bytes.is_empty()
                    || bytes[0] != b'#'
                    || value[1..]
                        .chars()
                        .any(|c| !"0123456789abcdef".contains(c))
                {
                    return false;
                }
            }
            "ecl" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 || value.chars().any(|c| !"0123456789".contains(c)) {
                    return false;
                }
            }
            _ => {}
        }
    }

    conditions.is_empty() || conditions == ["cid"]
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lines = get_input(input);

        let mut valid_passports = 0;
        let mut passport: Vec<&str> = Vec::new();
        for line in &lines {
            if line.is_empty() {
                if check_passport_part1(&passport) {
                    valid_passports += 1;
                }
                passport.clear();
            } else {
                passport.push(line);
            }
        }

        if check_passport_part1(&passport) {
            valid_passports += 1;
        }

        Some(valid_passports.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let lines = get_input(input);

        let mut valid_passports = 0;
        let mut passport: Vec<&str> = Vec::new();
        for line in &lines {
            if line.is_empty() {
                if check_passport_part2(&passport) {
                    valid_passports += 1;
                }
                passport.clear();
            } else {
                passport.push(line);
            }
        }

        if check_passport_part2(&passport) {
            valid_passports += 1;
        }

        Some(valid_passports.to_string())
    }
}
