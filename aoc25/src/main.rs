// Advent-of-Code 2023
// Day 25
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::{HashMap,HashSet};
use pathfinding::directed::edmonds_karp::*;

struct Interner<'a> {
    mapping: HashMap<&'a str, usize>,
}

impl<'a> Interner<'a> {
    fn new() -> Self {
        Interner{ mapping: HashMap::new() }
    }

    fn intern(&mut self, s: &'a str) -> usize {
        if let Some(&n) = self.mapping.get(&s) {
            n
        } else {
            let n = self.mapping.len();
            self.mapping.insert(s, n);
            n
        }
    }
}

struct Graph {
    nodes: HashSet<usize>,
    edges: HashSet<(usize, usize)>,
    successors: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph{ nodes: HashSet::new(), edges: HashSet::new(), successors: HashMap::new() }
    }

    fn insert_edge(&mut self, x: usize, y: usize) {
        self.nodes.insert(x);
        self.nodes.insert(y);
        let edge = if x < y { (x, y) } else { (y, x) };
        self.edges.insert(edge);
        if let Some(ref mut s) = self.successors.get_mut(&x) {
            s.insert(y);
        } else {
            let mut s = HashSet::new();
            s.insert(y);
            self.successors.insert(x, s);
        }
        if let Some(ref mut s) = self.successors.get_mut(&y) {
            s.insert(x);
        } else {
            let mut s = HashSet::new();
            s.insert(x);
            self.successors.insert(y, s);
        }
    }

    fn visit_all(&self, n: usize, visited: &mut HashSet<usize>, ignored_nodes: &HashSet<usize>, ignored_edges: &HashSet<(usize, usize)>) {
        if ignored_nodes.contains(&n) { return }
        if visited.contains(&n) { return }
        visited.insert(n);
        if let Some(s) = self.successors.get(&n) {
            for &m in s {
                let e = if n < m { (n, m) } else { (m, n) };
                if !ignored_edges.contains(&e) {
                    self.visit_all(m, visited, ignored_nodes, ignored_edges)
                }
            }
        }
    }

    fn extract_component(&self, deleted_nodes: &mut HashSet<usize>, ignored_edges: &HashSet<(usize, usize)>) -> Option<HashSet<usize>> {
        if let Some(&n) = self.nodes.iter().filter(|i| !deleted_nodes.contains(i)).next() {
            let mut visited = HashSet::new();
            self.visit_all(n, &mut visited, deleted_nodes, ignored_edges);
            for i in visited.iter() {
                deleted_nodes.insert(*i);
            }
            Some(visited)
        } else {
            None
        }
    }

    fn to_components(&self, ignored_edges: &HashSet<(usize, usize)>) -> Vec<HashSet<usize>> {
        let mut v = Vec::new();
        let mut deleted = HashSet::new();
        while let Some(c) = self.extract_component(&mut deleted, ignored_edges) {
            v.push(c);
        }
        v
    }

    fn read_line<'a>(&mut self, s: &'a str, interner: &mut Interner<'a>) {
        match s.split(":").collect::<Vec<_>>().as_slice() {
            [left, right] => {
                let x = interner.intern(left.trim());
                let ys =
                    right.split_whitespace().map(|y| interner.intern(y)).collect::<Vec<_>>();
                for y in ys.into_iter() {
                    self.insert_edge(x, y);
                }
            },
            _ => panic!("bad line"),
        }
    }


    fn ek(&self, n: usize) -> Option<usize> {
        let vertices = self.nodes.iter().cloned().collect::<Vec<_>>();
        let mut caps: Vec<((usize, usize), i32)> = Vec::new();
        for &(x, y) in self.edges.iter() {
            caps.push(((x, y), 1));
            caps.push(((y, x), 1));
        }
        let (_, _, mc) = edmonds_karp::<usize, i32, std::vec::IntoIter<((usize, usize), i32)>, DenseCapacity<i32>>(
            &vertices, &0, &n, caps.into_iter());
        if mc.len() != 3 {
            return None;
        }
        let ignored = mc.iter().map(|&((x, y), _)| if x < y { (x, y) } else { (y, x) }).collect();
        let components = self.to_components(&ignored);
        Some(components[0].len() * components[1].len())
    }

    fn find_ek(&self) -> usize {
        for i in 1..self.nodes.len() {
            if let Some(prod) = self.ek(i) {
                return prod
            }
        }
        0
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

    let mut g = Graph::new();
    let mut interner = Interner::new();
    for line in contents.lines() {
        g.read_line(line, &mut interner);
    }

    let prod = g.find_ek();
    println!("{prod}");
}
