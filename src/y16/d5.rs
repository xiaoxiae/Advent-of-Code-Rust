use crate::util::Day;
use rayon::prelude::*;

pub struct D5;

static CONST_SIZE: usize = 3_usize.pow(13);

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let id = input.trim();
        let mut password = vec![None; 8];
        let mut valid_chars = 0;
        let mut i = 0;

        let input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        for _ in 0..8 {
            loop {
                match (i..i + CONST_SIZE).into_par_iter().find_map_first(|i| {
                    let mut copy = input.clone();
                    let mut j = i;

                    while j != 0 {
                        copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                        j /= 10;
                    }

                    let hash = md5::compute(&copy);

                    if hash[0] == 0 && hash[1] == 0 && (hash[2].wrapping_shr(4)) == 0 {
                        Option::from((i, format!("{:02x}", hash[2]).chars().nth(1)))
                    } else {
                        None
                    }
                }) {
                    None => i += CONST_SIZE,
                    Some((new_i, char)) => {
                        i = new_i + 1;

                        password[valid_chars] = char;
                        valid_chars += 1;

                        break;
                    }
                }
            }
        }

        Some(password.into_iter().filter_map(|c| c).collect())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let id = input.trim();
        let mut password = vec![None; 8];
        let mut valid_chars = 0;
        let mut i = 0;

        let input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        while valid_chars != 8 {
            loop {
                match (i..i + CONST_SIZE).into_par_iter().find_map_first(|i| {
                    let mut copy = input.clone();
                    let mut j = i;

                    while j != 0 {
                        copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                        j /= 10;
                    }

                    let hash = md5::compute(&copy);

                    if hash[0] == 0 && hash[1] == 0 && (hash[2].wrapping_shr(4)) == 0 {
                        Option::from((i, hash[2] % 16, format!("{:02x}", hash[3]).chars().nth(0)))
                    } else {
                        None
                    }
                }) {
                    None => i += CONST_SIZE,
                    Some((new_i, position, char)) => {
                        i = new_i + 1;

                        if position as usize >= password.len()
                            || !password[position as usize].is_none()
                        {
                            continue;
                        }

                        password[position as usize] = char;
                        valid_chars += 1;

                        break;
                    }
                }
            }
        }

        Some(password.into_iter().filter_map(|c| c).collect())
    }
}
