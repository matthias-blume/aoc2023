// Advent-of-Code 2023
// Day 17
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Board {
    height: usize,
    width: usize,
    m: Vec<Vec<u64>>,
}

impl Board {
    fn new(m: Vec<Vec<u64>>) -> Self {
        Board {
            height: m.len(),
            width: if m.len() > 0 { m[0].len() } else { 0 },
            m: m,
        }
    }

    fn read_line(line: &str) -> Vec<u64> {
        line.chars().map(|c| c.to_digit(10).expect("digit") as u64).collect()
    }

    fn from(input: &str) -> Self {
        Self::new(input.lines().map(Self::read_line).collect())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

type State = (usize, usize, Direction, usize);
type Memo = HashMap<State, u64>;
type Work = VecDeque<State>;

fn navigate(board: &Board, memo: &mut Memo, work: &mut Work, result: &mut u64)  {
    while let Some(state @ (i, j, _, _)) = work.pop_front() {
        let &loss = memo.get(&state).unwrap();
        if loss >= *result { continue }
        if i == board.height - 1 && j == board.width - 1 {
            *result = loss;
            continue;
        } else {
            try_schedule(board, state, loss, memo, work);
        }
    }
}

fn schedule(board: &Board, state: State, loss: u64, memo: &mut Memo, work: &mut Work) {
    let (i, j, _, _) = state;
    let loss = loss + board.m[i][j];
    let &prev = memo.get(&state).unwrap_or(&std::u64::MAX);
    if prev > loss {
        memo.insert(state, loss);
        work.push_back(state);
    }
}

fn try_schedule(board: &Board, state: State, loss: u64, memo: &mut Memo, work: &mut Work) {
    let (i, j, d, dsteps) = state;
    if d != Down && i > 0 && (d != Up || dsteps < 10) && (d == Up || dsteps >= 4) {
        schedule(board, (i - 1, j, Up, if d == Up { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Up && i < board.height - 1 && (d != Down || dsteps < 10) && (d == Down || dsteps >= 4)  {
        schedule(board, (i + 1, j, Down, if d == Down { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Right && j > 0 && (d != Left || dsteps < 10) && (d == Left || dsteps >= 4) {
        schedule(board, (i, j - 1, Left, if d == Left { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Left && j < board.width - 1 && (d != Right || dsteps < 10) && (d == Right || dsteps >= 4) {
        schedule(board, (i, j + 1, Right, if d == Right { dsteps + 1 } else { 1 }), loss, memo, work);
    }
}

fn find_best(board: &Board) -> u64 {
    let mut result = std::u64::MAX;
    let mut memo = HashMap::new();
    let mut work = VecDeque::new();
    let initial = (0, 0, Right, 0);
    memo.insert(initial, 0);
    work.push_back(initial);
    navigate(&board, &mut memo, &mut work, &mut result);
    result
}

fn main() {
    let mut args = env::args();
    let program = match args.next() {
        Some(arg) => arg,
        _ => panic!("no program name"),
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        _ => panic!("{}: no input file name", program),
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let board = Board::from(&contents);
    let result = find_best(&board);

    println!("{result}");
}
