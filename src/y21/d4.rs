//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2021/tree/master/04
use crate::util::Day;

pub struct D4;

type Board = Vec<Vec<Option<i64>>>;

fn parse(input: &str) -> (Vec<i64>, Vec<Board>) {
    let text = input.trim();
    let blocks: Vec<&str> = text.split("\n\n").collect();

    let numbers: Vec<i64> = blocks[0]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for block in &blocks[1..] {
        let mut b: Board = Vec::new();
        for line in block.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let row: Vec<Option<i64>> = line
                .split_whitespace()
                .map(|x| Some(x.parse().unwrap()))
                .collect();
            b.push(row);
        }
        boards.push(b);
    }

    (numbers, boards)
}

fn mark_board(board: &mut Board, number: i64) {
    for line in board.iter_mut() {
        for cell in line.iter_mut() {
            if *cell == Some(number) {
                *cell = None;
                return;
            }
        }
    }
}

fn completed(board: &Board) -> bool {
    for i in 0..board.len() {
        let mut marked = true;
        for j in 0..board[i].len() {
            if board[i][j].is_some() {
                marked = false;
            }
        }
        if marked {
            return true;
        }
    }

    for i in 0..board[0].len() {
        let mut marked = true;
        for j in 0..board.len() {
            if board[j][i].is_some() {
                marked = false;
            }
        }
        if marked {
            return true;
        }
    }

    false
}

fn board_sum(board: &Board) -> i64 {
    let mut total = 0;
    for i in 0..board[0].len() {
        for j in 0..board.len() {
            if let Some(v) = board[i][j] {
                total += v;
            }
        }
    }
    total
}

impl Day for D4 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (numbers, mut boards) = parse(input);

        for number in numbers {
            for board in boards.iter_mut() {
                mark_board(board, number);
                if completed(board) {
                    return Some((board_sum(board) * number).to_string());
                }
            }
        }

        None
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (numbers, mut boards) = parse(input);

        let mut completed_boards = vec![false; boards.len()];

        for number in numbers {
            for i in 0..boards.len() {
                mark_board(&mut boards[i], number);
                if completed(&boards[i]) {
                    completed_boards[i] = true;

                    if completed_boards.iter().all(|&c| c) {
                        return Some((board_sum(&boards[i]) * number).to_string());
                    }
                }
            }
        }

        None
    }
}
