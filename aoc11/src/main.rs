// Advent-of-Code 2023
// Day 11
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;

type Pos = (usize, usize);
type HalfTransform = Vec<usize>;
type CoordTransform = (HalfTransform, HalfTransform);

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

fn half_transform<'a>(
    factor: usize, galaxies: &'a Vec<Pos>, select: impl Fn(&'a Pos) -> usize)
    -> HalfTransform {
    let occupied: HashSet<usize> = galaxies.iter().map(select).collect();
    let largest = occupied.iter().max().copied().unwrap_or(0);
    (0..=largest).scan(0, |stretch, i| {
        let s = *stretch;
        if !occupied.contains(&i) { *stretch += factor-1 }
        Some(s + i)
    }).collect()
}

fn transform_pos((r, c): Pos, (rt, ct): &CoordTransform) -> Pos {
    (rt[r], ct[c])
}

fn total_distance(galaxies: &Vec<Pos>, stretch_factor: usize) -> usize {
    let trans = (half_transform(stretch_factor, &galaxies, |&(r, _)| r),
                 half_transform(stretch_factor, &galaxies, |&(_, c)| c));
    let len = galaxies.len();
    (0..len)
        .fold(0, |isum, i| {
            let pi = transform_pos(galaxies[i], &trans);
            (i+1..len).fold(isum, |jsum, j| {
                let pj = transform_pos(galaxies[j], &trans);
                jsum + pi.0.abs_diff(pj.0) + pi.1.abs_diff(pj.1)
            })
        })
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
    let stretch_factor: usize =
        if let Some(arg) = args.next() {
            arg.parse().expect("stretch factor")
        } else {
            2
        };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let galaxies = read_galaxies(&contents);
    let total = total_distance(&galaxies, stretch_factor);

    println!("{total}")
}
