// Advent-of-Code 2023
// Day 19
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
enum Action {
    Accept,
    Reject,
    SendToWorkflow(String),
}
use Action::*;

impl Action {
    fn from(s: &str) -> Self {
        match s {
            "A" => Accept,
            "R" => Reject,
            _ => SendToWorkflow(s.to_string()),
        }
    }

    fn count(&self, rd: RangeData, workflows: &Vec<Workflow>) -> i64 {
        match self {
            Accept => rd.num_combinations(),
            Reject => 0,
            SendToWorkflow(w) => {
                let wf = workflows.iter().find(|x| x.name == *w).expect("workflow!");
                wf.count(rd, workflows)
            },
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum Prop { X, M, A, S }

impl Prop {
    fn from(s: &str) -> Self {
        match s {
            "x" => Prop::X,
            "m" => Prop::M,
            "a" => Prop::A,
            "s" => Prop::S,
            _ => panic!("prop")
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Comp { Less, Greater }

impl Comp {
    fn from(s: &str) -> Self {
        match s {
            "<" => Comp::Less,
            ">" => Comp::Greater,
            _ => panic!("comp"),
        }
    }
}

struct Condition(Prop, Comp, i64);

impl Condition {
    fn from(s: &str) -> Self {
        Condition(Prop::from(&s[0..1]), Comp::from(&s[1..2]), s[2..].parse::<i64>().expect("condition value"))
    }

    fn holds_for(&self, data: &Data) -> bool {
        if let Some(&prop_val) = data.0.get(&self.0) {
            match self.1 {
                Comp::Less => prop_val < self.2,
                Comp::Greater => prop_val > self.2,
            }
        } else { false }
    }

    fn split(&self, rd: &RangeData) -> (Option<RangeData>, Option<RangeData>) {
        let Condition(p, c, v) = self;
        let (start, end) = rd.0.get(&p).expect("range!");
        let (gstart, gend, bstart, bend) = match c {
            Comp::Less => {
                if end <= v { (*start, *end, 0, 0) }
                else if v < start { (0, 0, *start, *end) }
                else { (*start, *v, *v, *end) }
            },
            Comp::Greater => {
                if start > v { (*start, *end, 0,  0) }
                else if v >= end { (0, 0, *start, *end) }
                else { (*v + 1, *end, *start, *v + 1) }
            },
        };
        let good =
            if gstart < gend {
                let mut rdgood = (*rd).clone();
                rdgood.0.insert(*p, (gstart, gend));
                Some(rdgood)
            } else { None };
        let bad =
            if bstart < bend {
                let mut rdbad = (*rd).clone();
                rdbad.0.insert(*p, (bstart, bend));
                Some(rdbad)
            } else { None };
        (good, bad)
    }
}

struct Rule(Condition, Action);

impl Rule {
    fn from(s: &str) -> Self {
        match s.split(":").collect::<Vec<_>>().as_slice() {
            [cond_str, act_str] =>
                Rule(Condition::from(cond_str), Action::from(act_str)),
            _ => panic!("rule"),
        }
    }

    fn eval(&self, data: &Data) -> Option<Action> {
        if self.0.holds_for(data) { Some(self.1.clone()) } else { None }
    }

    fn split(&self, rd: &RangeData) -> (Option<RangeData>, Option<RangeData>) {
        self.0.split(rd)
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    catchall: Action,
}

impl Workflow {
    fn from(s: &str) -> Self {
        match s.split("{").collect::<Vec<_>>().as_slice() {
            [name, rest] => {
                match rest[..rest.len()-1].split(",").collect::<Vec<_>>().as_slice() {
                    [rule_strings @ .., catchall_string] =>
                        Workflow {
                            name: name.to_string(),
                            rules: rule_strings.iter().map(|&s| Rule::from(s)).collect(),
                            catchall: Action::from(catchall_string),
                        },
                    _ => panic!("rules"),
                }
            },
            _ => panic!("workflow name"),
        }
    }

    fn eval(&self, data: &Data) -> Action {
        for rule in self.rules.iter() {
            if let Some(action) = rule.eval(data) {
                return action;
            }
        }
        self.catchall.clone()
    }

    fn count(&self, rd: RangeData, workflows: &Vec<Workflow>) -> i64 {
        let mut cur = rd;
        let mut total = 0;
        for rule in self.rules.iter() {
            let (good, bad) = rule.split(&cur);
            if let Some(g) = good {
                total += rule.1.count(g, workflows);
            };
            if let Some(b) = bad {
                cur = b;
            } else {
                return total
            }
        }
        total += self.catchall.count(cur, workflows);
        total
    }
}

#[derive(Debug)]
struct Data(HashMap<Prop, i64>);

impl Data {
    fn read_prop_val(s: &str) -> (Prop, i64) {
        (Prop::from(&s[0..1]), s[2..].parse::<i64>().expect("data value"))
    }

    fn from(s: &str) -> Self {
        Data(s[1..s.len()-1].split(",").map(Data::read_prop_val).collect())
    }

    fn total(&self) -> i64 {
        self.0.values().sum()
    }

    fn is_accepted(&self, workflows: &Vec<Workflow>) -> bool {
        let mut cur = String::from("in");
        loop {
            let wf = workflows.iter().find(|&x| x.name == *cur).expect("workflow");
            match wf.eval(self) {
                Accept => return true,
                Reject => return false,
                SendToWorkflow(w) => cur = w,
            }
        }
    }
}

#[derive(Clone)]
struct RangeData(HashMap<Prop, (i64, i64)>);

impl RangeData {
    fn num_combinations(&self) -> i64 {
        self.0.values().map(|(start, end)| end-start).product()
    }

    fn full() -> Self {
        RangeData(
            vec![(Prop::X, (1, 4001)), (Prop::M, (1, 4001)), (Prop::A, (1, 4001)), (Prop::S, (1, 4001))]
                .into_iter()
                .collect())
    }

    fn count_all(workflows: &Vec<Workflow>) -> i64 {
        let wf = workflows.iter().find(|x| x.name == "in").expect("start workflow");
        wf.count(RangeData::full(), workflows)
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

    let mut workflows = Vec::new();
    let mut data_section = false;
    let mut data = Vec::new();
    for line in contents.lines() {
        if line == "" {
            data_section = true;
        } else if !data_section {
            workflows.push(Workflow::from(line));
        } else {
            data.push(Data::from(line));
        }
    }

    let mut total = 0;
    for d in data {
        if d.is_accepted(&workflows) {
            total += d.total();
        }
    }

    println!("part 1: {total}");

    let combinations = RangeData::count_all(&workflows);

    println!("part 2: {combinations}");
}
