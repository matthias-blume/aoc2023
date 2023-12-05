use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::cmp::min;
use std::cmp::max;

#[derive(Clone, Copy)]
struct Range(u64, u64); // (start, len)

struct RangeMap {
    src: Range,
    dst_start: u64,
}

// Build valid range from start and end point (if possible).
fn start_end_range(s: u64, e: u64) -> Option<Range> {
    if e > s { Some(Range(s, e - s)) } else { None }
}

// Left overhang (non-empty portion of x that lies outside to the left of y).
fn range_left_overhang(Range(xs, xl): Range, Range(ys, _): Range) -> Option<Range> {
    start_end_range(xs, min(ys, xs + xl))
}

// Intersection of x and y (non-empty portion that lies within both).
fn range_intersection(Range(xs, xl): Range, Range(ys, yl): Range) -> Option<Range> {
    start_end_range(max(xs, ys), min(xs + xl, ys + yl))
}

// Right overhang (non-empty portion of x that lies outside to the right of y).
fn range_right_overhang(Range(xs, xl): Range, Range(ys, yl): Range) -> Option<Range> {
    start_end_range(max(xs, ys + yl), xs + xl)
}

// Apply a RangeMap to a single Range, assuming that it lies fully within the
// source range.
fn map_single_range(Range(s, l): Range, m: &RangeMap) -> Range {
    Range(s + m.dst_start - m.src.0, l)
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
fn map_range_into(range: Range, sorted_mapping: &Vec<RangeMap>, dest: &mut Vec<Range>) {
    let mut x: Range = range;
    for rm in sorted_mapping {
        if let Some(l) = range_left_overhang(x, rm.src) { dest.push(l) }
        if let Some(m) = range_intersection(x, rm.src) { dest.push(map_single_range(m, &rm)) }
        if let Some(r) = range_right_overhang(x, rm.src) { x = r }
        else { return }
    }
    dest.push(x)
}

// Sorts the mapping and then applies it to all given ranges, resulting in
// a new list of ranges.
fn apply_mapping(cur: Vec<Range>, mapping: &mut Vec<RangeMap>) -> Vec<Range> {
    mapping.sort_by(|a, b| a.src.0.cmp(&b.src.0));
    let mut result = Vec::new();
    cur.iter().for_each(|r| map_range_into(*r, mapping, &mut result));
    result
}

// Upon seeing a new map type, checks that the old kind matches the
// map's source.  Then returns the new kind.
fn changed_kind(kind: String, map_type: &str) -> String {
    match map_type.split("-").collect::<Vec<_>>().as_slice() {
        [from, "to", to] => {
            if kind != *from { panic!("wrong transition for {}: {}-to-{}", kind, from, to) }
            String::from(*to)
        },
        _ => panic!("bad map type: {}", map_type),
    }
}

fn main() {
    if let [_, file_path] = env::args().collect::<Vec<_>>().as_slice() {

        let path = Path::new(file_path);
        let file = File::open(&path).expect("open file");
        let reader = BufReader::new(file);
        
        let mut cur = Vec::new();
        let mut mapping = Vec::new();
        let mut kind = String::from("unknown kind");
        
        for line_result in reader.lines() {
            let line = line_result.expect("line");
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["seeds:", seeds_strings @ ..] => {
                    kind = String::from("seed");
                    cur = seeds(seeds_strings)
                },
                [map_type, "map:"] => {
                    cur = apply_mapping(cur, &mut mapping);
                    mapping = Vec::new();
                    kind = changed_kind(kind, map_type)
                },
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
        println!("Lowest {kind} is {smallest}");
    } else {
        panic!("file path argument");
    }
}
