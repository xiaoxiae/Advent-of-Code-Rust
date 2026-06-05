//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/20
use crate::util::Day;

pub struct D20;

type Image = Vec<String>;

/// Parse the whole input into a list of (id, image) tiles.
fn parse(input: &str) -> Vec<(u64, Image)> {
    let mut images = Vec::new();
    for block in input.trim().split("\n\n") {
        let lines: Vec<&str> = block.split('\n').collect();
        // "Tile 2647:" -> 2647
        let id: u64 = lines[0]
            .split(' ')
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let image: Image = lines[1..].iter().map(|s| s.to_string()).collect();
        images.push((id, image));
    }
    images
}

/// Yield the 8 possible symmetric variants of the image.
fn yield_symmetric_images(image: &Image) -> Vec<Image> {
    let mut result = Vec::with_capacity(8);
    for &h in &[true, false] {
        // horizontal
        for &v in &[true, false] {
            // vertical
            for &d in &[true, false] {
                // diagonal
                let mut new_image: Image = image.clone();

                if v {
                    new_image.reverse();
                }

                if h {
                    new_image = new_image
                        .iter()
                        .map(|row| row.chars().rev().collect::<String>())
                        .collect();
                }

                if d {
                    let n = new_image.len();
                    let cols: Vec<Vec<char>> =
                        new_image.iter().map(|s| s.chars().collect()).collect();
                    new_image = (0..n)
                        .map(|r| (0..n).map(|c| cols[c][r]).collect::<String>())
                        .collect();
                }

                result.push(new_image);
            }
        }
    }
    result
}

/// Return true if image i2 can be below i1.
fn down(i1: &Image, i2: &Image) -> bool {
    i1[i1.len() - 1] == i2[0]
}

/// Return true if image i2 can be to the right of i1.
fn right(i1: &Image, i2: &Image) -> bool {
    let a: String = i1.iter().map(|row| row.chars().last().unwrap()).collect();
    let b: String = i2.iter().map(|row| row.chars().next().unwrap()).collect();
    a == b
}

type Cell = (u64, Image);

fn fill(
    square: &mut Vec<Vec<Option<Cell>>>,
    images: &[(u64, Image)],
    used: &mut Vec<bool>,
    current: usize,
) -> bool {
    let side = square.len();
    if current == side * side {
        return true;
    }

    let x = current % side;
    let y = current / side;

    for (idx, (id, image)) in images.iter().enumerate() {
        if used[idx] {
            continue;
        }
        let id = *id;

        for symmetric_image in yield_symmetric_images(image) {
            let ok_down = y == 0 || {
                let above = square[y - 1][x].as_ref().unwrap();
                down(&above.1, &symmetric_image)
            };
            let ok_right = x == 0 || {
                let left = square[y][x - 1].as_ref().unwrap();
                right(&left.1, &symmetric_image)
            };

            if ok_down && ok_right {
                square[y][x] = Some((id, symmetric_image.clone()));
                used[idx] = true;
                if fill(square, images, used, current + 1) {
                    return true;
                }
                used[idx] = false;
                square[y][x] = None;
            }
        }
    }
    false
}

fn contains_monster(waters: &[Vec<char>], x: usize, y: usize, monster: &[&str]) -> bool {
    for (yd, row) in monster.iter().enumerate() {
        for (xd, ch) in row.chars().enumerate() {
            if ch == ' ' {
                continue;
            }
            if waters[y + yd][x + xd] != '#' {
                return false;
            }
        }
    }
    true
}

fn count_monsters(waters: &[Vec<char>]) -> usize {
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let w = monster[0].len();
    let h = monster.len();

    let mut total = 0;
    for y in 0..(waters.len() - h) {
        for x in 0..(waters[0].len() - w) {
            if contains_monster(waters, x, y, &monster) {
                total += 1;
            }
        }
    }
    total
}

impl Day for D20 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        // The original 20-1.py quits after printing the number of tiles.
        let images = parse(input);
        Some(images.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let images = parse(input);
        let side = (images.len() as f64).sqrt() as usize;

        let mut square: Vec<Vec<Option<Cell>>> = vec![vec![None; side]; side];
        let mut used = vec![false; images.len()];
        fill(&mut square, &images, &mut used, 0);

        let tile_size = square[0][0].as_ref().unwrap().1.len();
        let inner = tile_size - 2;

        // Build the assembled image with borders stripped.
        let water_side = side * inner;
        let mut waters: Vec<String> = vec![String::new(); water_side];

        for (i, row_of_tiles) in square.iter().enumerate() {
            for cell in row_of_tiles {
                let b = &cell.as_ref().unwrap().1;
                for (j, row) in b[1..b.len() - 1].iter().enumerate() {
                    let stripped: String = row[1..row.len() - 1].to_string();
                    waters[i * inner + j].push_str(&stripped);
                }
            }
        }

        let waters_img: Image = waters.clone();

        for sym in yield_symmetric_images(&waters_img) {
            let grid: Vec<Vec<char>> = sym.iter().map(|s| s.chars().collect()).collect();
            let result = count_monsters(&grid);

            if result != 0 {
                let total = waters
                    .iter()
                    .flat_map(|row| row.chars())
                    .filter(|&c| c == '#')
                    .count();
                return Some((total - 15 * result).to_string());
            }
        }

        None
    }
}
