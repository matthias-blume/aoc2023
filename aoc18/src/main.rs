// Advent-of-Code 2023
// Day 18
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::BTreeMap;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    NE,
    SE,
    NW,
    SW,
    NS,
}
use Tile::*;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
use Direction::*;

impl Direction {
    fn from(s: &str) -> Self {
        match s {
            "R" => East,
            "L" => West,
            "U" => North,
            "D" => South,
            _ => panic!("bad direction"),
        }
    }
}

type Row = BTreeMap<i64, Tile>;
type Board = BTreeMap<i64, Row>;
type Pos = (i64, i64);

struct Step(Direction, i64);

impl Step {
    fn from_color(col: &str) -> Self {
        let dir =
            match &col[7..8] {
                "0" => East,
                "1" => South,
                "2" => West,
                "3" => North,
                _ => panic!("bad encoded color")
            };
        let dist = i64::from_str_radix(&col[2..7], 16).expect("encoded steps");
        Step(dir, dist)
    }
    
    fn from(line: &str) -> Self {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            [dir, dist, col] =>
                //Step(Direction::from(dir), dist.parse::<i64>().expect("distance")),
                Step::from_color(col),
            _ => panic!("bad step"),
        }
    }

    fn next(&self, (i, j): Pos) -> Pos {
        match self {
            Step(North, d) => (i - d, j),
            Step(South, d) => (i + d, j),
            Step(West, d) => (i, j - d),
            Step(East, d) => (i, j + d),
        }
    }
    
    fn to_board(steps: &Vec<Self>) -> Board {
        let initial = (0, 0);
        let mut board: Board = BTreeMap::new();
        let mut prev_d = steps[steps.len() - 1].0;
        let mut cur = initial;
        for step in steps.iter() {
            let prev = cur;
            cur = step.next(prev);
            let corner = match (prev_d, step.0) {
                (South, East) | (West, North) => NE,
                (South, West) | (East, North) => NW,
                (North, East) | (West, South) => SE,
                (North, West) | (East, South) => SW,
                _ => panic!("not a corner"),
            };
            if let Some(ref mut row) = board.get_mut(&prev.0) {
                row.insert(prev.1, corner);
            } else {
                let mut row = BTreeMap::new();
                row.insert(prev.1, corner);
                board.insert(prev.0, row);
            }
            prev_d = step.0;
        }
        board
    }
}

fn register_ns(row: &mut Row, j: &i64) {
    if !row.contains_key(j) {
        row.insert(*j, NS);
    }
}

// Calculate area inside loop on an individual row.
// Insert NS tiles into next row.
fn row_area(row: &Row) -> (i64, Row) {
    let mut skipped_row = BTreeMap::new();
    let area = row.keys()
        .fold((false, false, 0, 0), |(inside, was_north, n, j0), j| {
            let tile = row.get(j).expect("tile");
            match tile {
                NS => {
                    register_ns(&mut skipped_row, j);
                    (!inside, false, if inside { n + (j - j0) } else { n + 1 }, *j)
                },
                NE => (inside, true, if inside { n + (j - j0) } else { n + 1 }, *j),   // L---
                SE => {
                    register_ns(&mut skipped_row, j);
                    (inside, false, if inside { n + (j - j0) } else { n + 1 }, *j)
                },  // F---
                NW => (was_north == inside, false, n + (j - j0), *j),  // L---J  vs. F---J
                SW => {
                    register_ns(&mut skipped_row, j);
                    (was_north != inside, false, n + (j - j0), *j)
                },  // F---7  vs. L---7
            }
        })
        .2;
    (area, skipped_row)
}

// Count area inside the loop on a LoopBoard.
fn board_area(board: &Board) -> i64 {
    let mut prev_key = -1;
    let mut skipped_row = BTreeMap::new();
    let mut area = 0;
    for key in board.keys() {
        let num_skipped = key - prev_key - 1;
        if num_skipped > 0 {
            let line_area = row_area(&skipped_row).0;
            eprintln!("{num_skipped} * {line_area}");
            area += num_skipped * line_area;
        }
        let row = board.get(key).expect("row");
        for (&k, &v) in row.into_iter() {
            skipped_row.insert(k, v);
        }
        let (row_area, new_skipped) = row_area(&skipped_row);
        eprintln!("{key}: {row_area}");
        area += row_area;
        skipped_row = new_skipped;
        prev_key = *key;
    }
    area
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

    let steps = contents.lines().map(Step::from).collect();
    let board = Step::to_board(&steps);
    let area = board_area(&board);
    
    println!("{area}");
}
