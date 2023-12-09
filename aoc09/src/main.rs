use std::env;
use std::fs;

fn extrapolate(v: Vec<i64>) -> (i64, i64) {
    if v.iter().all(|&x| x == 0) {
        (0, 0)
    } else {
        let (l, r) = extrapolate(v.iter().zip(v[1..].iter()).map(|(x0, x1)| x1 - x0).collect());
        (v.first().unwrap() - l, v.last().unwrap() + r)
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
        _ => panic!("{}: no program name", program),
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut total_r = 0;
    let mut total_l = 0;
    
    for line in contents.lines() {
        let (l, r) = extrapolate(line.split_whitespace().map(|x| x.parse().unwrap()).collect());
        total_l += l;
        total_r += r;
    }

    println!("{total_l} {total_r}");
}
