use std::env;
use std::fs;

fn read_color(input: &str) -> (u32, u32, u32) {
    let v: Vec<&str> = input.split_whitespace().collect();
    let value: u32 = v[0].parse().unwrap();
    match v[1] {
        "red" => (value, 0, 0),
        "green" => (0, value, 0),
        "blue" => (0, 0, value),
        _ => (0, 0, 0),
    }
}

fn add_triplet(x: (u32, u32, u32), y: (u32, u32, u32)) -> (u32, u32, u32) {
    let (xr, xg, xb) = x;
    let (yr, yg, yb) = y;
    (xr + yr, xg + yg, xb + yb)
}

fn read_triplet(input: &str) -> (u32, u32, u32) {
    input.split(",").map(read_color).fold((0, 0, 0), add_triplet)
}

fn read_triplets(input: &str) -> Vec<(u32, u32, u32)> {
    input.split(";").map(read_triplet).collect()
}

fn read_game_number(input: &str) -> u32 {
    let v: Vec<&str> = input.split_whitespace().collect();
    v[1].parse().unwrap()
}


fn read_game(line: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let v: Vec<&str> = line.split(":").collect();
    (read_game_number(v[0]), read_triplets(v[1]))
}

fn triplet_possible(red: u32, green: u32, blue: u32, x: &(u32, u32, u32)) -> bool {
    let (r, g, b) = x;
    *r <= red && *g <= green && *b <= blue
}

fn minimum_triplet_power(triplets: &Vec<(u32, u32, u32)>) -> u32 {
    let mut min_red: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_blue: u32 = 0;
    for (r, g, b) in triplets.iter() {
        if *r > min_red { min_red = *r }
        if *g > min_green { min_green = *g }
        if *b > min_blue { min_blue = *b }
    }
    min_red * min_green * min_blue
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let red: u32 = args[2].parse().unwrap();
    let green: u32 = args[3].parse().unwrap();
    let blue: u32 = args[4].parse().unwrap();

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut possible_sum: u32 = 0;
    let mut power_sum: u32 = 0;
    
    for line in contents.lines() {
        let (number, triplets) = read_game(line);
        if triplets.iter().all(|t| triplet_possible(red, green, blue, t)) {
            possible_sum += number
        }
        power_sum += minimum_triplet_power(&triplets)
    }
    println!("sum of possible game numbers: {possible_sum}; sum of powers: {power_sum}")
}
