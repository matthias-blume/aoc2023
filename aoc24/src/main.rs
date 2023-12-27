// Advent-of-Code 2023
// Day 24
// Author: Matthias Blume

use std::env;
use std::fs;

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
struct State {
    position: Vector,
    velocity: Vector,
}

impl State {
    fn from(s: &str) -> Self {
        match s.split("@").collect::<Vec<_>>().as_slice() {
            [p, v] =>
                State {
                    position: Vector::from(p),
                    velocity: Vector::from(v)
                },
            _ => panic!("bad state"),
        }
    }
}

// Find x-y coordinates where the projections of the two vectors
// into the x-y plane intersect.
// Determine whether all of the following are true:
//  - The intersection exists.
//  - It lies in the future (t >= 0) for both vectors.
//  - Both x and y coordinates of the intersection lie within
//    the interval low..=high.
fn has_xy_collision(
    State { position: Vector{ x: x1, y: y1, .. },
            velocity: Vector{ x: vx1, y: vy1, .. } }: State,
    State{ position: Vector{ x: x2, y: y2, .. },
           velocity: Vector{ x: vx2, y: vy2, .. } }: State,
    low: f64, high: f64) -> bool {
    let d = vy1 * vx2 - vx1 * vy2;
    if d == 0.0 { return false }  // parallel
    let t = ((x1 - x2) * vy2 - (y1 - y2) * vx2) / d;
    if t < 0.0 { return false }   // in the past for one
    let t2 = ((x1 - x2) + t * vx1) / vx2;
    if t2 < 0.0 { return false }  // .. or for the other
    let x = x1 + t * vx1;
    let y = y1 + t * vy1;
    x >= low && x <= high && y >= low && y <= high
}

// Solve set of linear equations using Gaussian elimination.
fn gauss(mut m: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = m.len();
    assert_eq!(n, b.len());
    assert_eq!(n, m[0].len());
    // Bring m into row echelon form (and adjust b accordingly):
    for h in 0..n {
        // Find best pivot.
        let mut i_max = h;
        for i in h+1..n {
            if m[i][h].abs() > m[i_max][h].abs() { i_max = i }
        }
        if m[i_max][h] == 0.0 { return None }  // determinant 0
        if i_max != h {
            m.swap(h, i_max);
            b.swap(h, i_max);
        }
        let pvt = m[h][h];
        for i in h+1..n {
            let f = m[i][h] / pvt;
            m[i][h] = 0.0;
            for j in h+1..n {
                m[i][j] -= m[h][j] * f;
            }
            b[i] -= b[h] * f;
        }
    }
    // Row echelon form achieved! Now substitute back:
    for h in (0..n).rev() {
        for i in h+1..n {
            b[h] -= m[h][i] * b[i];
        }
        b[h] /= m[h][h];
    }
    Some(b)
}

fn main() {
    let (file_path, low, high) =
        match env::args().collect::<Vec<_>>().as_slice() {
            [_, fp] =>
                (fp.clone(), 200_000_000_000_000.0, 400_000_000_000_000.0),
            [_, fp, l, h] =>
                (fp.clone(), l.parse().expect("low"), h.parse().expect("high")),
            [p, ..] =>
                panic!("usage: {} <file>   or   {} <file> <low> <high>", p, p),
            [] =>
                panic!("empty command line args"),
        };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut states = Vec::new();
    for line in contents.lines() {
        states.push(State::from(&line));
    }

    // Part 1:
    let count: usize = (0..states.len())
        .map(|i| (i+1..states.len())
             .filter(|&j| has_xy_collision(states[i], states[j], low, high))
             .count())
        .sum();

    println!("part 1: {count}");

    // Part 2:
    //
    // Grab the first four inputs and construct system of linear equations.
    //
    // Notes: Any four should do. Three inputs already determine
    // the solution, but the resulting system of equations is
    // non-linear.  When using four inputs we can remove the non-linearity
    // by treating non-linear (mixed) terms as if they were independent
    // variables. These turn out to be linear, so we can just solve for
    // them along with the rest.

    // Matrix rows alternate between y- and z-types, one of each per input.
    let yrow = | i | { let s: State = states[i];
                       vec![1.0, 0.0,
                            -s.velocity.y, s.velocity.x, 0.0,
                            s.position.y, -s.position.x, 0.0] };
    let zrow = | i | { let s: State = states[i];
                       vec![0.0, 1.0,
                            -s.velocity.z, 0.0, s.velocity.x,
                            s.position.z, 0.0, -s.position.x] };

    // Likewise, right-hand sides alternate between y- and z-types.
    let yrhs = | i | { let s: State = states[i];
                       s.velocity.x * s.position.y
                       - s.position.x * s.velocity.y };
    let zrhs = | i | { let s: State = states[i];
                       s.velocity.x * s.position.z
                       - s.position.x * s.velocity.z };
    
    // Coefficient matrix m:
    let m = (0..4).flat_map(|i| [yrow(i), zrow(i)]).collect();

    // Right-hand side vector b:
    let b = (0..4).flat_map(|i| [yrhs(i), zrhs(i)]).collect();

    if let Some(solution) = gauss(m, b) {
        // Slots 0 and 1 contain solutions for the above mentioned
        // non-linear terms.
        let x = solution[2];
        let y = solution[3];
        let z = solution[4];
        // Velocity components are in slots 5, 6, and 7.
        let sum = x + y + z;
        println!("part 2: {sum}");
    } else {
        println!("part 2: no solution");
    }
}
