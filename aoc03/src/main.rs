use std::env;
use std::fs;
use std::collections::HashSet;

fn symbol_locations_of(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let mut symbol_locations = HashSet::new();
    let mut star_locations = HashSet::new();
    let mut pos: i32 = 0;
    for c in line.chars() {
        if c != '.' && !c.is_digit(10) {
            symbol_locations.insert(pos);
            if c == '*' {
                star_locations.insert(pos);
            }
        }
        pos = pos + 1
    }
    (symbol_locations, star_locations)
}

// Each number is represented as a triplet: (value, start, end).
// The pair (start, end) marks the starting and ending positions
// within the line where the number was found.  Both are inclusive.
fn numbers_of(line: &str) -> Vec<(u32, i32, i32)> {
    let mut cur: Option<u32> = None;
    let mut start: i32 = 0;
    let mut pos: i32 = 0;
    let mut numbers = Vec::new();
    for c in line.chars() {
        if c.is_digit(10) {
            if let Some(value) = cur {
                // continue a number
                cur = Some(10 * value + c as u32 - '0' as u32)
            } else {
                // start new number
                cur = Some(c as u32 - '0' as u32);
                start = pos
            }
        } else if let Some(value) = cur {
            // record the a number
            numbers.push((value, start, pos-1));
            cur = None;
        }
        pos = pos + 1
    }
    if let Some(value) = cur {
        numbers.push((value, start, pos-1))
    }
    numbers
}

fn has_symbol_within_range(start: &i32, end: &i32, set: &HashSet<i32>) -> bool {
    (start-1 .. end+2).any(|pos| set.contains(&pos))
}

fn has_adjacent_symbol(start: &i32, end: &i32, prev: &HashSet<i32>, cur: &HashSet<i32>, next: &HashSet<i32>) -> bool {
    has_symbol_within_range(start, end, prev)
        || cur.contains(&(start-1))
        || cur.contains(&(end+1))
        || has_symbol_within_range(start, end, next)
}

fn parts_sum_of(numbers: &Vec<(u32, i32, i32)>, prev: &HashSet<i32>, cur: &HashSet<i32>, next: &HashSet<i32>) -> u32 {
    let mut sum: u32 = 0;
    for (number, start, end) in numbers.iter() {
        if has_adjacent_symbol(start, end, prev, cur, next) {
            sum += *number;
        }
    }
    sum
}

fn collect_adjacents(star: i32, numbers: &Vec<(u32, i32, i32)>, adjacents: &mut Vec<u32>) {
    for (value, start, end) in numbers {
        if star >= start-1 && star <= end+1 {
            adjacents.push(*value);
        }
    }
}

fn gear_ratio_sum_of(stars: &HashSet<i32>, prev: &Vec<(u32, i32, i32)>, cur: &Vec<(u32, i32, i32)>, next: &Vec<(u32, i32, i32)>) -> u32 {
    let mut sum: u32 = 0;
    for star in stars {
        let mut adjacents: Vec<u32> = Vec::new();
        collect_adjacents(*star, prev, &mut adjacents);
        collect_adjacents(*star, cur, &mut adjacents);
        collect_adjacents(*star, next, &mut adjacents);
        if adjacents.len() == 2 {
            sum += adjacents[0] * adjacents[1];
        }
    }
    sum
}

fn main() {
   let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut cur_symbols = HashSet::new();
    let mut cur_numbers = Vec::new();
    let mut next_symbols = HashSet::new();
    let mut next_stars = HashSet::new();
    let mut next_numbers = Vec::new();
    let mut parts_sum: u32 = 0;
    let mut gear_ratio_sum: u32 = 0;
    
    for line in contents.lines() {
        let prev_symbols = cur_symbols;
        let prev_numbers = cur_numbers;
        cur_symbols = next_symbols;
        let cur_stars = next_stars;
        cur_numbers = next_numbers;
        let (sym, sta) = symbol_locations_of(line);
        next_symbols = sym;
        next_stars = sta;
        next_numbers = numbers_of(line);
        parts_sum += parts_sum_of(&cur_numbers, &prev_symbols, &cur_symbols, &next_symbols);
        gear_ratio_sum += gear_ratio_sum_of(&cur_stars, &prev_numbers, &cur_numbers, &next_numbers);
    }
    parts_sum += parts_sum_of(&next_numbers, &cur_symbols, &next_symbols, &HashSet::new());
    gear_ratio_sum += gear_ratio_sum_of(&next_stars, &cur_numbers, &next_numbers, &Vec::new());
    println!("Parts sum is {parts_sum}.  Gear ratio sum is {gear_ratio_sum}.");
}
