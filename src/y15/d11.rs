use itertools::Itertools;
use crate::util::Day;

pub struct D11;


static FORBIDDEN: [u8; 3] = [b'i' - b'a', b'o' - b'a', b'l' - b'a'];


fn increment(password: &mut Vec<u8>) {
    for i in (0..password.len()).rev() {
        password[i] += 1;

        if password[i] == 26 {
            password[i] = 0;
        } else {
            break;
        }
    }
}

fn valid(password: &Vec<u8>) -> bool {
    // contains pairs
    let mut i = 0;
    let mut pairs = 0;
    while i < password.len() - 1 {
        if password[i] == password[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    if pairs < 2 {
        return false;
    }

    // forbidden letters
    if password.iter().any(|&c| FORBIDDEN.contains(&c)) {
        return false;
    }

    // raising sequence
    if !password.windows(3).any(|w| (w[0] + 2 == w[1] + 1) && (w[1] + 1 == w[2])) {
        return false;
    }

    true
}

fn solve(input: &str, count: usize) -> Option<String> {
    let mut password = input.trim().chars().map(|c| c as u8 - b'a').collect::<Vec<_>>();

    for _ in 0..count {
        increment(&mut password);  // assumes the first one is already expired :)
        
        while !valid(&password) {
            increment(&mut password);
        }
    }

    Option::from(password.iter().map(|&c| (c + b'a') as char).collect::<String>())
}


impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        solve(input, 1)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        solve(input, 2)
    }
}
