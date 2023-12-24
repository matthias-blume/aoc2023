// Advent-of-Code 2023
// Day 24
// Author: Matthias Blume

use std::env;
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: f64,
    y: f64,
    // z: f64,
}

impl Vector {
    fn from(s: &str) -> Self {
        match s.split(",").map(str::trim).collect::<Vec<_>>().as_slice() {
            [xs, ys, _ /* zs */] => Vector{
                x: xs.parse().expect("x"),
                y: ys.parse().expect("y"),
                // z: zs.parse().expect("z"),
            },
            _ => panic!("bad vector"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Phase {
    position: Vector,
    velocity: Vector,
}

impl Phase{
    fn from(s: &str) -> Self {
        match s.split("@").collect::<Vec<_>>().as_slice() {
            [p, v] =>
                Phase{
                    position: Vector::from(p),
                    velocity: Vector::from(v)
                },
            _ => panic!("bad phase"),
        }
    }
}

// Find "future" (t>=0) x-y coordinates where
// the projections of the two vectors into the x-y plane
// intersect.
fn xy_collision(
    Phase{
        position: Vector{ x: x1, y: y1, .. },
        velocity: Vector{ x: vx1, y: vy1, .. }
    }: Phase,
    Phase{
        position: Vector{ x: x2, y: y2, .. },
        velocity: Vector{ x: vx2, y: vy2, .. }
    }: Phase) -> Option<Vector> {
    let d = vy1 * vx2 - vx1 * vy2;
    if d == 0.0 { return None }
    let t = ((x1 - x2) * vy2 - (y1 - y2) * vx2) / d;
    if t < 0.0 { return None }
    let t2 = ((x1 - x2) + t * vx1) / vx2;
    if t2 < 0.0 { return None }
    Some(Vector{ x: x1 + t * vx1, y: y1 + t * vy1 })
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

    let low = 200000000000000.0;
    let high = 400000000000000.0;
    
    let mut phases = Vec::new();
    for line in contents.lines() {
        phases.push(Phase::from(&line));
    }

    // Part 1:
    let mut count = 0;
    for i in 0..phases.len() {
        for j in i+1..phases.len() {
            let ph1 = phases[i];
            let ph2 = phases[j];
            if let Some(v) = xy_collision(ph1, ph2) {
                if v.x >= low && v.x <= high && v.y >= low && v.y <= high {
                    count += 1;
                }
            }
        }
    }

    // Part 2, obtained "manually".
    let s : i64 = 
        318090941338468 + 124187623124113 + 231363386790708;
    
    println!("{count}, {s}");
}
