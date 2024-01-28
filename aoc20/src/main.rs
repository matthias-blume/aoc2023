// Advent-of-Code 2023
// Day 20
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::{HashMap,VecDeque};

use util::iter::*;

enum Type {
    NOP,
    NAND,
    FF,
}
use Type::*;

type Connections = Vec<usize>;

struct Element {
    typ: Type,
    num: usize,
    ins: Connections,
    outs: Connections,
}

impl Element {
    fn get_outs<'a>(s: &'a str, interner: &mut Interner<'a>) -> Connections {
        s.split(",").map(|x| interner.intern(x.trim())).collect()
    }
    
    fn from<'a>(s: &'a str, interner: &mut Interner<'a>) -> Self {
        match s.split("->").map(str::trim).boxed()[..] {
            ["broadcaster", right] => Element{
                typ: NOP,
                num: 0,
                ins: Vec::new(),
                outs: Element::get_outs(right, interner),
            },
            [left, right] => {
                let name = &left[1..];
                let typ = match &left[0..1] {
                    "&" => NAND,
                    "%" => FF,
                    _ => panic!("bad type"),
                };
                Element{
                    typ,
                    num: interner.intern(name),
                    ins: Vec::new(),
                    outs: Element::get_outs(right, interner),
                }
            },
            _ => panic!("bad element"),
        }
    }
}

struct Interner<'a> {
    mapping: HashMap<&'a str, usize>,
}

impl<'a> Interner<'a> {
    fn new() -> Self {
        Self{ mapping: HashMap::from([("*", 0)]) } // 0 is broadcaster
    }

    fn intern(&mut self, name: &'a str) -> usize {
        let n = self.size();
        *self.mapping.entry(name).or_insert(n)
    }

    fn size(&self) -> usize {
        self.mapping.len()
    }

    fn known(&self, name: &'a str) -> Option<usize> {
        self.mapping.get(name).cloned()
    }
}

struct Circuit<'a> {
    interner: Interner<'a>,
    elements: Vec<Element>,
}

impl<'a> Circuit<'a> {
    fn from(input: &'a str) -> Self {
        let mut interner = Interner::new();
        let mut elements = HashMap::new();
        let mut ins_table: HashMap<_, Vec<_>> = HashMap::new();
        for line in input.lines() {
            let element = Element::from(line, &mut interner);
            for &out in element.outs.iter() {
                ins_table.entry(out).or_default().push(element.num);
            }
            elements.insert(element.num, element);
        }
        let nelem = interner.size();
        let mut circuit = Circuit{ interner, elements: Vec::new() };
        for i in 0..nelem {
            if let Some(element) = elements.remove(&i) {
                circuit.elements.push(Element{
                    ins: ins_table.remove(&i).unwrap_or_default(),
                    ..element
                });
            } else {
                circuit.elements.push(Element{
                    num: i,
                    typ: NOP,
                    ins: ins_table.remove(&i).unwrap_or_default(),
                    outs: Vec::new(),
                });
            }
        }
        circuit
    }

    fn once(&self, state: &mut State) -> (usize, usize) {
        let mut high = 0;
        let mut low = self.elements[0].outs.len() + 1;
        let mut q = VecDeque::new();
        for &bc in self.elements[0].outs.iter() {
            q.push_back((bc, false));
        }
        while let Some((n, l)) = q.pop_front() {
            let e = &self.elements[n];
            let maybe_out_l = match e.typ {
                NOP => None,
                NAND => Some(!e.ins.iter().all(|&i| state.levels[i])),
                FF => (!l).then_some(!state.levels[n]),
            };
            if let Some(out_l) = maybe_out_l {
                state.levels[n] = out_l;
                let nouts = e.outs.len();
                if out_l { high += nouts } else { low += nouts };
                for &out in e.outs.iter() {
                    q.push_back((out, out_l));
                }
            }
        }
        (high, low)
    }

    fn repeatedly(&self, state: &mut State, rounds: usize) -> (usize, usize) {
        let mut high = 0;
        let mut low = 0;
        for _ in 0..rounds {
            let (h, l) = self.once(state);
            high += h;
            low += l;
        }
        (high, low)
    }

    fn sends_low_pulse_on(&self, state: &mut State, watched: usize) -> bool {
        let mut q = VecDeque::new();
        for &bc in self.elements[0].outs.iter() {
            q.push_back((bc, false));
        }
        while let Some((n, l)) = q.pop_front() {
            if !l && n == watched { return true }
            let e = &self.elements[n];
            let maybe_out_l = match e.typ {
                NOP => None,
                NAND => Some(!e.ins.iter().all(|&i| state.levels[i])),
                FF => (!l).then_some(!state.levels[n]),
            };
            if let Some(out_l) = maybe_out_l {
                state.levels[n] = out_l;
                for &out in e.outs.iter() {
                    q.push_back((out, out_l));
                }
            }
        }
        false
    }

    fn count_until_low_pulse_on(&self, state: &mut State, watched: usize) -> usize {
        let mut count = 1;
        while !self.sends_low_pulse_on(state, watched) {
            if count % 1000000000 == 0 {
                eprintln!("{count}");
            }
            count += 1;
        }
        count
    }
}

struct State {
    levels: Vec<bool>,
}

impl State {
    fn for_circuit(circuit: &Circuit) -> Self {
        State{ levels: (0..circuit.elements.len()).map(|_| false).collect() }
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

    let circuit = Circuit::from(&contents);
    let mut state1 = State::for_circuit(&circuit);
    let (high, low) = circuit.repeatedly(&mut state1, 1000);

    println!("Part 1: {high} {low} {}", high*low);

    // This is brute-force part 2.  (I don't know a better way for the general
    // case, as it seems to encompass SAT-solving.  The actual puzzle input can
    // be solved by inspection, but it is very special in nature and not anywhere
    // near the special case.  Brute-forcing the puzzle is hopeless, since the
    // answer is enormous.)
    if let Some(rx) = circuit.interner.known("rx") {
        let mut state2 = State::for_circuit(&circuit);
        let count = circuit.count_until_low_pulse_on(&mut state2, rx);
        println!("Part 2: {count}");
    } else {
        println!("Part 2: no rx");
    }
}
