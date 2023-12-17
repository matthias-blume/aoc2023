use std::env;
use std::fs;

use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Copy, Clone)]
enum Part {
    One,
    Two,
}

impl Part {
    fn permits(self, old: Direction, new: Direction, dsteps: usize) -> bool {
        match self {
            Part::One =>
                old != new || dsteps < 3,
            Part::Two =>
                (old != new || dsteps < 10) && (old == new || dsteps >= 4),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    i: usize,
    j: usize,
    dinfo: Option<(Direction, usize)>,
}

impl State {
    fn next(&self, d: Direction, board: &Board, part: Part) -> Option<Self> {
        let dinfo =
            if let Some((old_d, dsteps)) = self.dinfo {
                if old_d == d.reverse() { return None }
                if !part.permits(old_d, d, dsteps) { return None }
                let dsteps = if old_d == d { dsteps + 1 } else { 1 };
                Some((d, dsteps))
            } else {
                Some((d, 1))
            };
        match d {
            Up => (self.i > 0)
                .then(|| State{ i: self.i - 1, j: self.j, dinfo }),
            Down => (self.i < board.height - 1)
                .then(|| State{ i: self.i + 1, j: self.j, dinfo }),
            Left => (self.j > 0)
                .then(|| State{ i: self.i, j: self.j - 1, dinfo }),
            Right => (self.j < board.width - 1)
                .then(|| State{ i: self.i, j: self.j + 1, dinfo }),
        }
    }
}

struct Board {
    height: usize,
    width: usize,
    losses: Vec<Vec<u64>>,
}

impl Board {
    fn new(losses: Vec<Vec<u64>>) -> Self {
        let height = losses.len();
        Board{ height,
               width: if height > 0 { losses[0].len() } else { 0 },
               losses,
        }
    }

    fn read_line(line: &str) -> Vec<u64> {
        line.chars().map(|c| c.to_digit(10).expect("digit") as u64).collect()
    }

    fn from(input: &str) -> Self {
        Self::new(input.lines().map(Self::read_line).collect())
    }

    fn with_loss(&self, state: State) -> (State, u64) {
        (state, self.losses[state.i][state.j])
    }
    
    fn successors(&'_ self, state: State, part: Part) -> impl IntoIterator<Item = (State, u64)> + '_ {
        vec![Up, Down, Left, Right]
            .into_iter()
            .filter_map(
                move |d| state.next(d, self, part)
                    .map(|state| self.with_loss(state)))
    }

    fn is_goal(&self, state: &State) -> bool {
        state.i == self.height - 1 && state.j == self.width - 1
    }
}

fn find_best(board: &Board, part: Part) -> u64 {
    dijkstra(&State{ i: 0, j: 0, dinfo: None },
             |&state| board.successors(state, part),
             |state| board.is_goal(state))
        .unwrap().1
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
    let part1 = find_best(&board, Part::One);

    println!("part 1: {part1}");

    let part2 = find_best(&board, Part::Two);
    println!("part2 : {part2}");
}
