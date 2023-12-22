// Advent-of-Code 2023
// Day 22
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from(s: &str) -> Self {
        match s.split(",").collect::<Vec<_>>().as_slice() {
            [xs, ys, zs] =>
                Point{
                    x: xs.parse().expect("x"),
                    y: ys.parse().expect("y"),
                    z: zs.parse().expect("z"),
                },
            _ => panic!("bad point"),
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

impl Brick {
    fn from_points(p1: Point, p2: Point) -> Self {
        Brick{
            x: Extent::from(p1.x, p2.x),
            y: Extent::from(p1.y, p2.y),
            z: Extent::from(p1.z, p2.z),
        }
    }

    fn from(s: &str) -> Self {
        match s.split("~").collect::<Vec<_>>().as_slice() {
            [s1, s2] => Self::from_points(Point::from(s1), Point::from(s2)),
            _ => panic!("bad brick"),
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

    fn insert(&mut self, b: &Brick) {
        let z = (0..self.len())
            .filter_map(|i| self.bricks[i]
                        .can_collide_with(b)
                        .then_some(self.bricks[i].z.high + 1))
            .max()
            .unwrap_or(1);
        self.bricks.push(
            Brick{ z: Extent::from(z, b.z.high - b.z.low + z), ..*b });
    }

    fn from_bricks(mut bricks: Vec<Brick>) -> Self {
        bricks.sort();
        let mut s = Stacking{ bricks: Vec::new() };
        bricks.iter().for_each(|b| s.insert(b));
        s
    }

    fn supported(&self, n: usize) -> IndexSet {
        let b = &self.bricks[n];
        (n+1..self.len()).filter(|&i| b.supports(&self.bricks[i])).collect()
    }

    fn all_supported(&self) -> Vec<IndexSet> {
        (0..self.len()).map(|i| self.supported(i)).collect()
    }

    fn supporters(&self, n: usize) -> IndexSet {
        let b = &self.bricks[n];
        (0..n).filter(|&i| self.bricks[i].supports(b)).collect()
    }

    fn all_supporters(&self) -> Vec<IndexSet> {
        (0..self.len()).map(|i| self.supporters(i)).collect()
    }
}

struct SupportInfo {
    sz: usize,
    supported: Vec<IndexSet>,
    supporters: Vec<IndexSet>,
}

impl SupportInfo {
    fn from(stacking: &Stacking) -> Self {
        SupportInfo{
            sz: stacking.len(),
            supported: stacking.all_supported(),
            supporters: stacking.all_supporters(),
        }
    }

    fn is_removable(&self, n: usize) -> bool {
        self.supported[n].iter().all(|&i| self.supporters[i].len() != 1)
    }

    fn topple(&self, n: usize, toppled: &mut HashSet<usize>) {
        if toppled.contains(&n) { return }
        toppled.insert(n);
        for j in self.supported[n].iter() {
            if self.supporters[*j].is_subset(&toppled) {
                self.topple(*j, toppled)
            }
        }
    }

    fn num_toppled(&self, n: usize) -> usize {
        let mut toppled = HashSet::new();
        self.topple(n, &mut toppled);
        toppled.len() - 1
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

    let bricks = contents.lines().map(Brick::from).collect::<Vec<_>>();
    let stacking = Stacking::from_bricks(bricks);
    let sinfo = SupportInfo::from(&stacking);

    let total = (0..sinfo.sz).filter(|&n| sinfo.is_removable(n)).count();
    println!("num removable: {total}");

    let ntopple: usize = (0..sinfo.sz).map(|n| sinfo.num_toppled(n)).sum();
    println!("toppled: {ntopple}");
}
