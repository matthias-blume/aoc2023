use std::env;
use std::fs;
use std::collections::HashMap;

fn step<'a>(d: char, state: &'a String, directions: &'a HashMap<String, (String, String)>) -> &'a String {
    let (l, r) = directions.get(state).unwrap();
    match d {
        'L' => l,
        'R' => r,
        _ => panic!("bad turn")
    }
}

fn all_steps<'a>(rl: &String, initial: &'a String, directions: &'a HashMap<String, (String, String)>) -> &'a String {
    let mut state = initial;
    for d in rl.chars() {
        state = step(d, state, directions)
    }
    state
}

fn transition<'a>(state: &String, transitions: &'a HashMap<String, String>) -> &'a String {
    transitions.get(state).unwrap()
}

fn ends_with(state: &String, c: char) -> bool {
    state.chars().nth(2).unwrap() == c
}

fn big_transition<'a>(big_state: &Vec<&String>, transitions: &'a HashMap<String, String>) -> Vec<&'a String> {
    big_state.iter().map(|s| transition(s, transitions)).collect()
}

fn calc_steps(initial: &String, transitions: &HashMap<String, String>) -> u64 {
    let mut state = initial;
    let mut n = 0;
    while !ends_with(&state, 'Z') {
        state = transition(state, transitions);
        n += 1;
    }
    n
}

fn is_big_end_state(state: &Vec<&String>) -> bool {
    state.iter().all(|s| ends_with(s, 'Z'))
}

fn count_big_steps(initial: &Vec<&String>, transitions: &HashMap<String, String>) -> u64 {
    let mut state = initial.clone();
    let mut n = 0;
    while !is_big_end_state(&state) {
        state = big_transition(&state, transitions);
        n += 1;
        if n % 1000000 == 0 {
            eprint!(".");
        }
    }
    n
}

fn initial_big_state<'a>(directions: &'a HashMap<String, (String, String)>) -> Vec<&'a String> {
    directions.iter().filter_map(|d| if ends_with(d.0, 'A') { Some(d.0) } else { None }).collect()
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
        _ => panic!("{}: no input file name", program),
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

    let rllen = rl.len() as u64;
    let transitions = directions.iter().map(|d| (d.0.to_string(), all_steps(&rl, d.0, &directions).to_string())).collect::<HashMap<_, _>>();
    
    let result = directions
        .iter()
        .map(|d| d.0)
        .filter(|x| ends_with(x, 'A'))
        .map(|s| calc_steps(s, &transitions))
        .fold(1, lcm)
        * rllen;
  
    println!("lucky guess {result}");

    let big_start = initial_big_state(&directions);
    let big_result = count_big_steps(&big_start, &transitions) * rllen;

    println!("brute force: {big_result}");
    
}
