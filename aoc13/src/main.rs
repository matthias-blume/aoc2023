use std::env;
use std::fs;

type Row = Vec<bool>;
type Pattern = Vec<Row>;
type Coords = (usize, usize);
type Direction = dyn Fn(Coords) -> Coords;

fn dims(p: &Pattern) -> Coords { (p.len(), p[0].len()) }

fn access(p: &Pattern, coords: Coords, direction: &Direction) -> bool {
    let (r, c) = direction(coords);
    p[r][c]
}

fn mirror_defects(p: &Pattern, m: usize, direction: &Direction) -> usize {
    let (xsz, ysz) = direction(dims(p));
    let m2 = 2 * m;
    let start = m2.max(xsz) - xsz;
    (start..m)
        .map(|i|
             (0..ysz)
             .filter(|&j| access(p, (i, j), direction) !=
                     access(p, (m2 - i - 1, j), direction))
             .count())
        .sum()
}

fn mirror_score(p: &Pattern, m: usize, defects: usize, direction: &Direction) -> usize {
    if mirror_defects(p, m, direction) == defects { m } else { 0 }
}

fn reflection_score(p: &Pattern, defects: usize) -> usize {
    let (nrows, ncols) = dims(p);
    let hor: &Direction = &|d| d;
    let vert: &Direction = &|(r, c)| (c, r);
    100 * (1..nrows).map(|i| mirror_score(p, i, defects, hor)).sum::<usize>()
        + (1..ncols).map(|j| mirror_score(p, j, defects, vert)).sum::<usize>()
}

fn read_row(line: &str) -> Row {
    line.chars().map(|c| c == '#').collect()
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
    let defects = match args.next() {
        Some(arg) => arg.parse().expect("number of smudges"),
        None => 0
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut total = 0;
    let mut pattern = Vec::new();

    for line in contents.lines() {
        if line.len() == 0 {
            total += reflection_score(&pattern, defects);
            pattern = Vec::new();
        } else {
            pattern.push(read_row(line))
        }
    }

    if pattern.len() > 0 {
        total += reflection_score(&pattern, defects);
    }
    
    println!("{total}");
}
