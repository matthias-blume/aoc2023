use std::env;
use std::fs;
use std::collections::HashMap;

// Splits input line into the map part (a String) and the vector of
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

// Can a group with the given length fit onto the beginning of s?
fn can_fit_group(s: &[u8], len: usize) -> bool {
    s[..len].iter().all(|&c| c != b'.')  // don't sit on '.'
        && s.get(len) != Some(&b'#')      // don't abut '#' on right
}

// Counts in how many ways the groups in v can fit onto s, using the
// memoized version for recursive calls.
fn cnt(s: &[u8], v: &[u64], memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    match v {
        [] => { // No more groups.  Make sure that tail can be empty.
            return if can_be_empty(s) { 1 } else { 0 }
        },
        [vfirst, vrest @ ..] => {
            let g = *vfirst as usize;
            // Minimum space needed for groups in v, including gaps.
            let remaining = v.iter().sum::<u64>() as usize + vrest.len();
            let maxpos = s.len() - remaining;  // max start pos for first group
            (0..=maxpos)
                .map(|p|
                     if can_be_empty(&s[..p]) && can_fit_group(&s[p..], g) {
                         let nextpos = s.len().min(p + g + 1);
                         cnt_memo(&s[nextpos..], vrest, memo)
                     } else { 0 })
                .sum()
        }
    }
}

// Memoized cnt().
fn cnt_memo(s: &[u8], v: &[u64], memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    let key = (v.len(), s.len());
    if let Some(&n) = memo.get(&key) { n }
    else {
        let n = cnt(s, v, memo);
        memo.insert(key, n);
        n
    }
}

// Counts in how many ways the groups in v can fit onto s.
fn count(s: &[u8], v: &Vec<u64>) -> u64 {
    cnt_memo(s, &v[..], &mut HashMap::new())
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
        _ => panic!("{}: no input file name", program),
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
