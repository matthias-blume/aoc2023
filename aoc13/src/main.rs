use std::env;
use std::fs;

type Row = Vec<bool>;
type Pattern = Vec<Row>;
type Coords = (usize, usize);

// Identity function on coordinates represents "horizontal".
// To get vertical behavior, use function that swaps coordinates.
type Direction = dyn Fn(Coords) -> Coords;

// Reads pattern dimensions (assuming that all rows are of the same size).
fn dims(p: &Pattern) -> Coords {
    let r = p.len();
    (r, if r > 0 { p[0].len() } else { 0 })
}

// Access the given coordinates in a pattern, considering the direction.
fn access(p: &Pattern, coords: Coords, direction: &Direction) -> bool {
    let (r, c) = direction(coords);
    p[r][c]
}

// Calculates the number of defects ("smudges") that account for
// a mirroring line at m (= row or column, depending on direction).
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

// Scores a mirror line at m.  The score is m if the actual number of smudges
// matches the expected number.  Otherwise the score is 0.
fn mirror_score(p: &Pattern, m: usize, defects: usize, direction: &Direction) -> usize {
    if mirror_defects(p, m, direction) == defects { m } else { 0 }
}

// Sum of scores of all possible reflection lines, given the expected number
// of defects (smudges).
fn reflection_score(p: &Pattern, defects: usize) -> usize {
    let (nrows, ncols) = dims(p);
    let hor: &Direction = &|d| d;
    let vert: &Direction = &|(r, c)| (c, r);
    100 * (1..nrows).map(|i| mirror_score(p, i, defects, hor)).sum::<usize>()
        + (1..ncols).map(|j| mirror_score(p, j, defects, vert)).sum::<usize>()
}

// Reads a Row.  '#' is true, '.' (and everything else) is false.
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
        _ => panic!("{}: no input file name", program),
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

    total += reflection_score(&pattern, defects);
    
    println!("{total}");
}
