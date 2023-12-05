use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;

struct Range(u64, u64); // (start, len)

struct RangeMap {
    src: Range,
    dst_start: u64,
}

// Describe relation of Range x relative to Range y:
enum RangeRelation {
    L,    // x fully to the left of y
    LM,   // x overlaps y but also overhangs to the left
    LMR,  // x overlaps y but overhangs to both sides
    M,    // x fully within y
    MR,   // x overlaps y but also overhang to the right
    R,    // x fully to the right of y
}

// Determines RangeRelation of a Range x relative to a Range y.
fn range_relation(Range(xs, xl): &Range, Range(ys, yl): &Range) -> RangeRelation {
    if xs < ys {
        if xs + xl <= *ys {
            RangeRelation::L
        } else if *xs + *xl <= *ys + *yl {
            RangeRelation::LM
        } else {
            RangeRelation::LMR
        }
    } else if *xs < *ys + *yl {
        if xs + xl <= *ys + *yl {
            RangeRelation::M
        } else {
            RangeRelation::MR
        }
    } else {
        RangeRelation::R
    }
}

// Reads seed values in pairs (start, len).
fn seeds(spec: &[&str]) -> Vec<Range> {
    let mut v = Vec::new();
    let mut iter = spec.iter();
    while let Some(start_str) = iter.next() {
        let len_str = iter.next().unwrap();
        v.push(Range(start_str.parse().unwrap(), len_str.parse().unwrap()))
    }
    v
}

// Apply full mapping to a single Range.
//
// The mapping can split a single range into multiple ranges depending on
// how it intersects with the various source ranges within the mapping.
//
// The mapping is sorted by increasing source ranges.
fn map_range_into(r: &Range, sorted_mapping: &Vec<RangeMap>, dest: &mut Vec<Range>) {
    let mut x: Range = Range(r.0, r.1);  // start with copy of r
    for RangeMap{ src, dst_start } in sorted_mapping {
        // Each RangeMap splits x into 1, 2, or 3 parts.  If there is a "right" part,
        // then continue considering remaining RangeMap instances (which are all
        // located to the right).  Otherwise return.
        // If there is a "middle" part (an overlap of x with the src range, then
        // map that to the corresponding destination range using dst_start.
        match range_relation(&x, &src) {
            RangeRelation::L => { dest.push(x); return },
            RangeRelation::LM => { dest.push(Range(x.0, src.0 - x.0));
                                   dest.push(Range(*dst_start, x.1 - (src.0 - x.0)));
                                   return },
            RangeRelation::LMR => { dest.push(Range(x.0, src.0 - x.0));
                                    dest.push(Range(*dst_start, src.1));
                                    x = Range(src.0 + src.1, x.1 - (src.0 - x.0) - src.1) },
            RangeRelation::M => { dest.push(Range(*dst_start + x.0 - src.0, x.1)); return },
            RangeRelation::MR => { dest.push(Range(*dst_start + x.0 - src.0, src.1 - (x.0 - src.0)));
                                   x = Range(src.0 + src.1, x.1 - (src.1 - (x.0 - src.0))) },
            RangeRelation::R => (),
        }
    }
    dest.push(x)
}

// Sorts the mapping and then applies it to all given ranges, resulting in
// a new list of ranges.
fn apply_mapping(cur: Vec<Range>, mapping: &mut Vec<RangeMap>) -> Vec<Range> {
    mapping.sort_by(|a, b| a.src.0.cmp(&b.src.0));
    let mut result = Vec::new();
    cur.iter().for_each(|r| map_range_into(r, mapping, &mut result));
    result
}

fn main() {
    if let [_, file_path] = env::args().collect::<Vec<_>>().as_slice() {

        let path = Path::new(file_path);
        let file = File::open(&path).expect("open file");
        let reader = BufReader::new(file);
        
        let mut cur = Vec::new();
        let mut mapping = Vec::new();
        
        for line_result in reader.lines() {
            let line = line_result.expect("line");
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["seeds:", seeds_strings @ ..] => cur = seeds(seeds_strings),
                [_, "map:"] => { cur = apply_mapping(cur, &mut mapping); mapping = Vec::new() },
                [d, s, l] =>
                    mapping.push(RangeMap{ dst_start: d.parse().unwrap(),
                                           src: Range(s.parse().unwrap(),
                                                      l.parse().unwrap()) }),
                [] => (),
                _ => panic!("invalid input"),
            }
        }
        cur = apply_mapping(cur, &mut mapping);
        let smallest = cur.iter().map(|r| r.0).min().unwrap();
        println!("Lowest location is {smallest}");
    } else {
        panic!("file path argument");
    }
}
