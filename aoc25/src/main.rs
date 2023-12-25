// Advent-of-Code 2023
// Day 25
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::{HashMap,HashSet};
use pathfinding::directed::edmonds_karp::edmonds_karp_sparse;

type Node<'a> = &'a str;
type NodeSet<'a> = HashSet<Node<'a>>;
type NodeVector<'a> = Vec<Node<'a>>;
type Edge<'a> = (Node<'a>, Node<'a>);
type EdgeSet<'a> = HashSet<Edge<'a>>;

struct Graph<'a> {
    nodes: NodeVector<'a>,
    edges: EdgeSet<'a>,
    successors: HashMap<Node<'a>, NodeSet<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph{
            nodes: Vec::new(),
            edges: HashSet::new(),
            successors: HashMap::new(),
        }
    }

    fn insert_node(&mut self, n: Node<'a>) {
        if !self.nodes.contains(&n) {
            self.nodes.push(n);
        }
    }
    
    fn insert_edge(&mut self, e: Edge<'a>) {
        self.edges.insert(e);
        if let Some(ref mut s) = self.successors.get_mut(&e.0) {
            s.insert(e.1);
        } else {
            let mut s = HashSet::new();
            s.insert(e.1);
            self.successors.insert(e.0, s);
        }
    }
    
    fn visit_all(&self, n: Node<'a>,
                 visited: &mut NodeSet<'a>,
                 ignored: &NodeSet<'a>) {
        if ignored.contains(&n) || visited.contains(&n) { return }
        visited.insert(n);
        if let Some(s) = self.successors.get(&n) {
            for &m in s {
                self.visit_all(m, visited, ignored);
            }
        }
    }

    fn num_reachable(&self, n: Node<'a>, ignored: &NodeSet<'a>) -> usize {
        let mut visited = HashSet::new();
        self.visit_all(n, &mut visited, ignored);
        visited.len()
    }
    
    fn read_line(&mut self, s: &'a str) {
        match s.split(":").collect::<Vec<_>>().as_slice() {
            [left, right] => {
                let x = left.trim();
                for y in right.split_whitespace() {
                    self.insert_node(x);
                    self.insert_node(y);
                    self.insert_edge((x, y));
                    self.insert_edge((y, x));
                }
            },
            _ => panic!("bad line"),
        }
    }

    fn ek(&self, n: usize) -> Option<usize> {
        let v0 = self.nodes[0];
        let vn = self.nodes[n];
        match edmonds_karp_sparse(&self.nodes, &v0, &vn,
                                  self.edges.iter().cloned().map(|e| (e, 1))) {
            (_, 3, mc) => {
                let ignored_from_0 =
                    mc.iter().map(|((_, y), _)| y).cloned().collect();
                let ignored_from_n =
                    mc.iter().map(|((x, _), _)| x).cloned().collect();
                let num_0 = self.num_reachable(v0, &ignored_from_0);
                let num_n = self.num_reachable(vn, &ignored_from_n);
                Some(num_0 * num_n)
            },
            _ => None
        }
    }

    fn find_ek(&self) -> usize {
        (1..self.nodes.len()).find_map(|i| self.ek(i)).unwrap_or(0)
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
    for line in contents.lines() {
        g.read_line(line);
    }

    let prod = g.find_ek();
    println!("{prod}");
}
