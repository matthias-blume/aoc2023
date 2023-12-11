use std::env;
use std::fs;

type Pos = (usize, usize);

fn read_unexpanded_line(r: usize, line: &str, set: &mut Vec<Pos>) -> bool {
    let mut c = 0;
    let mut non_empty = false;
    for ch in line.chars() {
        match ch {
            '.' => (),
            '#' => { set.push((r, c)); non_empty = true },
            _ => panic!("bad space"),
        }
        c += 1
    }
    non_empty
}

fn read_unexpanded_lines(contents: &str, set: &mut Vec<Pos>) {
    let mut r = 0;
    for line in contents.lines() {
        if read_unexpanded_line(r, line, set) {
            r += 1;
        } else {
            r += 1000000;
        }
    }
}

fn col_is_empty(col: usize, set: &Vec<Pos>) -> bool {
    set.iter().all(|&(_, c)| c != col)
}

fn col_stretch(ncols: usize, set: &Vec<Pos>) -> Vec<usize> {
    let mut stretch = 0;
    let mut v = Vec::new();
    for c in 0..ncols {
        v.push(stretch);
        if col_is_empty(c, set) {
            stretch += 999999
        }
    }   
    v
}

fn expand(unexpanded: Vec<Pos>) -> Vec<Pos> {
    let ncols = unexpanded.iter().fold(0, |nc, &(_, c)| if c > nc { c } else { nc }) + 1;
    let stretch = col_stretch(ncols, &unexpanded);
    unexpanded.iter().map(|&(r, c)| (r, c + stretch[c])).collect()
}

fn manhattan_distance(x: Pos, y: Pos) -> usize {
    (if x.0 < y.0 { y.0 - x.0 } else { x.0 - y.0 })
        + (if x.1 < y.1 { y.1 - x.1 } else { x.1 - y.1 })
}

fn total_distance(expanded: &Vec<Pos>) -> usize {
    let len = expanded.len();
    (0..len).map(|i| (i+1..len).map(|j| manhattan_distance(expanded[i], expanded[j])).sum::<usize>()).sum()
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

    let mut unexpanded = Vec::new();
    read_unexpanded_lines(&contents, &mut unexpanded);
    let expanded = expand(unexpanded);
    let total = total_distance(&expanded);
    
    println!("{total}")
}
