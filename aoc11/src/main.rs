use std::env;
use std::fs;

type Pos = (usize, usize);

fn read_galaxies(contents: &str) -> Vec<Pos> {
    contents
        .lines()
        .enumerate()
        .flat_map(|(r, line)|
                  line.chars().enumerate()
                  .filter_map(move |(c, ch)|
                              match ch {
                                  '.' => None,
                                  '#' => Some((r, c)),
                                  _ => panic!("bad space"),
                              }))
        .collect()
}

fn compute_stretch<'a>(
    factor: usize, galaxies: &'a Vec<Pos>, select: impl Fn(&'a Pos) -> usize)
    -> Vec<usize> {
    let len = 1 + galaxies.iter().map(&select).max().unwrap_or(0);
    (0..len).scan(0, |stretch, i| {
        let s = *stretch;
        let is_empty = galaxies.iter().all(|p| select(p) != i);
        if is_empty { *stretch += factor-1 }
        Some(s)
    }).collect()
}

fn expand(galaxies: Vec<Pos>, stretch_factor: usize) -> Vec<Pos> {
    let row_stretch = compute_stretch(stretch_factor, &galaxies, |&(r, _)| r);
    let col_stretch = compute_stretch(stretch_factor, &galaxies, |&(_, c)| c);
    galaxies.iter().map(|&(r, c)| (r + row_stretch[r], c + col_stretch[c])).collect()
}

fn distance(x: Pos, y: Pos) -> usize {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

fn total_distance(expanded: &Vec<Pos>) -> usize {
    let len = expanded.len();
    (0..len)
        .fold(0, |isum, i|
              (i+1..len).fold(isum, |jsum, j|
                              jsum + distance(expanded[i], expanded[j])))
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
    let stretch_factor: usize =
        if let Some(arg) = args.next() {
            arg.parse().expect("stretch factor")
        } else {
            2
        };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let galaxies = read_galaxies(&contents);
    let expanded_galaxies = expand(galaxies, stretch_factor);
    let total = total_distance(&expanded_galaxies);

    println!("{total}")
}
