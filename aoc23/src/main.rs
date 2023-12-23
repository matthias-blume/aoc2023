// Advent-of-Code 2023
// Day 23
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Spot {
    Path,
    Forest,
    North,
    South,
    West,
    East,
}
use Spot::*;

struct Row {
    spots: Vec<Spot>,
}

#[derive(Clone, Copy)]
enum Mode {
    WithSlopes,
    WithoutSlopes,
}
use Mode::*;

impl Row {
    fn from(s: &str, mode: Mode) -> Self {
        Row{ spots: s.chars()
             .map(|c| {
                 match (c, mode) {
                     ('.', _) => Path,
                     ('#', _) => Forest,
                     ('^' | 'v' | '>' | '<', WithoutSlopes) => Path,
                     ('^', WithSlopes) => North,
                     ('v', WithSlopes) => South,
                     ('>', WithSlopes) => East,
                     ('<', WithSlopes) => West,
                     _ => panic!("bad spot"),
                 }
             })
             .collect()
        }
    }
}

struct Field {
    height: i64,
    width: i64,
    rows: Vec<Row>,
}

type Pos = (i64, i64);
type PosSet = HashSet<Pos>;

fn north(p: Pos) -> Pos {
    (p.0 - 1, p.1)
}

fn south(p: Pos) -> Pos {
    (p.0 + 1, p.1)
}

fn west(p: Pos) -> Pos {
    (p.0, p.1 - 1)
}

fn east(p: Pos) -> Pos {
    (p.0, p.1 + 1)
}

impl Field {
    fn from(s: &str, mode: Mode) -> Self {
        let rows =
            s.lines().map(|line| Row::from(line, mode)).collect::<Vec<_>>();
        Field{
            height: rows.len() as i64,
            width: rows[0].spots.len() as i64,
            rows,
        }
    }

    fn at(&self, p: Pos) -> Spot {
        self.rows[p.0 as usize].spots[p.1 as usize]
    }

    fn start(&self) -> Pos {
        let j = self.rows[0].spots.iter()
            .position(|&f| f == Path)
            .expect("starting position");
        (0, j as i64)
    }
    
    fn all_paths(&self, p: Pos,
                 steps: usize,
                 visited: &mut PosSet,
                 longest: &mut usize) {
        if p.0 < 0 || p.0 >= self.height ||
            p.1 < 0 || p.1 >= self.width ||
            visited.contains(&p) {
                return
            };
        let f = self.at(p);
        if f == Forest { return };
        if p.0 == self.height - 1 {
            *longest = steps.max(*longest);
            return
        }
        visited.insert(p);
        let steps = steps + 1;
        let next =
            match f {
                Forest => vec![],
                Path => vec![north(p), south(p), east(p), west(p)],
                North => vec![north(p)],
                South => vec![south(p)],
                East => vec![east(p)],
                West => vec![west(p)],
            };
        next.into_iter()
            .for_each(|np| self.all_paths(np, steps, visited, longest));
        visited.remove(&p);
    }

    fn longest_path(&self) -> usize {
        let mut longest = 0;
        let mut visited = HashSet::new();
        self.all_paths(self.start(), 0, &mut visited, &mut longest);
        longest
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
        _ => panic!("{}: no input file name", program),
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let longest_with_slopes =
        Field::from(&contents, WithSlopes).longest_path();
    println!("with slopes: {longest_with_slopes}");

    let longest_without_slopes =
        Field::from(&contents, WithoutSlopes).longest_path();
    println!("without slopes: {longest_without_slopes}");
}
