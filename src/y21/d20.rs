//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/20
use crate::util::Day;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct D20;

type Coordinate = (i64, i64);
type Image = FxHashMap<Coordinate, u8>;

fn yield_change_coordinates(image: &Image) -> FxHashSet<Coordinate> {
    let mut coordinates = FxHashSet::default();
    for &(x, y) in image.keys() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                coordinates.insert((x + dx, y + dy));
            }
        }
    }
    coordinates
}

fn get_default_value(phase: i64, algorithm: &[u8]) -> i64 {
    // 0 by default (if the algorithm doesn't flip all values each iteration),
    // otherwise phase % 2.
    if algorithm[0] == b'.' {
        0
    } else {
        phase % 2
    }
}

fn get_next_pixel_value(
    coordinate: Coordinate,
    image: &Image,
    algorithm: &[u8],
    phase: i64,
) -> u8 {
    let (x, y) = coordinate;

    let mut index: usize = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let c = (x + dx, y + dy);
            let value = match image.get(&c) {
                None => get_default_value(phase, algorithm),
                Some(&ch) => {
                    if ch == b'.' {
                        0
                    } else {
                        1
                    }
                }
            };
            index = (index << 1) | (value as usize);
        }
    }

    algorithm[index]
}

fn get_next_image(image: &Image, algorithm: &[u8], phase: i64) -> Image {
    let mut new_image = Image::default();
    for coordinate in yield_change_coordinates(image) {
        new_image.insert(
            coordinate,
            get_next_pixel_value(coordinate, image, algorithm, phase),
        );
    }
    new_image
}

fn parse(input: &str) -> (Vec<u8>, Image) {
    let input = input.trim();
    let (algorithm, image_string) = input.split_once("\n\n").unwrap();
    let algorithm: Vec<u8> = algorithm.bytes().filter(|&b| b != b'\n').collect();

    let mut image = Image::default();
    for (y, line) in image_string.lines().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            image.insert((x as i64, y as i64), ch);
        }
    }
    (algorithm, image)
}

fn run(input: &str, iterations: i64) -> usize {
    let (algorithm, mut image) = parse(input);
    for i in 0..iterations {
        image = get_next_image(&image, &algorithm, i);
    }
    image.values().filter(|&&v| v == b'#').count()
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Some(run(input, 2).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Some(run(input, 50).to_string())
    }
}
