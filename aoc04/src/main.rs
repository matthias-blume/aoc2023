use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn score_card(input: &str, multipliers: &mut HashMap<u32, u32>) -> (u32, u32) {
    let name_data_str: Vec<&str> = input.split(":").collect();
    let name_str = name_data_str[0];
    let card_number_str: Vec<&str> = name_str.split_whitespace().collect();
    let number: u32 = card_number_str[1].parse().expect("card number");
    let multiplier: u32 = *multipliers.get(&number).unwrap_or(&1);
    let data_str = name_data_str[1];
    let winning_have_str: Vec<&str> = data_str.split("|").collect();
    let winning: HashSet<u32> = winning_have_str[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let have: Vec<u32> = winning_have_str[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut count: u32 = 0;
    for h in have {
        if winning.contains(&h) {
            count = count+1
        }
    }
    for won_card in number+1..=number+count {
        *multipliers.entry(won_card).or_insert(1) += multiplier;
    }
    let points = if count > 0 { 1 << (count - 1) } else { 0 };  // for part 1
    (points, multiplier)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut multipliers = HashMap::new();
    let mut points_sum: u32 = 0;  // answer to part 1
    let mut card_count: u32 = 0;  // answer to part 2

    for line in contents.lines() {
        let (p, c) = score_card(line, &mut multipliers);
        points_sum += p;
        card_count += c;
    }

    println!("Points sum is {points_sum}, number of collected cards is {card_count}.");
}
