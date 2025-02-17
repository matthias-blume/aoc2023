// Advent-of-Code 2023
// Day 06
// Author: Matthias Blume

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

use util::iter::*;

fn num_winning_inputs((&time_limit, &previous_max): (&f64, &f64)) -> f64 {
    let tl2 = time_limit / 2.0;
    let r = (tl2 * tl2 - previous_max - 1.0).sqrt();
    (tl2 + r).floor() - (tl2 - r).ceil() + 1.0
}

fn main() {
    if let [_, file_path] = &env::args().boxed()[..] {

        let path = Path::new(file_path);
        let file = File::open(&path).expect("open file");
        let reader = BufReader::new(file);

        let mut times = Vec::new();
        let mut distances = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result.expect("line");
            match &line.split_whitespace().boxed()[..] {
                ["Time:", times_str @ ..] =>
                    times = times_str.iter().map(|s| s.parse().unwrap()).collect(),
                ["Distance:", distances_str @ ..] =>
                    distances = distances_str.iter().map(|s| s.parse().unwrap()).collect(),
                _ => panic!("invalid input"),
            }
        }

        let result: f64 =
            times.iter().zip(distances.iter())
            .map(num_winning_inputs)
            .product();

        println!("{result}");

    } else {
        panic!("file path argument");
    }
}
