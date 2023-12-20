// Advent-of-Code 2023
// Day 19
// Author: Matthias Blume

use std::env;
use std::fs;

mod data {

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum Action<'a> {
        Accept,
        Reject,
        SendToWorkflow(&'a str),
    }
    pub use Action::*;

    impl<'a> Action<'a> {
        fn from(s: &'a str) -> Self {
            match s {
                "A" => Accept,
                "R" => Reject,
                _ => SendToWorkflow(s),
            }
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone, Hash)]
    pub enum Prop { X, M, A, S }

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
    pub enum Comp { Less, Greater }

    impl Comp {
        fn from(s: &str) -> Self {
            match s {
                "<" => Comp::Less,
                ">" => Comp::Greater,
                _ => panic!("comp"),
            }
        }
    }

    pub struct Condition {
        pub prop: Prop,
        pub comp: Comp,
        pub value: i64,
    }

    impl Condition {
        fn from(s: &str) -> Self {
            Condition {
                prop: Prop::from(&s[0..1]),
                comp: Comp::from(&s[1..2]),
                value: s[2..].parse::<i64>().expect("condition value"),
            }
        }
    }

    pub struct Rule<'a> {
        pub condition: Condition,
        pub action: Action<'a>,
    }

    impl<'a> Rule<'a> {
        fn from(s: &'a str) -> Self {
            match s.split(":").collect::<Vec<_>>().as_slice() {
                [cond_str, act_str] =>
                    Rule{
                        condition: Condition::from(cond_str),
                        action: Action::from(act_str),
                    },
                _ => panic!("rule"),
            }
        }
    }

    pub struct Workflow<'a> {
        pub name: &'a str,
        pub rules: Vec<Rule<'a>>,
        pub catchall: Action<'a>,
    }

    impl<'a> Workflow<'a> {
        pub fn from(s: &'a str) -> Self {
            match s.split("{").collect::<Vec<_>>().as_slice() {
                [name, rest] => {
                    match rest[..rest.len()-1].split(",").collect::<Vec<_>>().as_slice() {
                        [rule_strings @ .., catchall_string] =>
                            Workflow {
                                name: name,
                                rules: rule_strings.iter().map(|&s| Rule::from(s)).collect(),
                                catchall: Action::from(catchall_string),
                            },
                        _ => panic!("rules"),
                    }
                },
                _ => panic!("workflow name"),
            }
        }
    }

    pub struct WorkflowSuite<'a> {
        workflows: Vec<Workflow<'a>>,
    }

    impl<'a> WorkflowSuite<'a> {
        pub fn new(v: Vec<Workflow<'a>>) -> Self {
            WorkflowSuite{ workflows: v }
        }

        pub fn get(&'a self, name: &str) -> &'a Workflow<'a> {
            self.workflows.iter().find(|&w| w.name == name).expect("workflow?")
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Data<T>{ pub x: T,  pub m: T, pub a: T, pub s: T }

    impl<T> Data<T> where T: Copy {
        pub fn get<'a>(&'a self, p: Prop) -> T {
            match p {
                Prop::X => self.x,
                Prop::M => self.m,
                Prop::A => self.a,
                Prop::S => self.s,
            }
        }

        pub fn update(self, p: Prop, v: T) -> Self {
            match p {
                Prop::X => Data{ x: v, ..self },
                Prop::M => Data{ m: v, ..self },
                Prop::A => Data{ a: v, ..self },
                Prop::S => Data{ s: v, ..self },
            }
        }
    }

    impl Data<i64> {
        fn zero() -> Self {
            Data{ x: 0, m: 0, a: 0, s: 0 }
        }
    
        fn read_prop(self, s: &str) -> Self {
            if &s[1..2] != "=" { panic!("bad prop value spec: {}", s); };
            self.update(Prop::from(&s[0..1]),
                        s[2..].parse::<i64>().expect("data value"))
        }

        pub fn from(s: &str) -> Self {
            s[1..s.len()-1].split(",").fold(Self::zero(), Self::read_prop)
        }
    }
}

mod part1 {
    use crate::data::*;
    use std::collections::HashSet;
    
    impl Data<i64> {
        pub fn total(&self) -> i64 {
            self.x + self.m + self.a + self.s
        }
    }

    impl Condition {
        pub fn holds_for(&self, data: Data<i64>) -> bool {
            let prop_val =  data.get(self.prop);
            match self.comp {
                Comp::Less => prop_val < self.value,
                Comp::Greater => prop_val > self.value,
            }
        }
    }
    
