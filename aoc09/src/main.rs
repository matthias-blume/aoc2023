// Advent-of-Code 2023
// Day 09
// Author: Matthias Blume

use std::env;
use std::fs;

struct LR(i64, i64);

impl std::iter::Sum for LR {
    fn sum<I: Iterator<Item = LR>>(iter: I) -> LR {
        iter.fold(LR(0, 0), |x, y| LR(x.0+y.0, x.1+y.1))
    }
}

fn extrapolate(v: Vec<i64>) -> LR {
    if v.iter().all(|&x| x == 0) {
        LR(0, 0)
    } else {
        let last_i = v.len()-1;
        let LR(l, r) = extrapolate((0..last_i).map(|i| v[i+1]-v[i]).collect());
        LR(v[0] - l, v[last_i] + r)
    }
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

    let LR(total_l, total_r) = contents
        .lines()
        .map(|line| extrapolate(line.split_whitespace().map(|x| x.parse().unwrap()).collect()))
        .sum();

    println!("{total_l} {total_r}");
}
