use std::env;
use std::fs;
use std::collections::HashMap;

// Splits an input line into the map part (String) and the vector of
// group lengths.
fn read_line(line: &str) -> (String, Vec<u64>) {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [s, nums] =>
            (s.to_string(),
             nums.split(",")
             .map(|x| x.parse().expect("group length"))
             .collect()),
        _ => panic!("bad input line"),
    }
}

// Unfolds the input map string and the group vector "factor" times.
fn unfold(s: String, v: Vec<u64>, factor: u32) -> (String, Vec<u64>) {
    let mut ss = s.to_owned();
    let mut vv = v.to_owned();
    for _ in 0 .. factor - 1 {
        ss.push_str("?");
        ss.push_str(&s);
        vv.extend(&v);
    }
    (ss, vv)
}

// Checks that s does not contain any '#'.
fn can_be_empty(s: &[u8]) -> bool {
    s.iter().all(|&c| c != b'#')
}

// Can a group fit on s on the interval [start, end)?
fn can_fit_group(s: &[u8], start: usize, end: usize) -> bool {
    (start .. end).all(|i| s[i] != b'.')         // don't sit on '.'
        && (start == 0 || s[start - 1] != b'#')  // don't abut '#' on left
        && (end >= s.len() || s[end] != b'#')    // don't abut '#' on right
}

// Counts in how many ways the groupsin v can fit on s,
// starting from position pos.
fn cnt(s: &[u8], v: &[u64], pos: usize,
       memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    match v {
        [] => { // No more groups.  Make sure that tail can be empty.
            return if pos > s.len() || can_be_empty(&s[pos..]) { 1 } else { 0 }
        },
        [vfirst, vrest @ ..] => {
            let g = *vfirst as usize;
            // Minimum space needed for groups in v, including gaps.
            let remaining = v.iter().sum::<u64>() as usize + vrest.len();
            let maxpos = s.len() - remaining;  // max start pos for first group
            (pos..=maxpos)
                .map(|p|
                     if can_be_empty(&s[pos..p]) && can_fit_group(s, p, p + g) {
                         cnt_memo(s, vrest, p + g + 1, memo)
                     } else { 0 })
                .sum()
        }
    }
}

// Memoized cnt().
fn cnt_memo(s: &[u8], v: &[u64], pos: usize,
            memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    let key = (v.len(), pos);
    if let Some(&n) = memo.get(&key) { n }
    else {
        let n = cnt(s, v, pos, memo);
        memo.insert(key, n);
        n
    }
}

// How many ways can the groups in v fit on s?
fn count(s: &[u8], v: &Vec<u64>) -> u64 {
    cnt_memo(s, &v[..], 0, &mut HashMap::new())
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
