use std::env;
use std::fs;
use std::collections::HashMap;

fn calc_steps(rl: &String, initial: &String, directions: &HashMap<String, (String, String)>) -> u64 {
    let mut state = initial;
    let mut n = 0;
    while state.chars().nth(2).unwrap() != 'Z' {
        for d in rl.chars() {
            let (l, r) = directions.get(state).unwrap();
            match d {
                'L' => state = l,
                'R' => state = r,
                _ => panic!("bad direction")
            }
            n += 1
        }
    }
    n
}

fn gcd(x0: u64, y0: u64) -> u64 {
    let (mut x, mut y) = (x0, y0);
    if x > y { (x, y) = (y, x) }
    while x > 0 {
        (x, y) = (y % x, x)
    }
    y
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
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

    let mut rl = String::from("");
    let mut directions = HashMap::new();

    for line in contents.lines() {
        match line.split("=").collect::<Vec<_>>().as_slice() {
            [""] => (),
            [word] => { rl = word.to_string() },
            [] => (),
            [lhs, rhs] => {
                if rhs.len() != 11 { panic!("bad rhs") }
                match rhs[2..10].split(",").collect::<Vec<_>>().as_slice() {
                    [l, r] => { directions.insert(lhs[0..3].to_string(), (l[0..3].to_string(), r[1..4].to_string())); },
                    _ => { panic!("expected (l, r), found {}", rhs) },
                }
            },
            _ => { panic!("bad input: {}", line) },
        }
    }

    let result = directions
        .iter()
        .map(|d| d.0)
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .map(|s| calc_steps(&rl, s, &directions))
        .fold(1, lcm);
  
    println!("{result}");
}
