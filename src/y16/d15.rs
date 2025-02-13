use crate::util::Day;
use regex::Regex;

pub struct D15;

fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut disks: Vec<(i64, i64)> = vec![];

    let re = Regex::new(r"has (\d+) positions;.*position (\d+)").unwrap();
    for cap in re.captures_iter(input) {
        disks.push((
            cap[1].parse::<i64>().unwrap(),
            cap[2].parse::<i64>().unwrap(),
        ))
    }

    disks
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;

        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_s, old_t)
}

fn solve(disks: Vec<(i64, i64)>) -> usize {
    // (a_1 + 1 + t) % n_1 == 0
    // (a_2 + 2 + t) % n_2 == 0
    // ...
    //
    // (a'_1 + t) % n_1 == 0
    // (a'_2 + t) % n_2 == 0
    // ...
    //
    // t == -a'_1 (mod n_1)
    // t == -a'_2 (mod n_2)
    // ...
    //
    // then use https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(constructive_proof)

    let mut n_1 = 1;
    let mut a_1 = 0;

    for (idx, (n_2, mut a_2)) in disks.iter().enumerate() {
        a_2 = -(a_2 + idx as i64 + 1);

        let (m_1, m_2) = extended_gcd(n_1, *n_2);

        a_1 = (a_1 * m_2 * n_2 + a_2 * m_1 * n_1).rem_euclid(n_1 * n_2);
        n_1 *= n_2;
    }

    a_1 as usize
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let disks = parse(input);

        Option::from(solve(disks).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut disks = parse(input);

        disks.push((11, 0));

        Option::from(solve(disks).to_string())
    }
}
