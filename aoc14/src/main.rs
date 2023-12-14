use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Item {
    Nothing,
    Square,
    Round,
}
use crate::Item::*;

type Board = Vec<Vec<Item>>;

fn new_fp(n: usize, init: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(init);
    }
    v
}

fn tilt_north(b: &mut Board) -> usize {
    let nrows = b.len();
    let ncols = b[0].len();
    let mut fp = new_fp(ncols, 0);
    let mut weight = 0;
    for r in 0..nrows {
        for c in 0..ncols {
            match b[r][c] {
                Nothing => (),
                Square => fp[c] = r as i32 + 1,
                Round => {
                    let new_r = fp[c];
                    b[r][c] = Nothing;
                    b[new_r as usize][c] = Round;
                    fp[c] = new_r + 1;
                    weight += nrows - new_r as usize;
                }
            }
        }
    }
    weight
}

fn tilt_west(b: &mut Board) {
    let nrows = b.len();
    let ncols = b[0].len();
    let mut fp = new_fp(nrows, 0);
    for c in 0..ncols {
        for r in 0..nrows {
            match b[r][c] {
                Nothing => (),
                Square => fp[r] = c as i32 + 1,
                Round => {
                    let new_c = fp[r];
                    b[r][c] = Nothing;
                    b[r][new_c as usize] = Round;
                    fp[r] = new_c + 1;
                }
            }
        }
    }
}

fn tilt_south(b: &mut Board) {
    let nrows = b.len();
    let ncols = b[0].len();
    let mut fp = new_fp(ncols, nrows as i32 - 1);
    for r in (0..nrows).rev() {
        for c in 0..ncols {
            match b[r][c] {
                Nothing => (),
                Square => fp[c] = r as i32 - 1,
                Round => {
                    let new_r = fp[c];
                    b[r][c] = Nothing;
                    b[new_r as usize][c] = Round;
                    fp[c] = new_r - 1;
                }
            }
        }
    }
}

fn tilt_east(b: &mut Board) -> usize {
    let nrows = b.len();
    let ncols = b[0].len();
    let mut fp = new_fp(nrows, ncols as i32 - 1);
    let mut weight = 0;
    for c in (0..ncols).rev() {
        for r in 0..nrows {
            match b[r][c] {
                Nothing => (),
                Square => fp[r] = c as i32 - 1,
                Round => {
                    let new_c = fp[r];
                    b[r][c] = Nothing;
                    b[r][new_c as usize] = Round;
                    fp[r] = new_c - 1;
                    weight += nrows - r;
                }
            }
        }
    }
    weight
}

fn cycle(b: &mut Board) -> usize {
    tilt_north(b);
    tilt_west(b);
    tilt_south(b);
    tilt_east(b)
}

#[derive(Hash, Eq, PartialEq)]
struct RowSummary(u8, Vec<u8>);

#[derive(Hash, Eq, PartialEq)]
struct Summary(Vec<RowSummary>);

fn summarize_row(r: u8, row: &Vec<Item>) -> Option<RowSummary> {
    let mut v = Vec::new();
    for c in 0..row.len() {
        if row[c] == Round {
            v.push(c as u8);
        }
    }
    if v.len() > 0 { Some(RowSummary(r, v)) } else { None }
}

fn summarize(b: &Board) -> Summary {
    Summary(b.iter().enumerate().filter_map(|(r, row)| summarize_row(r as u8, row)).collect())
}

fn ncycle(b: &mut Board, n: u64) -> usize {
    let mut history = HashMap::new();
    for round in 0..n-1 {
        let summary = summarize(b);
        if let Some(previous_round) = history.get(&summary) {
            let cycle_length = round - previous_round;
            let remaining = (n-1-previous_round) % cycle_length;
            for _ in 0..remaining {
                cycle(b);
            }
            return cycle(b)
        } else {
            history.insert(summary, round);
            cycle(b);
        }
    }
    cycle(b)
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

    let mut board = Vec::new();
    for line in contents.lines() {
        board.push(read_row(line))
    }
    
    let total = ncycle(&mut board, 1000000000);
    
    println!("{total}");
}
