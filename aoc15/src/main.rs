use std::env;
use std::fs;

struct Lens {
    label: String,
    strength: u8,
}

struct Box{
    n: u8,  // box number
    lenses: Vec<Lens>,
}

struct Boxes(Vec<Box>);

fn ascii_hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |accu, &c| ((accu + c as usize) * 17) % 256)
}

fn line_total(line: &str) -> usize {
    line.split(",").map(ascii_hash).sum()
}

impl Box {
    fn lens_position(self: &Self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|lens| lens.label == label)
    }

    fn remove_lens(self: &mut Self, label: &str) {
        if let Some(i) = self.lens_position(label) {
            self.lenses.remove(i);
        }
    }

    fn insert_lens(self: &mut Self, label: &str, strength: u8) {
        let lens = Lens{ label: label.to_string(), strength };
        if let Some(i) = self.lens_position(label) {
            self.lenses[i] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn strength(self: &Self) -> usize {
        (self.n as usize + 1) *
            self.lenses.iter().enumerate()
                .map(|(position, lens)|
                     (position + 1) * (lens.strength as usize))
                .sum::<usize>()
    }
}

impl Boxes {
    fn apply_instruction(self: &mut Self, ins: &str) {
        let Boxes(ref mut boxes) = self;
        match ins.split("=").collect::<Vec<_>>().as_slice() {
            [label, strength] => {
                let h = ascii_hash(label);
                let s = strength.parse::<u8>().expect("lens strength");
                boxes[h].insert_lens(label, s)
            },
            _ => {
                if ins.chars().last() != Some('-') { panic!("bad instruction") };
                let label = &ins[..ins.len()-1];
                let h = ascii_hash(&label);
                boxes[h].remove_lens(label);
            }
        }
    }

    fn apply_line(self: &mut Self, line: &str) {
        line.split(",").for_each(|ins| self.apply_instruction(ins))
    }

    fn strength(self: &Self) -> usize {
        let Boxes(boxes) = self;
        boxes.iter().map(|b| b.strength()).sum()
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
        part1_total += line_total(line);
        boxes.apply_line(line);
    }

    let part2_total = boxes.strength();
    
    println!("part 1: {part1_total}, part 2: {part2_total}")
}
