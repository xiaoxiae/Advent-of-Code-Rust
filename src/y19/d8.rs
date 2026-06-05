//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/08
use crate::util::Day;

pub struct D8;

const W: usize = 25;
const H: usize = 6;

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let pixels: Vec<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut min_zeroes = usize::MAX;
        let mut min_zeroes_value = 0;

        for layer in pixels.chunks(W * H) {
            let zeroes = layer.iter().filter(|&&p| p == 0).count();
            if zeroes < min_zeroes {
                let ones = layer.iter().filter(|&&p| p == 1).count();
                let twos = layer.iter().filter(|&&p| p == 2).count();
                min_zeroes_value = ones * twos;
                min_zeroes = zeroes;
            }
        }

        Some(min_zeroes_value.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let pixels: Vec<u32> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        // image[y][x] is a stack of layer values
        let mut image: Vec<Vec<Vec<u32>>> = vec![vec![Vec::new(); W]; H];

        let mut i = 0;
        while i < pixels.len() {
            let mut y = 0;
            while y < H {
                let mut x = 0;
                while x < W {
                    image[y][x].push(pixels[i]);
                    i += 1;
                    x += 1;
                }
                y += 1;
            }
        }

        let mut out = String::new();
        for y in 0..H {
            for x in 0..W {
                if let Some(&p) = image[y][x].iter().find(|&&p| p != 2) {
                    out.push(if p != 0 { 'O' } else { ' ' });
                }
            }
            out.push('\n');
        }

        Some(out)
    }
}
