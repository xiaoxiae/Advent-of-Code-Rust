use itertools::Itertools;
use crate::util::Day;

pub struct D10;



fn look_and_say(input: &Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    input.iter().flat_map(|(digit, count)| {
        vec![count, digit]
    }).chunk_by(|&c| c)
        .into_iter()
        .map(|(_, group)| {
            let items = group.collect::<Vec<_>>();

            (*items[0], items.len() as u8)
        })
        .collect::<Vec<_>>()
}


fn solve(input: &str, iterations: usize) -> usize {
    let mut input = input.trim().chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(_, group)| {
            let items = group.collect::<Vec<_>>();

            (items[0].to_digit(10).unwrap() as u8, items.len() as u8)
        })
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        input = look_and_say(&input);
    }

    input.into_iter().map(|(_, c)| c as usize).sum::<usize>()
}



impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let length = solve(input, 40);

        Option::from(length.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let length = solve(input, 50);

        Option::from(length.to_string())
    }
}
