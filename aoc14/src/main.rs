use std::env;
use std::fs;
use std::collections::HashMap;

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

    fn tilt<'a>(self: &'a mut Self, axis: TiltAxis, direction: Direction) -> &'a mut Self {
        let iend = match axis { Hor => self.nrows, Vert => self.ncols };
        let jend = match axis { Hor => self.ncols, Vert => self.nrows };
        for j in 0..jend {
            let mut free = match direction { Down => 0, Up => iend as i32 - 1 };
            let mut adjust_position = |i, j, increment| match self.at(i, j, axis) {
                Nothing => (),
                Square => free = i as i32 + increment,
                Round => {
                    let new_i = free;
                    free = new_i + increment;
                    *self.at(i, j, axis) = Nothing;
                    *self.at(new_i as usize, j, axis) = Round;
                },
            };
            match direction {
                Down => for i in 0..iend { adjust_position(i, j, 1); },
                Up => for i in (0..iend).rev() { adjust_position(i, j, -1); },
            }
        }
        self
    }
    
    fn cycle<'a>(self: &'a mut Self) -> &'a mut Self {
        self.tilt(Hor, Down); // north
        self.tilt(Vert, Down); // west
        self.tilt(Hor, Up); // south
        self.tilt(Vert, Up) // east
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

    fn ncycle<'a>(self: &'a mut Self, n: u64) -> &'a mut Self {
        let mut history = HashMap::new();
        for i in 0..n {
            let summary = self.summarize();
            if let Some(prev_i) = history.get(&summary) {
                let remaining = (n - prev_i) % (i - prev_i);
                for _ in 0..remaining { self.cycle(); }
                return self
            } else {
                history.insert(summary, i);
                self.cycle();
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
