use crate::util::Day;
use md5;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct D4;

static CONST_SIZE: usize = 3_usize.pow(13);

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut i = 0;
        loop {
            match (i..i + CONST_SIZE).into_par_iter().find_first(|&i| {
                let mut copy = input.clone();
                let mut j = i.clone();

                while j != 0 {
                    copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                    j /= 10;
                }

                let hash = md5::compute(&copy);

                hash[0] == 0 && hash[1] == 0 && (hash[2].wrapping_shr(4)) == 0
            }) {
                None => i += CONST_SIZE,
                Some(v) => return Option::from(v.to_string()),
            }
        }
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        let mut i = 0;
        loop {
            match (i..i + CONST_SIZE).into_par_iter().find_first(|&i| {
                let mut copy = input.clone();
                let mut j = i.clone();

                while j != 0 {
                    copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                    j /= 10;
                }

                let hash = md5::compute(&copy);

                hash[0] == 0 && hash[1] == 0 && hash[2] == 0
            }) {
                None => i += CONST_SIZE,
                Some(v) => return Option::from(v.to_string()),
            }
        }
    }
}
