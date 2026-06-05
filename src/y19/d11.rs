//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/11
use crate::util::Day;
use crate::y19_intcode::{Intcode, Step};
use rustc_hash::FxHashMap;

pub struct D11;

/// Run the painting robot. `start_color` is the color of the initial panel (0, 0).
/// Returns the map of painted panels keyed by (x, y).
fn run_robot(input: &str, start_color: i64) -> FxHashMap<(i64, i64), i64> {
    let mut c = Intcode::from_input(input);

    let mut area: FxHashMap<(i64, i64), i64> = FxHashMap::default();
    if start_color != 0 {
        area.insert((0, 0), start_color);
    }

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    // part 1 starts facing direction 0; part 2 starts at direction 1.
    let mut direction: i64 = if start_color != 0 { 1 } else { 0 };
    let mut paint_move_toggle = true; // whether to paint or to move the robot

    loop {
        match c.run() {
            Step::NeedInput => {
                let color = *area.get(&(x, y)).unwrap_or(&0);
                c.input(color);
            }
            Step::Output(v1) => {
                if paint_move_toggle {
                    area.insert((x, y), v1);
                } else {
                    direction += if v1 == 0 { 1 } else { -1 };

                    match direction.rem_euclid(4) {
                        0 => y += 1,
                        1 => x -= 1,
                        2 => y -= 1,
                        3 => x += 1,
                        _ => unreachable!(),
                    }
                }

                paint_move_toggle = !paint_move_toggle;
            }
            Step::Halt => break,
        }
    }

    area
}

impl Day for D11 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let area = run_robot(input, 0);
        Some(area.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let area = run_robot(input, 1);

        let min_x = area.keys().map(|&(x, _)| x).min().unwrap();
        let max_x = area.keys().map(|&(x, _)| x).max().unwrap();
        let min_y = area.keys().map(|&(_, y)| y).min().unwrap();
        let max_y = area.keys().map(|&(_, y)| y).max().unwrap();

        let mut result = String::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let c = *area.get(&(x, y)).unwrap_or(&0);
                result.push(if c == 0 { ' ' } else { '#' });
            }
            result.push('\n');
        }

        Some(result)
    }
}
