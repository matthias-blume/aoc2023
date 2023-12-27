// Advent-of-Code 2023
// Day 24
// Author: Matthias Blume

use std::env;
use std::fs;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn from(s: &str) -> Self {
        match s.split(",").map(str::trim).collect::<Vec<_>>().as_slice() {
            [xs, ys, zs] => Vector{
                x: xs.parse().expect("x"),
                y: ys.parse().expect("y"),
                z: zs.parse().expect("z"),
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
    Some(Vector{ x: x1 + t * vx1, y: y1 + t * vy1, z: 0.0 })
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

    let default_low = 200_000_000_000_000.0;
    let default_high = 400_000_000_000_000.0;
    
    let (low, high) =
        match args.next() {
            Some(arg) => {
                let l: f64 = arg.parse().expect("low");
                match args.next() {
                    Some(arg) => {
                        let h: f64 = arg.parse().expect("high");
                        (l, h)
                    },
                    None => (l, default_high),
                }
            },
            None => (default_low, default_high),
        };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

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

    println!("part 1: {count}");

    // Grab the first 4 inputs.
    // Note: Three inputs already determine the solution, but
    // solving involves non-linear equations.  Using 4 inputs
    // we can remove the non-linearity by treating mixed terms
    // as if they were independent variables.
    let s1 = phases[0].position;
    let s2 = phases[1].position;
    let s3 = phases[2].position;
    let s4 = phases[3].position;
    let v1 = phases[0].velocity;
    let v2 = phases[1].velocity;
    let v3 = phases[2].velocity;
    let v4 = phases[3].velocity;

    let zero = 0.0;
    let one = 1.0;

    // Set up the linear equation.
    // (This matrix is column-major, i.e., transposed.)
    let m: Array2<f64> = array![
        [one, zero, one, zero, one, zero, one, zero],
        [zero, one, zero, one, zero, one, zero, one],
        [-v1.y, -v1.z, -v2.y, -v2.z, -v3.y, -v3.z, -v4.y, -v4.z],
        [v1.x, zero, v2.x, zero, v3.x, zero, v4.x, zero],
        [zero, v1.x, zero, v2.x, zero, v3.x, zero, v4.x],
        [s1.y, s1.z, s2.y, s2.z, s3.y, s3.z, s4.y, s4.z],
        [-s1.x, zero, -s2.x, zero, -s3.x, zero, -s4.x, zero],
        [zero, -s1.x, zero, -s2.x, zero, -s3.x, zero, -s4.x]];

    let rhs: Array1<f64> = array![
        v1.x * s1.y - s1.x * v1.y,
        v1.x * s1.z - s1.x * v1.z,
        v2.x * s2.y - s2.x * v2.y,
        v2.x * s2.z - s2.x * v2.z,
        v3.x * s3.y - s3.x * v3.y,
        v3.x * s3.z - s3.x * v3.z,
        v4.x * s4.y - s4.x * v4.y,
        v4.x * s4.z - s4.x * v4.z];

    let solution = m.solve_t_into(rhs).expect("solution");
    let x = solution[2];
    let y = solution[3];
    let z = solution[4];
    let sum = (x + y + z).round();

    println!("part 2: {sum} ");
}
