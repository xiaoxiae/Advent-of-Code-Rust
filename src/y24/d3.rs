use crate::util::Day;
use regex::Regex;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bytes = input.as_bytes();

        let mut i = 0;
        let mut total = 0;
        'outer: while i < bytes.len() {
            match bytes[i..] {
                [b'm', b'u', b'l', b'(', ..] => {
                    let mut n1: usize = 0;
                    i += 4;

                    while bytes[i] != b',' {
                        if !bytes[i].is_ascii_digit() {
                            continue 'outer;
                        }

                        n1 = n1 * 10 + (bytes[i] - b'0') as usize;
                        i += 1;
                    }

                    let mut n2: usize = 0;
                    i += 1;

                    while bytes[i] != b')' {
                        if !bytes[i].is_ascii_digit() {
                            continue 'outer;
                        }

                        n2 = n2 * 10 + (bytes[i] - b'0') as usize;
                        i += 1;
                    }

                    total += n1 * n2;
                    i += 1;
                }
                _ => i += 1,
            }
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bytes = input.as_bytes();
        let mut enabled = true;

        let mut i = 0;
        let mut total = 0;
        'outer: while i < bytes.len() {
            match bytes[i..] {
                [b'd', b'o', b'(', b')', ..] => {
                    enabled = true;
                    i += 4;
                }
                [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                    enabled = false;
                    i += 6;
                }
                [b'm', b'u', b'l', b'(', ..] => {
                    i += 4;

                    if !enabled {
                        continue;
                    }

                    let mut n1: usize = 0;

                    while bytes[i] != b',' {
                        if !bytes[i].is_ascii_digit() {
                            continue 'outer;
                        }

                        n1 = n1 * 10 + (bytes[i] - b'0') as usize;
                        i += 1;
                    }

                    let mut n2: usize = 0;
                    i += 1;

                    while bytes[i] != b')' {
                        if !bytes[i].is_ascii_digit() {
                            continue 'outer;
                        }

                        n2 = n2 * 10 + (bytes[i] - b'0') as usize;
                        i += 1;
                    }

                    total += n1 * n2;
                    i += 1;
                }
            _ => i += 1,
            }
        }

        Option::from(total.to_string())
    }

    /// --- Tom's Part 3 ---
    /// Actually, we would like to match things in the form "mul(a, b)...mul(c, d)"
    /// What we want to do for each match is to calculate a * d + b...c, where b...c
    /// is a concatenation of b, all digits in ... and c. Do and Don't still work.
    fn solve_part3(&self, input: &str) -> Option<String> {
        let mut total = 0;
        let mut enabled = true;

        for line in input.trim().lines() {
            let re = Regex::new(r"mul\((\d+),(\d+)(.+?)(\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

            for cap in re.captures_iter(line) {
                let whole = cap.get(0).unwrap().as_str();

                match whole {
                    "do()" => {
                        enabled = true;
                    }
                    "don't()" => {
                        enabled = false;
                    }
                    _ if enabled => {
                        let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                        let d = cap.get(5).unwrap().as_str().parse::<i64>().unwrap();

                        let b = cap.get(2).unwrap().as_str();
                        let c = cap.get(4).unwrap().as_str();
                        let middle = cap
                            .get(3)
                            .unwrap()
                            .as_str()
                            .chars()
                            .filter(|c| c.is_digit(10))
                            .collect::<String>();

                        total += a * d + format!("{}{}{}", b, middle, c).parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
        }

        Option::from(total.to_string())
    }
}