    impl<'a> Rule<'a> {
        fn eval(&self, data: Data<i64>) -> Option<Action<'a>> {
            self.condition.holds_for(data).then_some(self.action)
        }
    }

    impl <'a> Workflow<'a> {
        fn eval(&self, data: Data<i64>) -> Action<'a> {
            self.rules.iter()
                .find_map(|rule| rule.eval(data))
                .unwrap_or(self.catchall)
            }
    }

    impl WorkflowSuite<'_> {
        pub fn accepts(&self, data: Data<i64>) -> bool {
            let mut cur = "in";
            let mut seen = HashSet::new();
            loop {
                if seen.contains(cur) { panic!("cycle at {}", cur) };
                seen.insert(cur);
                match self.get(cur).eval(data) {
                    Accept => return true,
                    Reject => return false,
                    SendToWorkflow(w) => cur = w,
                }
            }
        }
    }
}

mod part2 {
    use crate::data::*;
    use std::collections::HashSet;

    pub type RangeData = Data<(i64, i64)>;

    type Key<'a> = (&'a str, RangeData);
    type Seen<'a> = HashSet<Key<'a>>;

    fn width((x, y): (i64, i64)) -> i64 { y - x }

    impl RangeData {
        fn num_combinations(&self) -> i64 {
            width(self.x) * width(self.m) * width(self.a) * width(self.s)
        }

        pub fn from_range(start: i64, len: i64) -> Self {
            let r = (start, start + len);
            Data { x: r, m: r, a: r, s: r }
        }
    }

    impl<'a> Action<'a> {
        fn count(&'a self, rd: RangeData, workflows: &'a WorkflowSuite<'a>,
                 seen: &mut Seen<'a>) -> i64 {
            match self {
                Accept => rd.num_combinations(),
                Reject => 0,
                SendToWorkflow(w) =>
                    workflows.get(w).count(rd, workflows, seen),
            }
        }
    }

    impl Comp {
        fn split_range(self, (start, end): (i64, i64), v: i64)
                       -> (Option<(i64, i64)>, Option<(i64, i64)>) {
            let s = match self { Comp::Less => v, Comp::Greater => v + 1 };
            let (low, high) =
                if s <= start { (None, Some((start, end))) }
            else if end <= s { (Some((start, end)), None) }
            else { (Some((start, s)), Some((s, end))) };
            match self {
                Comp::Less => (low, high),
                Comp::Greater => (high, low),
            }
        }
    }

    impl Condition {
        fn split(&self, rd: &RangeData)
                 -> (Option<RangeData>, Option<RangeData>) {
            let &Condition{ prop: p, comp: c, value: v } = self;
            let (good_range, bad_range) = c.split_range(rd.get(p), v);
            let good = good_range.map(|g| rd.update(p, g));
            let bad = bad_range.map(|b| rd.update(p, b));
            (good, bad)
        }
    }

    impl Rule<'_> {
        fn split(&self, rd: &RangeData)
                 -> (Option<RangeData>, Option<RangeData>) {
            self.condition.split(rd)
        }
    }

    impl<'a> Workflow<'a> {
        fn count(&'a self, rd: RangeData, workflows: &'a WorkflowSuite<'a>,
                 seen: &mut Seen<'a>) -> i64 {
            let key = (self.name, rd);
            if seen.contains(&key) { panic!("cycle at {}!", self.name) };
            seen.insert(key);
            let mut cur = rd;
            let mut total = 0;
            for rule in self.rules.iter() {
                let (good, bad) = rule.split(&cur);
                if let Some(g) = good {
                    total += rule.action.count(g, workflows, seen);
                };
                if let Some(b) = bad {
                    cur = b;
                } else {
                    return total
                }
            }
            total += self.catchall.count(cur, workflows, seen);
            total
        }
    }

    impl WorkflowSuite<'_> {
        pub fn count(&self, rd: RangeData) -> i64 {
            self.get("in").count(rd, self, &mut HashSet::new())
        }
    }
}

use data::*;

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
    let suite = WorkflowSuite::new(workflows);

    let total: i64 = data.iter()
        .filter_map(|&d| suite.accepts(d).then(|| d.total()))
        .sum();

    println!("part 1: {total}");

    let combinations = suite.count(part2::RangeData::from_range(1, 4000));

    println!("part 2: {combinations}");
}
