//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2018-19/tree/master/13
use crate::util::Day;

pub struct D13;

// possible cart moves, indexed by direction: 0=>, 1=v, 2=<, 3=^
const STEPS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// A grid cell holds either a track character or a cart (by its index).
#[derive(Clone, Copy)]
enum Cell {
    Track(char),
    Cart(usize),
}

#[derive(Clone)]
struct Cart {
    x: i64,
    y: i64,
    dir: i64,
    cross: i64,
    under: char, // the track piece the cart is standing on
    alive: bool,
}

// Parse the map into carts and a grid. The grid stores track characters at
// cart positions (the assumed underlying track), matching the Python which
// keeps the direction symbols on the grid initially but records the
// underlying track in cart[4].
fn parse(input: &str) -> (Vec<Cart>, Vec<Vec<Cell>>) {
    let data: Vec<Vec<char>> = input
        .split('\n')
        .map(|l| l.trim_end_matches('\r').chars().collect())
        .collect();

    let mut carts: Vec<Cart> = Vec::new();
    let mut grid: Vec<Vec<Cell>> = data
        .iter()
        .map(|row| row.iter().map(|&c| Cell::Track(c)).collect())
        .collect();

    for (i, row) in data.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let cart = match c {
                '>' => Some((0i64, '-')),
                'v' => Some((1i64, '|')),
                '<' => Some((2i64, '-')),
                '^' => Some((3i64, '|')),
                _ => None,
            };
            if let Some((dir, under)) = cart {
                carts.push(Cart {
                    x: j as i64,
                    y: i as i64,
                    dir,
                    cross: 0,
                    under,
                    alive: true,
                });
                // grid keeps the direction symbol char here initially
                grid[i][j] = Cell::Track(c);
            }
        }
    }

    (carts, grid)
}

// Order matching Python's heap on [x, y, ...]: sort by x, then y.
fn sorted_indices(carts: &[Cart]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..carts.len()).filter(|&i| carts[i].alive).collect();
    idx.sort_by(|&a, &b| (carts[a].x, carts[a].y).cmp(&(carts[b].x, carts[b].y)));
    idx
}

// Apply the track-piece logic (turn / intersection) to a cart's direction,
// based on the piece it is currently standing on (`under`).
fn turn(cart: &mut Cart) {
    match cart.under {
        '+' => {
            cart.dir = (cart.dir + cart.cross % 3 + 3) % 4;
            cart.cross += 1;
        }
        '/' => {
            cart.dir += if cart.dir == 0 || cart.dir == 2 { 3 } else { 1 };
            cart.dir %= 4;
        }
        '\\' => {
            cart.dir += if cart.dir == 0 || cart.dir == 2 { 1 } else { 3 };
            cart.dir %= 4;
        }
        _ => {}
    }
}

impl Day for D13 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut carts, mut grid) = parse(input);

        loop {
            for ci in sorted_indices(&carts) {
                // examine the piece that the cart is standing on
                let under = carts[ci].under;
                if under == '+' || under == '/' || under == '\\' {
                    turn(&mut carts[ci]);
                } else if under != '-' && under != '|' {
                    // collision (standing on a cart cell)
                    return Some(format!("{},{}", carts[ci].x, carts[ci].y));
                }

                // place the piece the cart was standing on back on the grid
                let (ox, oy) = (carts[ci].x, carts[ci].y);
                grid[oy as usize][ox as usize] = Cell::Track(carts[ci].under);

                // drive the cart in its direction
                let step = STEPS[carts[ci].dir as usize];
                carts[ci].x += step.0;
                carts[ci].y += step.1;

                // read what we are now standing on; if it is a cart, this is a
                // collision that will be flagged when this cart is next examined
                let (nx, ny) = (carts[ci].x, carts[ci].y);
                match grid[ny as usize][nx as usize] {
                    Cell::Track(c) => carts[ci].under = c,
                    Cell::Cart(_) => carts[ci].under = '#', // marker: not track
                }
                // put the cart on the map so the spot is marked occupied
                grid[ny as usize][nx as usize] = Cell::Cart(ci);
            }
        }
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut carts, mut grid) = parse(input);

        loop {
            for ci in sorted_indices(&carts) {
                if !carts[ci].alive {
                    continue;
                }

                let under = carts[ci].under;
                if under == '+' || under == '/' || under == '\\' {
                    turn(&mut carts[ci]);
                }

                // place the piece the cart was standing on back on the grid
                let (ox, oy) = (carts[ci].x, carts[ci].y);
                grid[oy as usize][ox as usize] = Cell::Track(carts[ci].under);

                // drive the cart in its direction
                let step = STEPS[carts[ci].dir as usize];
                carts[ci].x += step.0;
                carts[ci].y += step.1;

                let (nx, ny) = (carts[ci].x, carts[ci].y);
                // if the cell we moved onto holds a cart, remove both
                if let Cell::Cart(other) = grid[ny as usize][nx as usize] {
                    // restore the track the other cart was holding
                    grid[ny as usize][nx as usize] = Cell::Track(carts[other].under);
                    carts[other].alive = false;
                    carts[ci].alive = false;
                    continue;
                }

                // put the cart on the map
                if let Cell::Track(c) = grid[ny as usize][nx as usize] {
                    carts[ci].under = c;
                }
                grid[ny as usize][nx as usize] = Cell::Cart(ci);
            }

            // if there is only one cart left
            let alive: Vec<usize> = (0..carts.len()).filter(|&i| carts[i].alive).collect();
            if alive.len() == 1 {
                let c = &carts[alive[0]];
                return Some(format!("{},{}", c.x, c.y));
            }
        }
    }
}
