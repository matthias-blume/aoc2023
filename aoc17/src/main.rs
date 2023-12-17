use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Board {
    height: usize,
    width: usize,
    losses: Vec<Vec<u64>>,
}

impl Board {
    fn new(losses: Vec<Vec<u64>>) -> Self {
        Board {
            height: losses.len(),
            width: if losses.len() > 0 { losses[0].len() } else { 0 },
            losses,
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

enum Part {
    One,
    Two,
}

impl Part {
    fn permits(&self, old_d: Direction, new_d: Direction, dsteps: usize) -> bool {
        match self {
            Part::One => old_d != new_d || dsteps < 3,
            Part::Two => (old_d != new_d || dsteps < 10) && (old_d == new_d || dsteps >= 4),
        }
    }
}

fn navigate(board: &Board, memo: &mut Memo, work: &mut Work, result: &mut u64, part: &Part)  {
    while let Some(state @ (i, j, _, _)) = work.pop_front() {
        let &loss = memo.get(&state).unwrap();
        if loss >= *result { continue }
        if i == board.height - 1 && j == board.width - 1 {
            *result = loss;
            continue;
        } else {
            try_schedule(board, state, loss, memo, work, part);
        }
    }
}

fn schedule(board: &Board, state: State, loss: u64, memo: &mut Memo, work: &mut Work) {
    let (i, j, _, _) = state;
    let loss = loss + board.losses[i][j];
    let &prev = memo.get(&state).unwrap_or(&std::u64::MAX);
    if prev > loss {
        memo.insert(state, loss);
        work.push_back(state);
    }
}

fn try_schedule(board: &Board, state: State, loss: u64, memo: &mut Memo, work: &mut Work, part: &Part) {
    let (i, j, d, dsteps) = state;
    if d != Down && i > 0 && part.permits(Up, d, dsteps) {
        schedule(board, (i - 1, j, Up, if d == Up { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Up && i < board.height - 1 && part.permits(Down, d, dsteps) {
        schedule(board, (i + 1, j, Down, if d == Down { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Right && j > 0 && part.permits(Left, d, dsteps) {
        schedule(board, (i, j - 1, Left, if d == Left { dsteps + 1 } else { 1 }), loss, memo, work);
    }
    if d != Left && j < board.width - 1 && part.permits(Right, d, dsteps) {
        schedule(board, (i, j + 1, Right, if d == Right { dsteps + 1 } else { 1 }), loss, memo, work);
    }
}

fn find_best(board: &Board, part: &Part) -> u64 {
    let mut result = std::u64::MAX;
    let mut memo = HashMap::new();
    let mut work = VecDeque::new();
    let initial = (0, 0, Right, 0);
    memo.insert(initial, 0);
    work.push_back(initial);
    navigate(&board, &mut memo, &mut work, &mut result, part);
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
    let part1 = find_best(&board, &Part::One);
    let part2 = find_best(&board, &Part::Two);

    println!("part 1: {part1}, part2 : {part2}");
}
