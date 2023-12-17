// Advent-of-Code 2023
// Day 03
// Author: Matthias Blume

use std::env;
use std::fs;
use std::collections::HashSet;

fn symbol_and_star_locations_of(line: &str) -> (HashSet<i32>, HashSet<i32>) {
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
struct Number(u32, i32, i32);

fn numbers_of(line: &str) -> Vec<Number> {
    let mut cur = None;
    let mut start: i32 = 0;
    let mut pos: i32 = 0;
    let mut numbers = Vec::new();
    for c in line.chars() {
        match (c.is_digit(10), cur) {
            (true, Some(value)) => cur = Some(10 * value + c as u32 - '0' as u32),
            (true, None) => { cur = Some(c as u32 - '0' as u32); start = pos },
            (false, Some(value)) => { numbers.push(Number(value, start, pos-1)); cur = None },
            _ => (),
        }
        pos = pos + 1
    }
    if let Some(value) = cur {
        numbers.push(Number(value, start, pos-1))
    }
    numbers
}

fn has_symbol_within_range(start: i32, end: i32, set: &HashSet<i32>) -> bool {
    (start-1..=end+1).any(|pos| set.contains(&pos))
}

fn has_adjacent_symbol(start: i32, end: i32, prev: &HashSet<i32>, cur: &HashSet<i32>, next: &HashSet<i32>) -> bool {
    has_symbol_within_range(start, end, prev)
        || cur.contains(&(start-1))
        || cur.contains(&(end+1))
        || has_symbol_within_range(start, end, next)
}

fn parts_sum_of(numbers: &Vec<Number>, prev: &HashSet<i32>, cur: &HashSet<i32>, next: &HashSet<i32>) -> u32 {
    let mut sum: u32 = 0;
    for Number(number, start, end) in numbers.iter() {
        if has_adjacent_symbol(*start, *end, prev, cur, next) {
            sum += number;
        }
    }
    sum
}

fn collect_adjacents(star: i32, numbers: &Vec<Number>, adjacents: &mut Vec<u32>) {
    for Number(value, start, end) in numbers {
        if star >= start-1 && star <= end+1 {
            adjacents.push(*value);
        }
    }
}

fn gear_ratio_sum_of(stars: &HashSet<i32>, prev: &Vec<Number>, cur: &Vec<Number>, next: &Vec<Number>) -> u32 {
    let mut sum: u32 = 0;
    for star in stars {
        let mut adjacents = Vec::new();
        collect_adjacents(*star, prev, &mut adjacents);
        collect_adjacents(*star, cur, &mut adjacents);
        collect_adjacents(*star, next, &mut adjacents);
        if let [adj1, adj2] = adjacents.as_slice() {
            sum += adj1 * adj2
        }
    }
    sum
}

fn main() {
   let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let (mut cur_symbols, mut next_symbols, mut next_stars)
        = (HashSet::new(), HashSet::new(), HashSet::new());
    let (mut cur_numbers, mut next_numbers)
        = (Vec::new(), Vec::new());
    let mut parts_sum: u32 = 0;
    let mut gear_ratio_sum: u32 = 0;
    
    for line in contents.lines() {
        let (prev_symbols, prev_numbers) = (cur_symbols, cur_numbers);
        let cur_stars = next_stars;
        (cur_symbols, cur_numbers) = (next_symbols, next_numbers);
        (next_symbols, next_stars) = symbol_and_star_locations_of(line);
        next_numbers = numbers_of(line);
        parts_sum += parts_sum_of(&cur_numbers, &prev_symbols, &cur_symbols, &next_symbols);
        gear_ratio_sum += gear_ratio_sum_of(&cur_stars, &prev_numbers, &cur_numbers, &next_numbers);
    }
    parts_sum += parts_sum_of(&next_numbers, &cur_symbols, &next_symbols, &HashSet::new());
    gear_ratio_sum += gear_ratio_sum_of(&next_stars, &cur_numbers, &next_numbers, &Vec::new());
    println!("Parts sum is {parts_sum}.  Gear ratio sum is {gear_ratio_sum}.");
}
