use crate::util::Day;
use regex::Regex;

pub struct D9;

fn get_decompressed_length(input: &str, pattern_regex: &Regex, recursive: bool) -> usize {
    let mut i = 0;
    let mut len = 0;

    loop {
        if let Some(capture) = pattern_regex.captures(&input[i..]) {
            let full = capture.get(0).unwrap();
            let chars = capture["chars"].parse::<usize>().unwrap();
            let repeat = capture["repeat"].parse::<usize>().unwrap();

            len += full.start(); // everything before

            if recursive {
                len += repeat * get_decompressed_length(&input[i + full.end()..i + full.end() + chars], &pattern_regex, recursive);
            } else {
                len += repeat * chars;
            }


            i += full.end() + chars;
        } else {
            len += input.len() - i;
            break;
        }
    }

    len
}

fn solve(input: &str, recursive: bool) -> Option<String> {
    let input = input.trim();
    let pattern_regex = Regex::new(r"\((?P<chars>\d+)x(?P<repeat>\d+)\)").unwrap();

    let len = get_decompressed_length(input, &pattern_regex, recursive);

    Option::from(len.to_string())
}

impl Day for D9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        solve(input, false)
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        solve(input, true)
    }
}
