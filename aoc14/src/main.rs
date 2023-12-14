use std::env;
use std::fs;
use std::iter;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Item {
    Nothing,
    Square,
    Round,
}
use crate::Item::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum TiltAxis { Hor, Vert }
use crate::TiltAxis::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction { Up, Down }
use crate::Direction::*;

impl Direction {
    fn increment(self: Self) -> i32 {
        match self { Down => 1, Up => -1 }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct RowSummary(u8, Vec<u8>);

impl RowSummary {
    fn for_row(r: u8, row: &Vec<Item>) -> Option<Self> {
        let v = row.iter().enumerate()
            .filter_map(|(c, &item)| if item == Round { Some(c as u8) } else { None })
            .collect::<Vec<_>>();
        if v.len() > 0 { Some(RowSummary(r, v)) } else { None }
    }
}

struct Board(Vec<Vec<Item>>);

struct FreePos(Vec<i32>);

impl FreePos {
    fn set_square_at(self: &mut Self, i: usize, j: usize, direction: Direction) {
        self.0[j] = i as i32 + direction.increment()
    }

    fn slide_round_at(self: &mut Self, j: usize, direction: Direction) -> i32 {
        let new_i = self.0[j];
        self.0[j] = new_i + direction.increment();
        new_i
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Summary(Vec<RowSummary>);

impl Board {
    fn dims(self: &Self) -> (usize, usize) {
        let nrows = self.0.len();
        (nrows, if nrows == 0 { 0 } else { self.0[0].len() })
    }

    fn freepos(self: &Self, axis: TiltAxis, direction: Direction) -> FreePos {
        let (nrows, ncols) = self.dims();
        let (sz, end) = match axis { Hor => (ncols, nrows), Vert => (nrows, ncols) };
        let init = match direction { Down => 0, Up => end as i32 - 1 };
        FreePos(iter::repeat(init).take(sz).collect())
    }

    fn at<'a>(self: &'a mut Self, i: usize, j: usize, axis: TiltAxis) -> &'a mut Item {
        match axis { Hor => &mut self.0[i][j], Vert => &mut self.0[j][i] }
    }

    fn tilt(self: &mut Self, axis: TiltAxis, direction: Direction) {
        let (nrows, ncols) = self.dims();
        let mut freepos = self.freepos(axis, direction);
        let (iend, jend) = match axis { Hor => (nrows, ncols), Vert => (ncols, nrows) };
        let mut j_iteration = |i|
            for j in 0..jend {
                match self.at(i, j, axis) {
                    Nothing => (),
                    Square => freepos.set_square_at(i, j, direction),
                    Round => {
                        let new_i = freepos.slide_round_at(j, direction);
                        *self.at(i, j, axis) = Nothing;
                        *self.at(new_i as usize, j, axis) = Round
                    }
                }
            };
        match direction {
            Down => for i in 0..iend { j_iteration(i) },
            Up => for i in (0..iend).rev() { j_iteration(i) },
        }
    }

    fn cycle(self: &mut Self) {
        self.tilt(Hor, Down); // north
        self.tilt(Vert, Down); // west
        self.tilt(Hor, Up); // south
        self.tilt(Vert, Up); // east
    }

    fn weight(self: &Self) -> usize {
        let nrows = self.0.len();
        self.0
            .iter()
            .enumerate()
            .map(|(r, row)| (nrows - r) * row.iter().filter(|&item| item == &Round).count())
            .sum()
    }

    fn summarize(self: &Self) -> Summary {
        Summary(self.0.iter().enumerate()
                .filter_map(|(r, row)| RowSummary::for_row(r as u8, row))
                .collect())
    }

    fn ncycle(self: &mut Self, n: u64) {
        let mut history = HashMap::new();
        for round in 0..n {
            let summary = self.summarize();
            if let Some(previous_round) = history.get(&summary) {
                let cycle_length = round - previous_round;
                let remaining = (n-previous_round) % cycle_length;
                for _ in 0..remaining {
                    self.cycle();
                }
                return
            } else {
                history.insert(summary, round);
                self.cycle()
            }
        }
    }
}

fn read_row(line: &str) -> Vec<Item> {
    let mut v = Vec::new();
    for c in line.chars() {
        match c {
            '.' => v.push(Nothing),
            '#' => v.push(Square),
            'O' => v.push(Round),
            _ => panic!("bad item"),
        }
    }
    v
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

    let mut v = Vec::new();
    for line in contents.lines() {
        v.push(read_row(line))
    }

    let mut board = Board(v.clone());
    board.tilt(Hor, Down);
    let total = board.weight();
    println!("part1: {total}");

    let mut board = Board(v);
    board.ncycle(1000000000);
    let total = board.weight();
    
    println!("part2: {total}");
}
