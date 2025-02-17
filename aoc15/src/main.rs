// Advent-of-Code 2023
// Day 15
// Author: Matthias Blume

use std::env;
use std::fs;

use util::iter::*;

mod hash {
    pub fn ascii(s: &str) -> usize {
        s.as_bytes().iter().fold(0, |accu, &c| ((accu + c as usize) * 17) % 256)
    }
}

struct Lens<'a> {
    label: &'a str,
    strength: usize,
}

enum Instruction<'a> {
    AddLens{ label: &'a str, strength: usize, hash: usize },
    RemoveLens{ label: &'a str, hash: usize },
}
use crate::Instruction::*;

impl<'a> Instruction<'a> {
    fn from(input: &'a str) -> Self {
        match input.split("=").boxed()[..] {
            [label, s] => {
                let strength = s.parse().expect("lens strength");
                AddLens{ label, strength, hash: hash::ascii(label) }
            },
            _ => {
                if input.chars().last() != Some('-') {
                    panic!("bad instruction: '{}'", input)
                }
                let label = &input[0..input.len()-1];
                RemoveLens{ label, hash: hash::ascii(label) }
            }
        }
    }
}

struct Box<'a> {
    n: usize,  // box number
    lenses: Vec<Lens<'a>>,
}

impl<'a> Box<'a> {
    fn lens_position(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|lens| lens.label == label)
    }

    fn remove_lens(&mut self, label: &str) {
        if let Some(i) = self.lens_position(label) {
            self.lenses.remove(i);
        }
    }

    fn insert_lens(&mut self, label: &'a str, strength: usize) {
        let lens = Lens{ label, strength };
        if let Some(i) = self.lens_position(label) {
            self.lenses[i] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn strength(&self) -> usize {
        (self.n + 1) *
            self.lenses.iter().enumerate()
                .map(|(position, lens)| (position + 1) * lens.strength)
                .sum::<usize>()
    }
}

struct Boxes<'a>(Vec<Box<'a>>);

impl<'a> Boxes<'a> {
    fn apply_instruction(&mut self, ins: &'a str) {
        let Boxes(ref mut boxes) = self;
        match Instruction::from(ins) {
            AddLens{ label, strength, hash } =>
                boxes[hash].insert_lens(label, strength),
            RemoveLens{ label, hash } =>
                boxes[hash].remove_lens(label),
        }
    }

    fn apply_line(&mut self, line: &'a str) {
        line.split(",").for_each(|ins| self.apply_instruction(ins))
    }

    fn strength(&self) -> usize {
        let Boxes(boxes) = self;
        boxes.iter().map(|b| b.strength()).sum()
    }
}

mod part1 {
    pub fn line_total(line: &str) -> usize {
        line.split(",").map(crate::hash::ascii).sum()
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

    let mut boxes: Boxes =
        Boxes((0..=255).map(|n| Box{ n, lenses: Vec::new() }).collect());

    let mut part1_total = 0;
    
    for line in contents.lines() {
        part1_total += part1::line_total(line);
        boxes.apply_line(line);
    }

    let part2_total = boxes.strength();
    
    println!("part 1: {part1_total}, part 2: {part2_total}")
}
