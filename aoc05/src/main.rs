use std::env;
use std::fs;

struct RangeMap {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

struct Range(u64, u64);

fn seeds(spec: &[&str]) -> Vec<Range> {
    let mut v = Vec::new();
    let mut iter = spec.iter();
    while let Some(start_str) = iter.next() {
        let len_str = iter.next().unwrap();
        v.push(Range(start_str.parse().unwrap(), len_str.parse().unwrap()))
    }
    v
}

fn map_range_into(r: &Range, mapping: &Vec<RangeMap>, dest: &mut Vec<Range>) {
    let Range(mut rstart, mut rlen) = r;
    for &RangeMap{ dst_start, src_start, len } in mapping {
        if rstart < src_start {
            if rstart + rlen <= src_start {
                break
            }
            let l = src_start - rstart;
            rlen -= l;
            dest.push(Range(rstart, l));
            rstart = src_start;
        }
        if rstart < src_start + len {
            if rstart + rlen <= src_start + len {
                // fully contained
                dest.push(Range(dst_start + (rstart - src_start), rlen));
                return
            }
            // overhang
            let l = rstart + rlen - src_start - len;
            dest.push(Range(dst_start + (rstart - src_start), src_start + len - rstart));
            rstart = src_start + len;
            rlen = l;
        }
    }
    if rlen > 0 {
        dest.push(Range(rstart, rlen))
    }
}

fn apply_mapping(cur: Vec<Range>, mapping: &mut Vec<RangeMap>) -> Vec<Range> {
    mapping.sort_by(|a, b| a.src_start.cmp(&b.src_start));
    let mut result = Vec::new();
    cur.iter().for_each(|r| map_range_into(r, mapping, &mut result));
    result
}

fn main() {
    if let [_, file_path] = env::args().collect::<Vec<_>>().as_slice() {
    
        let contents = fs::read_to_string(file_path)
            .expect("Could not read file");

        let mut cur = Vec::new();
        let mut mapping = Vec::new();
        
        for line in contents.lines() {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["seeds:", seeds_strings @ ..] => cur = seeds(seeds_strings),
                [_, "map:"] => { cur = apply_mapping(cur, &mut mapping); mapping = Vec::new() },
                [d, s, l] =>
                    mapping.push(RangeMap{ dst_start: d.parse().unwrap(),
                                           src_start: s.parse().unwrap(),
                                           len: l.parse().unwrap() }),
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
