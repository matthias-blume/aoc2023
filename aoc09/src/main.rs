use std::env;
use std::fs;

type LR = (i64, i64);

fn extrapolate(v: Vec<i64>) -> LR {
    if v.iter().all(|&x| x == 0) {
        (0, 0)
    } else {
        let (l, r) = extrapolate(v.iter().zip(v[1..].iter()).map(|(x0, x1)| x1 - x0).collect());
        (v[0] - l, v[v.len()-1] + r)
    }
}

fn componentwise_add(x: LR, y: LR) -> LR { (x.0 + y.0, x.1 + y.1) }

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

    let (total_l, total_r) = contents
        .lines()
        .fold((0, 0),
              |accu, line| componentwise_add(
                  accu,
                  extrapolate(
                      line.split_whitespace()
                          .map(|x| x.parse().unwrap())
                          .collect())));

    println!("{total_l} {total_r}");
}
