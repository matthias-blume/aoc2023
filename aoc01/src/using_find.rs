use std::env;
use std::fs;

const DIGITS: &[(&str, u32)] = &[
    ("0", 0),
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9)];

fn first_digit(line: &str) -> u32 {
    let mut idx: usize = std::usize::MAX;
    let mut val: u32 = 0;
    for (s, v) in DIGITS {
        match line.find(s) {
            None => (),
            Some(i) => { if i <= idx { idx = i; val = *v } },
        }
    }
    val
}

fn last_digit(line: &str) -> u32 {
    let mut idx: usize = 0;
    let mut val: u32 = 0;
    for (s, v) in DIGITS {
        match line.rfind(s) {
            None => (),
            Some(i) => { if i >= idx { idx = i; val = *v } },
        }
    }
    val
}

fn line_value(line: &str) -> u32 {
    10 * first_digit(line) + last_digit(line)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut tally: u32 = 0;

    for line in contents.lines() {
        tally += line_value(line)
    }
    println!("tally is {tally}");
}
