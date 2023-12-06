use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::cmp::max;

fn num_winning_inputs(time_limit: u64, previous_max: u64) -> i64 {
    let tl = time_limit as f64;
    let tl2 = tl / 2.0;
    let pm = previous_max as f64;
    let d = tl2 * tl2 - pm - 1.0;
    if d < 0.0 { 0 }
    else {
        let r = d.sqrt();
        let x1 = (tl2 - r).ceil() as i64;
        let x2 = (tl2 + r).floor() as i64;
        max(x2 - x1 + 1, 0)
    }
}

fn main() {
    if let [_, file_path] = env::args().collect::<Vec<_>>().as_slice() {

        let path = Path::new(file_path);
        let file = File::open(&path).expect("open file");
        let reader = BufReader::new(file);

        let mut times = Vec::new();
        let mut distances = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result.expect("line");
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["Time:", times_str @ ..] =>
                    times = times_str.iter().map(|s| s.parse().unwrap()).collect(),
                ["Distance:", distances_str @ ..] =>
                    distances = distances_str.iter().map(|s| s.parse().unwrap()).collect(),
                _ => panic!("invalid input"),
            }
        }

        let result =
            times.iter().zip(distances.iter())
            .map(|(&t, &d)| num_winning_inputs(t, d))
            .fold(1, |x, y| x * y);

        println!("{result}");

    } else {
        panic!("file path argument");
    }
}
