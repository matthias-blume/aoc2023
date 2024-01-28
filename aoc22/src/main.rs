// Advent-of-Code 2023
// Day 22
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;
use std::str::FromStr;
use std::convert::From;
use std::result::Result;

use util::iter::*;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl From<std::num::ParseIntError> for ParsePointError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParsePointError
    }
}

impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").boxed()[..] {
            [xs, ys, zs] =>
                Ok(Point{
                    x: xs.parse()?,
                    y: ys.parse()?,
                    z: zs.parse()?,
                }),
            _ => Err(ParsePointError),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Extent {
    low: i64,
    high: i64,
}

impl Extent {
    fn from(x: i64, y: i64) -> Self {
        if x < y { Extent{ low: x, high: y } }
        else { Extent{ low: y, high: x } }
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.high >= other.low && other.high >= self.low
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Brick {
    z: Extent,  // (derived) lexicographic ordering prefers z
    x: Extent,
    y: Extent,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBrickError;

impl From<ParsePointError> for ParseBrickError {
    fn from(_: ParsePointError) -> Self {
        ParseBrickError
    }
}

impl FromStr for Brick {
    type Err = ParseBrickError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("~").boxed()[..] {
            [s1, s2] => Ok(Self::from_points(s1.parse()?, s2.parse()?)),
            _ => Err(ParseBrickError),
        }
    }
}

impl Brick {
    fn from_points(p1: Point, p2: Point) -> Self {
        Brick{
            x: Extent::from(p1.x, p2.x),
            y: Extent::from(p1.y, p2.y),
            z: Extent::from(p1.z, p2.z),
        }
    }

    fn can_collide_with(&self, other: &Self) -> bool {
        self.x.overlaps_with(&other.x) &&
            self.y.overlaps_with(&other.y)
    }

    fn supports(&self, other: &Self) -> bool {
        self.can_collide_with(other) && self.z.high + 1 == other.z.low
    }
}

struct Stacking {
    bricks: Vec<Brick>
}

type IndexSet = HashSet<usize>;

impl Stacking {
    fn len(&self) -> usize {
        self.bricks.len()
    }

    fn insert(&mut self, b: Brick) {
        let z = (0..self.len())
            .filter_map(|i| self.bricks[i]
                        .can_collide_with(&b)
                        .then_some(self.bricks[i].z.high + 1))
            .max()
            .unwrap_or(1);
        self.bricks.push(
            Brick{ z: Extent::from(z, b.z.high - b.z.low + z), ..b });
    }

    fn from_bricks(mut bricks: Vec<Brick>) -> Self {
        bricks.sort();
        let mut s = Stacking{ bricks: Vec::new() };
        bricks.into_iter().for_each(|b| s.insert(b));
        s
    }

    fn supported(&self, n: usize) -> IndexSet {
        let b = &self.bricks[n];
        (n+1..self.len()).filter(|&i| b.supports(&self.bricks[i])).collect()
    }

    fn all_supported(&self) -> Vec<IndexSet> {
        (0..self.len()).map(|i| self.supported(i)).collect()
    }
}

struct SupportInfo {
    sz: usize,
    supported: Vec<IndexSet>,
    num_supporters: Vec<usize>,
}

impl SupportInfo {
    fn from(stacking: &Stacking) -> Self {
        let sz = stacking.len();
        let supported = stacking.all_supported();
        let mut num_supporters = vec![0; sz];
        supported.iter()
            .for_each(|s| s.iter()
                      .for_each(|&i| num_supporters[i] += 1));
        SupportInfo{ sz, supported, num_supporters }
    }

    fn is_removable(&self, n: usize) -> bool {
        self.supported[n].iter().all(|&i| self.num_supporters[i] != 1)
    }

    fn num_toppled(&self, n: usize) -> usize {
        let mut remaining_support = self.num_supporters.clone();
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(n);
        while let Some(i) = stack.pop() {
            for &j in &self.supported[i] {
                remaining_support[j] -= 1;
                if remaining_support[j] == 0 {
                    count += 1;
                    stack.push(j);
                }
            }
        }
        count
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MainError {
    msg: String,
}

impl From<ParseBrickError> for MainError {
    fn from(_: ParseBrickError) -> Self {
        MainError{ msg: String::from("unable to parse brick") }
    }
}

impl From<std::io::Error> for MainError {
    fn from(e: std::io::Error) -> Self {
        MainError{ msg: format!("I/O error: {}", e) }
    }
}

fn main() -> Result<(), MainError> {
    let mut args = env::args();
    let program = match args.next() {
        Some(arg) => arg,
        _ => { return Err(MainError{ msg: String::from("no program name") }) },
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        _ => { return Err(MainError{ msg: format!("{}: no input file name", program) }) },
    };

    let contents = fs::read_to_string(file_path)?;

    let bricks: Vec<Brick> =
        contents.lines().map(|s| s.parse()).collect::<Result<_, _>>()?;
    let stacking = Stacking::from_bricks(bricks);
    let sinfo = SupportInfo::from(&stacking);

    let total = (0..sinfo.sz).filter(|&n| sinfo.is_removable(n)).count();
    println!("num removable: {total}");

    let ntopple: usize = (0..sinfo.sz).map(|n| sinfo.num_toppled(n)).sum();
    println!("toppled: {ntopple}");

    Ok(())
}
