use std::env;
use std::fs;

fn ascii_hash(s: &str) -> u64 {
    s.as_bytes().iter().fold(0, |accu, &c| ((accu + c as u64) * 17) % 256)
}

fn line_total(line: &str) -> u64 {
    line.split(",").map(ascii_hash).sum()
}

struct Lens(String, u8);   // label, strength

struct Box(u8, Vec<Lens>);  // boxnum, lenses

type Boxes = Vec<Box>;

fn remove_lens(label: &str, b: &mut Box) {
    for i in 0..b.1.len() {
        if b.1[i].0 == label {
            b.1.remove(i);
            return
        }
    }
}

fn insert_lens(label: &str, strength: u8, b: &mut Box) {
    let l = Lens(label.to_string(), strength);
    for i in 0..b.1.len() {
        if b.1[i].0 == label {
            b.1[i] = l;
            return;
        }
    }
    b.1.push(l)
}

fn apply_instruction(ins: &str, boxes: &mut Boxes) {
    match ins.split("=").collect::<Vec<_>>().as_slice() {
        [label, strength] => {
            let h = ascii_hash(label);
            let s = strength.parse::<u8>().unwrap();
            insert_lens(label, s, &mut boxes[h as usize])
        },
        _ => {
            if ins.chars().last().unwrap() != '-' { panic!("bad instruction") };
            let label = &ins[..ins.len()-1];
            let h = ascii_hash(&label);
            remove_lens(label, &mut boxes[h as usize]);
        }
    }
}

fn apply_line(line: &str, boxes: &mut Boxes) {
    for ins in line.split(",") {
        apply_instruction(ins, boxes);
    }
}

fn box_strength(b: &Box) -> usize {
    b.1.iter().enumerate()
        .map(|(i, &Lens(_, s))| (b.0 as usize + 1) * (i + 1) * (s as usize))
        .sum()
}

fn total_strength(boxes: &Boxes) -> usize {
    boxes.iter().map(box_strength).sum()
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

    let mut boxes: Boxes = Vec::new();
    for i in 0..=255 {
        boxes.push(Box(i, Vec::new()));
    }

    let mut part1_total = 0;
    
    for line in contents.lines() {
        part1_total += line_total(line);
        apply_line(line, &mut boxes);
    }

    let part2_total = total_strength(&boxes);
    
    println!("part 1: {part1_total}, part 2: {part2_total}")
}
