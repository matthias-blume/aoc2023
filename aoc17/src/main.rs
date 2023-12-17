use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::{PartialOrd,Ord,Reverse};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
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
struct Limits {
    at_least: Option<usize>,
    at_most: Option<usize>,
}

impl Limits {
    fn permit(self, old: Direction, new: Direction, dsteps: usize) -> bool {
        if let Some(at_least) = self.at_least {
            if old != new && dsteps < at_least { return false };
        }
        if let Some(at_most) = self.at_most {
            if old == new && dsteps >= at_most { return false };
        }
        true
    }
}
            
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct State {
    i: usize,
    j: usize,
    dinfo: Option<(Direction, usize)>,
}

impl State {
    fn next(&self, d: Direction, board: &Board, limits: Limits) -> Option<Self> {
        let dinfo =
            if let Some((old_d, dsteps)) = self.dinfo {
                if old_d == d.reverse() { return None }
                if !limits.permit(old_d, d, dsteps) { return None }
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
    
    fn successors(&'_ self, state: State, limits: Limits)
                  -> impl IntoIterator<Item = (State, u64)> + '_ {
        vec![Up, Down, Left, Right]
            .into_iter()
            .filter_map(
                move |d| state.next(d, self, limits)
                    .map(|state| self.with_loss(state)))
    }

    fn is_goal(&self, state: State) -> bool {
        state.i == self.height - 1 && state.j == self.width - 1
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct WorkItem {
    best: u64,
    state: State,
}

// Dijkstra using a BinaryHeap in lieu of a proper priority queue.
fn find_best(board: &Board, limits: Limits) -> u64 {
    let mut best_so_far = HashMap::new();
    let mut heap = BinaryHeap::new();
    let initial = State{ i: 0, j: 0, dinfo: None };
    best_so_far.insert(initial, 0);
    heap.push(Reverse(WorkItem{ best: 0, state: initial }));
    while let Some(Reverse(cur)) = heap.pop() {
        let &cur_best = best_so_far.get(&cur.state).unwrap_or(&std::u64::MAX);
        if board.is_goal(cur.state) {
            return cur_best
        }
        if cur_best < cur.best { continue }; // because not a real PQ
        for (new_state, loss) in board.successors(cur.state, limits) {
            let &orig = best_so_far.get(&new_state).unwrap_or(&std::u64::MAX);
            let new = cur_best + loss;
            if new < orig {
                best_so_far.insert(new_state, new);
                heap.push(Reverse(WorkItem{ state: new_state, best: new }));
            }
        }
    }
    std::u64::MAX
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
    let part1 =
        find_best(&board, Limits{ at_least: None, at_most: Some(3) });
    let part2 =
        find_best(&board, Limits{ at_least: Some(4), at_most: Some(10) });

    println!("part 1: {part1}, part2 : {part2}");
}
