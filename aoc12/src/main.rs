use std::env;
use std::fs;
use std::collections::HashMap;

// Splits an input line into the map part (String) and the vector of
// group lengths.
fn read_line(line: &str) -> (String, Vec<u64>) {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [s, nums] => (s.to_string(), nums.split(",").map(|x| x.parse().expect("group length")).collect()),
        _ => panic!("bad input line"),
    }
}

// Unfolds the input map string and the group vector "factor" times.
fn unfold(s: String, v: Vec<u64>, factor: u32) -> (String, Vec<u64>) {
    let mut ss = s.to_owned();
    let mut vv = v.to_owned();
    for _ in 0..factor-1 {
        ss.push_str("?");
        ss.push_str(&s);
        vv.extend(&v);
    }
    (ss, vv)
}

// Can the line s be empty on the interval [start, end)?
fn can_be_empty(s: &[u8], start: usize, end: usize) -> bool {
    for i in start..end {
        if s[i] == b'#' { return false }
    }
    true
}

// Can a group fit on s on the interval [start, end)?
fn can_fit_group(s: &[u8], start: usize, end: usize) -> bool {
    for i in start..end {
        if s[i] == b'.' { return false }  // would sit on '.'
    }
    if start > 0 && s[start-1] == b'#' { return false }  // abuts '#' on left
    if end < s.len() && s[end] == b'#' { return false }  // abuts '#' on right
    true
}

// Consider v[i..].  Counts in how many ways these groups can fit on s, starting
// from position pos.
fn cnt(s: &[u8], v: &Vec<u64>, i: usize, pos: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    let remaining = v[i..].iter().sum::<u64>() as usize;
    if i >= v.len() {
        return if can_be_empty(s, pos, s.len()) { 1 } else { 0 }
    }
    let maxpos = s.len() - remaining - (v.len() - i - 1);
    let group = v[i] as usize;
    let mut n = 0;
    for p in pos..=maxpos {
        if can_be_empty(s, pos, p) && can_fit_group(s, p, p+group) {
            n += cnt_memo(s, v, i+1, p+group+1, memo)
        }
    }
    n
}

// Memoized cnt().
fn cnt_memo(s: &[u8], v: &Vec<u64>, i: usize, pos: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&n) = memo.get(&(i, pos)) {
        n
    } else {
        let n = cnt(s, v, i, pos, memo);
        memo.insert((i, pos), n);
        n
    }
}

// How many ways can the groups in v fit on s?
fn count(s: &[u8], v: &Vec<u64>) -> u64 {
    cnt_memo(s, v, 0, 0, &mut HashMap::new())
}

// Read the line, unfold, and count.
fn count_line(line: &str, factor: u32) -> u64 {
    let (s, v) = read_line(line);
    let (s, v) = unfold(s, v, factor);
    count(s.as_bytes(), &v)
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
    let factor = match args.next() {
        Some(arg) => arg.parse().expect("unfold factor"),
        None => 1,
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let n: u64 = contents.lines().map(|line| count_line(line, factor)).sum();

    println!("sum is {n}");
}
