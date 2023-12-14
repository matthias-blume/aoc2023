use std::env;
use std::fs;
use std::collections::HashMap;
use num;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Item {
    Nothing,
    Square,
    Round,
}
use crate::Item::*;

#[derive(Copy, Clone)]
enum TiltAxis { Hor, Vert }
use crate::TiltAxis::*;

#[derive(Copy, Clone)]
enum Direction { Up, Down }
use crate::Direction::*;

#[derive(PartialEq, Eq, Hash)]
struct RowSummary(u8, Vec<u8>);

impl RowSummary {
    fn for_row(r: u8, row: &Vec<Item>) -> Option<Self> {
        let v = row.iter().enumerate()
            .filter_map(|(c, &item)| if item == Round { Some(c as u8) } else { None })
            .collect::<Vec<_>>();
        if v.len() > 0 { Some(RowSummary(r, v)) } else { None }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Summary(Vec<RowSummary>);

struct Board {
    nrows: usize,
    ncols: usize,
    items: Vec<Vec<Item>>,
}

impl Board {
    fn new(items: Vec<Vec<Item>>) -> Board {
        Board {
            nrows: items.len(),
            ncols: if items.len() > 0 { items[0].len() } else { 0 },
            items: items,
        }
    }

    fn at<'a>(self: &'a mut Self, i: usize, j: usize, axis: TiltAxis) -> &'a mut Item {
        match axis { Hor => &mut self.items[i][j], Vert => &mut self.items[j][i] }
    }

    fn tilt(mut self: Self, axis: TiltAxis, direction: Direction) -> Self {
        let iend = match axis { Hor => self.nrows, Vert => self.ncols };
        let jend = match axis { Hor => self.ncols, Vert => self.nrows };
        let (istart, istop, increment) =
            match direction {
                Down => (0, iend as i32 - 1, 1),
                Up => (iend as i32 - 1, 0, -1),
            };
        for j in 0..jend {
            let mut free = istart;
            for i in num::range_step_inclusive(istart, istop, increment) {
                match self.at(i as usize, j, axis) {
                    Nothing => (),
                    Square => free = i + increment,
                    Round => {
                        let new_i = free;
                        free = new_i + increment;
                        *self.at(i as usize, j, axis) = Nothing;
                        *self.at(new_i as usize, j, axis) = Round;
                    },
                }
            }
        }
        self
    }
    
    fn cycle(self: Self) -> Self {
        self.tilt(Hor, Down) // north
            .tilt(Vert, Down) // west
            .tilt(Hor, Up) // south
            .tilt(Vert, Up) // east
    }

    fn weight(self: &Self) -> usize {
        self.items
            .iter()
            .enumerate()
            .map(|(r, row)|
                 (self.nrows - r)
                 * row.iter().filter(|&item| item == &Round).count())
            .sum()
    }

    fn summarize(self: &Self) -> Summary {
        Summary(self.items.iter().enumerate()
                .filter_map(|(r, row)| RowSummary::for_row(r as u8, row))
                .collect())
    }

    fn ncycle(mut self: Self, n: u64) -> Self {
        let mut history = HashMap::new();
        for i in 0..n {
            let summary = self.summarize();
            if let Some(prev_i) = history.get(&summary) {
                let remaining = (n - prev_i) % (i - prev_i);
                for _ in 0..remaining { self = self.cycle(); }
                return self
            } else {
                history.insert(summary, i);
                self = self.cycle();
            }
        }
        self
    }
}

fn read_row(line: &str) -> Vec<Item> {
    line.chars()
        .map(|c| match c {
            '.' => Nothing,
            '#' => Square,
            'O' => Round,
            _ => panic!("bad item"),
        })
        .collect()
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

    let part1 = Board::new(v.clone()).tilt(Hor, Down).weight();
    let part2 = Board::new(v).ncycle(1000000000).weight();
    
    println!("part1: {part1}, part2: {part2}");
}
