//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/25
use crate::util::Day;

pub struct D25;

impl Day for D25 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut sea: Vec<Vec<u8>> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.bytes().collect())
            .collect();

        let h = sea.len();
        let w = sea[0].len();

        let mut step = 0;
        loop {
            step += 1;
            let mut moved = false;

            // East-facing herd ('>')
            let mut next_sea = sea.clone();
            for y in 0..h {
                for x in 0..w {
                    if sea[y][x] == b'>' {
                        let nx = (x + 1) % w;
                        if sea[y][nx] == b'.' {
                            next_sea[y][nx] = b'>';
                            next_sea[y][x] = b'.';
                            moved = true;
                        }
                    }
                }
            }
            sea = next_sea;

            // South-facing herd ('v')
            let mut next_sea = sea.clone();
            for y in 0..h {
                for x in 0..w {
                    if sea[y][x] == b'v' {
                        let ny = (y + 1) % h;
                        if sea[ny][x] == b'.' {
                            next_sea[ny][x] = b'v';
                            next_sea[y][x] = b'.';
                            moved = true;
                        }
                    }
                }
            }
            sea = next_sea;

            if !moved {
                break;
            }
        }

        Some(step.to_string())
    }
}
