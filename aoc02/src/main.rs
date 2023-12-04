use std::env;
use std::fs;

struct Triplet(u32, u32, u32);

fn read_color(input: &str) -> Triplet {
    let v: Vec<&str> = input.split_whitespace().collect();
    let value: u32 = v[0].parse().expect("color value");
    match v[1] {
        "red" => Triplet(value, 0, 0),
        "green" => Triplet(0, value, 0),
        "blue" => Triplet(0, 0, value),
        _ => panic!("invalid color name"),
    }
}

fn add_triplet(x: Triplet, y: Triplet) -> Triplet {
    Triplet(x.0 + y.0, x.1 + y.1, x.2 + y.2)
}

fn read_triplet(input: &str) -> Triplet {
    input.split(",").map(read_color).fold(Triplet(0, 0, 0), add_triplet)
}

fn read_triplets(input: &str) -> Vec<Triplet> {
    input.split(";").map(read_triplet).collect()
}

fn read_game_number(input: &str) -> u32 {
    match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
        ["Game", number_str] => number_str.parse().expect("number"),
        _ => panic!("missing or malformed Game spec"),
    }
}


fn read_game(line: &str) -> (u32, Vec<Triplet>) {
    let v: Vec<&str> = line.split(":").collect();
    (read_game_number(v[0]), read_triplets(v[1]))
}

fn triplet_possible(red: u32, green: u32, blue: u32, t: &Triplet) -> bool {
    t.0 <= red && t.1 <= green && t.2 <= blue
}

fn minimum_triplet_power(triplets: &Vec<Triplet>) -> u32 {
    // Notice the duality between min and max here.  The minimum *necessary* bag
    // triplet is computed by taking the component-wise max over all game triplets.
    let max_of = |f: fn(&Triplet) -> u32| triplets.iter().map(f).max().unwrap_or(0);
    max_of(|t| t.0) * max_of(|t| t.1) * max_of(|t| t.2)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let red = args[2].parse().expect("red");
    let green = args[3].parse().expect("green");
    let blue = args[4].parse().expect("blue");

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut possible_sum = 0;
    let mut power_sum = 0;
    
    for line in contents.lines() {
        let (number, triplets) = read_game(line);
        if triplets.iter().all(|t| triplet_possible(red, green, blue, t)) {
            possible_sum += number
        }
        power_sum += minimum_triplet_power(&triplets)
    }
    println!("sum of possible game numbers: {possible_sum}; sum of powers: {power_sum}")
}
