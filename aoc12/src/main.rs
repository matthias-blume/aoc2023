use std::env;
use std::fs;
use std::sync::mpsc;
use std::collections::HashMap;

fn read_line(line: &str) -> (String, Vec<u64>) {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
        [s, nums] => (s.to_string(), nums.split(",").map(|x| x.parse().expect("group length")).collect()),
        _ => panic!("bad input line"),
    }
}

fn unfold(s: String, v: Vec<u64>) -> (String, Vec<u64>) {
    let mut ss = s.to_owned();
    let mut vv = v.to_owned();
    for _ in 0..4 {
        ss.push_str("?");
        ss.push_str(&s);
        vv.extend(&v);
    }
    (ss, vv)
}

fn can_be_empty(s: &[u8], start: usize, end: usize) -> bool {
    for i in start..end {
        if s[i] == b'#' { return false }
    }
    true
}

fn can_fit_group(s: &[u8], start: usize, end: usize) -> bool {
    if end > s.len() { return false }
    for i in start..end {
        if s[i] == b'.' { return false }
    }
    if start > 0 && s[start-1] == b'#' { return false }
    if end < s.len() && s[end] == b'#' { return false }
    true
}

fn cnt(s: &[u8], v: &Vec<u64>, i: usize, pos: usize, remaining: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&n) = memo.get(&(i, pos)) {
        return n
    }
    if i >= v.len() {
        if can_be_empty(s, pos, s.len()) {
            memo.insert((i, pos), 1);
            return 1
        } else {
            memo.insert((i, pos), 0);
            return 0
        }
    }
    let maxpos = s.len() - remaining - (v.len() - i - 1);
    let group = v[i] as usize;
    let mut n = 0;
    for p in pos..=maxpos {
        if can_be_empty(s, pos, p) && can_fit_group(s, p, p+group) {
            n += cnt(s, v, i+1, p+group+1, remaining-group, memo)
        }
    }
    memo.insert((i, pos), n);
    n
}

fn count(s: &[u8], v: &Vec<u64>) -> u64 {
    cnt(s, v, 0, 0, (v.iter().sum::<u64>()) as usize, &mut HashMap::new())
}

fn count_line(line: &str) -> u64 {
    let (s, v) = read_line(line);
    let (s, v) = unfold(s, v);
    count(s.as_bytes(), &v)
}

fn count_lines(lines: &Vec<String>) -> u64 {
    lines.iter().map(|s| count_line(&s)).sum()
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

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    let mut lines: Vec<String> = Vec::new();

    for line in contents.lines() {
        lines.push(line.to_string());
        if lines.len() >= 1 {
            let tlines = lines;
            let ttx = tx.clone();
            handles.push(std::thread::spawn(move || {
                ttx.send(count_lines(&tlines)).unwrap()
            }));
            lines = Vec::new();
        }
    }
    let tlines = lines;
    handles.push(std::thread::spawn(move || {
        tx.send(count_lines(&tlines)).unwrap()
    }));
    
    let mut n = 0;
    let mut i = 0;
    for x in rx {
        n += x;
        i += 1;
        println!("..{i}");
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("sum is {n}");
}
