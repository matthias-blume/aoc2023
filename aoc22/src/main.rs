// Advent-of-Code 2023
// Day 22
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq,Eq,Clone,Copy,Hash,Debug,Ord,PartialOrd)]
struct Point {
    z: i64,
    x: i64,
    y: i64,
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

#[derive(PartialEq,Eq,Clone,Copy,Hash,Debug,Ord,PartialOrd)]
struct Brick {
    start: Point,
    end: Point,
}

fn ordered_range(x: i64, y: i64) -> (i64, i64) {
    if x < y { (x, y) } else { (y, x) }
}

fn intersect(or1: (i64, i64), or2: (i64, i64)) -> bool {
    or1.1 >= or2.0 && or2.1 >= or1.0
}

impl Brick {
    fn from(s: &str) -> Self {
        match s.split("~").collect::<Vec<_>>().as_slice() {
            [s1, s2] => {
                let p1 = Point::from(s1);
                let p2 = Point::from(s2);
                if p1 < p2 {
                    Brick{ start: p1, end: p2 }
                } else {
                    Brick{ start: p2, end: p1 }
                }
            },
            _ => panic!("bad brick"),
        }
    }

    fn xrange(&self) -> (i64, i64) {
        ordered_range(self.start.x, self.end.x)
    }

    fn yrange(&self) -> (i64, i64) {
        ordered_range(self.start.y, self.end.y)
    }

    fn can_collide_with(&self, other: &Self) -> bool {
        intersect(self.xrange(), other.xrange()) &&
            intersect(self.yrange(), other.yrange())
    }

    fn lower_into(&self, v: &mut Vec<Self>) {
        let mut z = 1;
        for i in 0..v.len() {
            if v[i].can_collide_with(self) && v[i].end.z >= z {
                z = v[i].end.z + 1
            }
        }
        let d = self.start.z - z;
        v.push(Brick{ start: Point{ z, ..self.start }, end: Point{ z: self.end.z - d, ..self.end } });
    }

    fn supports(&self, other: &Self) -> bool {
        self.can_collide_with(other) &&
            self.end.z + 1 == other.start.z
    }
}

fn num_support(v: &Vec<Brick>, n: usize) -> usize {
    let mut num = 0;
    for i in 0..n {
        if v[i].supports(&v[n]) {
            num += 1;
        }
    }
    num
}

fn supported(v: &Vec<Brick>, n: usize) -> Vec<usize> {
    (n+1..v.len()).filter(|&i| v[n].supports(&v[i])).collect()
}

fn supporters(v: &Vec<Brick>, n: usize) -> Vec<usize> {
    (0..n).filter(|&i| v[i].supports(&v[n])).collect()
}

fn is_lone_support(v: &Vec<Brick>, num_supports: &Vec<usize>, n: usize) -> bool {
    for i in n+1 .. v.len() {
        if v[n].supports(&v[i]) && num_supports[i] == 1 {
            return true
        }
    }
    false
}

fn destruction(v: &Vec<Brick>, all_supported: &Vec<Vec<usize>>, all_supporters: &Vec<Vec<usize>>, n: usize, toppled: &mut HashSet<usize>) {
    if toppled.contains(&n) { return }
    toppled.insert(n);
    for j in all_supported[n].iter() {
        if all_supporters[*j].iter().all(|k| toppled.contains(&k)) {
            destruction(v, all_supported, all_supporters, *j, toppled);
        }
    }
}

fn num_toppled(v: &Vec<Brick>, all_supported: &Vec<Vec<usize>>, all_supporters: &Vec<Vec<usize>>, n: usize) -> usize {
    let mut toppled = HashSet::new();
    destruction(v, all_supported, all_supporters, n, &mut toppled);
    toppled.len() - 1
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

    let mut bricks = contents.lines().map(Brick::from).collect::<Vec<_>>();

    bricks.sort();

    let mut lowered = Vec::new();
    for b in bricks.iter() {
        b.lower_into(&mut lowered);
    }
    
    let num_supports = (0..lowered.len()).map(|n| num_support(&lowered, n)).collect::<Vec<_>>();
    let total = (0..lowered.len()).filter(|&n| !is_lone_support(&lowered, &num_supports, n)).count();
    
    println!("{total}");

    let all_supported = (0..lowered.len()).map(|n| supported(&lowered, n)).collect::<Vec<_>>();
    let all_supporters = (0..lowered.len()).map(|n| supporters(&lowered, n)).collect::<Vec<_>>();

    let mut ntopple = 0;
    for n in 0..lowered.len() {
        ntopple += num_toppled(&lowered, &all_supported, &all_supporters, n);
    }

    println!("toppled: {ntopple}");
}
