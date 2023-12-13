use std::env;
use std::fs;

#[derive(Eq, PartialEq)]
enum Spot {
    Ash,
    Rock,
}

use crate::Spot::*;

type Row = Vec<Spot>;
type Pattern = Vec<Row>;

fn row_defects(p: &Pattern, i: usize, j: usize) -> usize {
    let nrows = p.len();
    if j >= nrows { return 0 }
    let ncols = p[0].len();
    (0..ncols).filter(|&c| p[i][c] != p[j][c]).count()
}

fn col_defects(p: &Pattern, i: usize, j: usize) -> usize {
    let nrows = p.len();
    let ncols = p[0].len();
    if j >= ncols { return 0 }
    (0..nrows).filter(|&r| p[r][i] != p[r][j]).count()
}

fn hor_mirror_defects(p: &Pattern, mrow: usize) -> usize {
    let mrow2 = 2 * mrow;
    (0..mrow).map(|i| row_defects(p, i, mrow2 - i - 1)).sum()
}

fn vert_mirror_defects(p: &Pattern, mcol: usize) -> usize {
    let mcol2 = 2 * mcol;
    (0..mcol).map(|j| col_defects(p, j, mcol2 - j - 1)).sum()
}

fn find_reflection(p: &Pattern, defects: usize) -> usize {
    let nrows = p.len();
    let ncols = p[0].len();
    let mut total = 0;
    for i in 0..nrows {
        if hor_mirror_defects(p, i) == defects {
            total += 100 * i
        }
    }
    for j in 0..ncols {
        if vert_mirror_defects(p, j) == defects {
            total += j
        }
    }
    total
}

fn read_row(line: &str) -> Row {
    line.chars().map(|c| match c {
        '#' => Rock,
        '.' => Ash,
        _ => panic!("bad spot"),
    }).collect()
}

fn main() {
    let mut args = env::args();
    let program = match args.next() {
        Some(arg) => arg,
        _ => panic!("no program name"),
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        _ => panic!("{}: no program name", program),
    };
    let defects = match args.next() {
        Some(arg) => arg.parse().expect("number of smudges"),
        None => 0
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut total = 0;
    let mut pattern = Vec::new();

    for line in contents.lines() {
        if line.len() == 0 {
            total += find_reflection(&pattern, defects);
            pattern = Vec::new();
        } else {
            pattern.push(read_row(line))
        }
    }

    if pattern.len() > 0 {
        total += find_reflection(&pattern, defects);
    }
    
    println!("{total}");
}
