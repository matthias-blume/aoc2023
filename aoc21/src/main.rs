// Advent-of-Code 2023
// Day 21
// Author: Matthias Blume

// This is crappy scratchpad code, used to search for the solution to part 2.

use std::env;
use std::fs;
use std::collections::BTreeSet;

fn md(x: i64, m: usize) -> usize {
    if x >= 0 { (x as usize) % m }
    else { (m - (((-x) as usize) % m)) % m }
}

struct Plot {
    height: usize,
    width: usize,
    rocks: Vec<Vec<bool>>,
}

impl Plot {
    fn x(&self, i: i64) -> usize {
        md(i, self.height)
    }

    fn y(&self, j: i64) -> usize {
        md(j, self.width)
    }

    fn is_outside(&self, (i, j): (i64, i64)) -> bool {
        i < 0 || j < 0 || i >= (self.height as i64) || j >= (self.width as i64)
    }
    
    fn is_empty(&self, i: i64, j: i64) -> bool {
        !self.rocks[self.x(i)][self.y(j)]
    }
}

type Occu = BTreeSet<(i64, i64)>;

fn make_initial(plot: &Plot, (i, j): (usize, usize)) -> Occu {
    vec![(i as i64, j as i64)].into_iter().collect()
}

fn step(plot: &Plot, state: Occu) -> Occu {
    let mut new_state = BTreeSet::new();
    for &(i, j) in state.iter() {
        vec![(i-1, j), (i+1, j), (i, j-1), (i, j+1)].iter()
            .for_each(|&(x, y)| if !plot.is_outside((x, y)) && plot.is_empty(x, y) { new_state.insert((x, y)); });
    }
    new_state
}

fn count_after(plot: &Plot, p: (usize, usize), n: usize) -> usize {
    let mut state = make_initial(plot, p);
    for _ in 0..n {
        state = step(plot, state);
    }
    let r = state.len();
    println!("after {}: ({}, {}): {}", n, p.0, p.1, r);
    r
}

fn double_step(plot: &Plot, state: Occu) -> (bool, Occu) {
    let state1 = step(plot, state.clone());
    let state2 = step(plot, state1);
    (state != state2, state2)
}

fn double_step_stabilize(plot: &Plot, p: (usize, usize), one_step: bool) -> (usize, usize) {
    let mut state = make_initial(plot, p);
    let mut changed = true;
    let mut steps = if one_step {
        state = step(plot, state);
        2
    } else  { 0 };
    loop {
        (changed, state) = double_step(plot, state);
        if !changed { break };
        steps += 2;
    }
    let num_inside = state.len();
    state = step(plot, state);
    let num_next = state.len();
    println!("stabilize: ({}, {}): steps: {}, inside: {}, next: {}", p.0, p.1, steps, num_inside, num_next);
    return (steps, num_inside)
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

    let mut pos = (0, 0);
    let rocks = contents.lines().enumerate().map(|(i, line)|
                                                line.chars().enumerate().map(|(j, c)| {
                                                    match c {
                                                        '#' => true,
                                                        '.' => false,
                                                        'S' => { pos = (i, j); false },
                                                        _ => panic!("bad spot"),
                                                    }
                                                }).collect::<Vec<bool>>()).collect::<Vec<_>>();

    let plot = Plot{ height: rocks.len(), width: rocks[0].len(), rocks };

    println!("height: {}, width: {}, i: {}, j: {}", plot.height, plot.width, pos.0, pos.1);

    let p0 = (65, 65);
    let pn = (0, 65);
    let ps = (130, 65);
    let pw = (65, 0);
    let pe = (65, 130);
    let pnw = (0, 0);
    let pne = (0, 130);
    let psw = (130, 0);
    let pse = (130, 130);

    let _ = double_step_stabilize(&plot, p0, false);
    let _ = double_step_stabilize(&plot, pn,  false);
    let _ = double_step_stabilize(&plot, ps,  false);
    let _ = double_step_stabilize(&plot, pw,  false);
    let _ = double_step_stabilize(&plot, pe,  false);
    let _ = double_step_stabilize(&plot, pnw, false);
    let _ = double_step_stabilize(&plot, psw, false);
    let _ = double_step_stabilize(&plot, pne, false);
    let _ = double_step_stabilize(&plot, pse, false);
    
    let n1: i64 = 7383;
    let n2: i64 = 7457; // origin settles here
    
    let straight_settles = 194; // n1, n2
    let diag_settles = 260; // n2, n1

    let n: i64 = 26501365;

    let rounds_straight = (n - 66) / 131;
    let r_straight = (n - 66) - rounds_straight * 131;
    let straight_rem = r_straight as usize;

    let rounds_diag = (n - 132) / 131;
    let r_diag = (n - 132) - rounds_diag * 131;
    let diag_rem = r_diag as usize;

    println!("straight {rounds_straight} {r_straight}, diag {rounds_diag} {r_diag}");


    let n_partial1 = count_after(&plot, pn, straight_rem);
    let n_partial1 = count_after(&plot, pn, straight_rem + 131);
    let n_partial1 = count_after(&plot, pn, straight_rem + 262);

    let s_partial1 = count_after(&plot, ps, straight_rem);
    let s_partial1 = count_after(&plot, ps, straight_rem + 131);
    let s_partial1 = count_after(&plot, ps, straight_rem + 262);

    let w_partial1 = count_after(&plot, pw, straight_rem);
    let w_partial1 = count_after(&plot, pw, straight_rem + 131);
    let w_partial1 = count_after(&plot, pw, straight_rem + 262);

    let e_partial1 = count_after(&plot, pe, straight_rem);
    let e_partial1 = count_after(&plot, pe, straight_rem + 131);
    let e_partial1 = count_after(&plot, pe, straight_rem + 262);

    let nw_partial0 = count_after(&plot, pnw, diag_rem);
    let nw_partial1 = count_after(&plot, pnw, diag_rem + 131);
    let nw_partial2 = count_after(&plot, pnw, diag_rem + 262);

    let sw_partial0 = count_after(&plot, psw, diag_rem);
    let sw_partial1 = count_after(&plot, psw, diag_rem + 131);
    let sw_partial2 = count_after(&plot, psw, diag_rem + 262);

    let ne_partial0 = count_after(&plot, pne, diag_rem);
    let ne_partial1 = count_after(&plot, pne, diag_rem + 131);
    let ne_partial2 = count_after(&plot, pne, diag_rem + 262);

    let se_partial0 = count_after(&plot, pse, diag_rem);
    let se_partial1 = count_after(&plot, pse, diag_rem + 131);
    let se_partial2 = count_after(&plot, pse, diag_rem + 262);

    let origin_total = n1;
    let straight_total =
        (5577 + 5557 + 5569 + 5565) +
        n2 * 4 +
        (n1 + n2) * (rounds_straight - 1) * 2;
    let diag_total =
        (rounds_diag+1) * (938 + 970 + 959 + 939) +
        rounds_diag * (6492 + 6460 + 6468 + 6480) +
        (rounds_diag * rounds_diag - 1) * n2 +
        (rounds_diag - 1) * (rounds_diag - 1) * n1;
    let total = origin_total + straight_total + diag_total;

    println!("{total}");
}
