use crate::util::Day;
use md5;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct D4;

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let mut input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        // TODO: not sure how to do this using infinite iterator
        (1..100_000_000)
            .into_par_iter()
            .find_first(|&i| {
                let mut copy = input.clone();
                let mut j = i.clone();

                while j != 0 {
                    copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                    j /= 10;
                }

                let hash = md5::compute(&copy);

                hash[0] == 0 && hash[1] == 0 && (hash[2].wrapping_shr(4)) == 0
            })
            .map(|i| i.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let id = input.trim();

        let mut input = id.chars().map(|c| c as u8).collect::<Vec<_>>();

        (1..100_000_000)
            .into_par_iter()
            .find_first(|&i| {
                let mut copy = input.clone();
                let mut j = i.clone();

                while j != 0 {
                    copy.insert(input.len(), (j % 10 + (b'0' as usize)) as u8);
                    j /= 10;
                }

                let hash = md5::compute(&copy);

                hash[0] == 0 && hash[1] == 0 && hash[2] == 0
            })
            .map(|i| i.to_string())
    }
}
