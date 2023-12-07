use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum CardType {
    CJoker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand(HandType, CardType, CardType, CardType, CardType, CardType);

fn card_type(c: char, treat_j_as_joker: bool) -> CardType {
    match c {
        '2' => CardType::C2,
        '3' => CardType::C3,
        '4' => CardType::C4,
        '5' => CardType::C5,
        '6' => CardType::C6,
        '7' => CardType::C7,
        '8' => CardType::C8,
        '9' => CardType::C9,
        'T' => CardType::CT,
        'J' if treat_j_as_joker => CardType::CJoker,
        'J' => CardType::CJ,
        'Q' => CardType::CQ,
        'K' => CardType::CK,
        'A' => CardType::CA,
        _ => panic!("bad card"),
    }
}

fn hand_type(mut types: Vec<CardType>) -> HandType {
    let jokers = types.iter().filter(|&t| t == &CardType::CJoker).count();
    types.sort();
    let mut counts = Vec::new();
    let mut n = 1;
    let mut last = types[0];
    for i in 1..5 {
        let cur = types[i];
        if cur == last {
            n += 1;
        } else {
            counts.push(n);
            n = 1;
            last = cur;
        }
    }
    counts.push(n);
    counts.sort();
    match (counts.as_slice(), jokers) {
        ([5], _) => HandType::FiveOfAKind,
        ([1, 4], 0) => HandType::FourOfAKind,
        ([1, 4], _) => HandType::FiveOfAKind,
        ([2, 3], 0) => HandType::FullHouse,
        ([2, 3], _) => HandType::FiveOfAKind,
        ([1, 1, 3], 0) => HandType::ThreeOfAKind,
        ([1, 1, 3], _) => HandType::FourOfAKind,
        ([1, 2, 2], 0) => HandType::TwoPair,
        ([1, 2, 2], 1) => HandType::FullHouse,
        ([1, 2, 2], 2) => HandType::FourOfAKind,
        ([1, 1, 1, 2], 0) => HandType::OnePair,
        ([1, 1, 1, 2], _) => HandType::ThreeOfAKind,
        ([1, 1, 1, 1, 1], 0) => HandType::HighCard,
        ([1, 1, 1, 1, 1], 1) => HandType::OnePair,
        _ => panic!("bad card counts {:?} {:?}", counts, types),
    }
}

fn hand(s: &str, treat_j_as_joker: bool) -> Hand {
    if s.len() != 5 { panic!("hand of wrong size") }
    let c1 = card_type(s.chars().nth(0).unwrap(), treat_j_as_joker);
    let c2 = card_type(s.chars().nth(1).unwrap(), treat_j_as_joker);
    let c3 = card_type(s.chars().nth(2).unwrap(), treat_j_as_joker);
    let c4 = card_type(s.chars().nth(3).unwrap(), treat_j_as_joker);
    let c5 = card_type(s.chars().nth(4).unwrap(), treat_j_as_joker);

    Hand(hand_type(Vec::from([c1, c2, c3, c4, c5])), c1, c2, c3, c4, c5)
}
    

fn main() {
    let mut args = env::args();
    let program =
        if let Some(arg) = args.next() { arg }
        else { panic!("no program name") };
    let file_path =
        if let Some(arg) = args.next() { arg }
        else { panic!("{}: no file path argument", program) };
    let treat_j_as_joker =
        match args.next() {
            Some(arg) if arg == "--treat_j_as_joker" => true,
            _ => false
        };

    let path = Path::new(&file_path);
    let file = File::open(&path).expect("open file");
    let reader = BufReader::new(file);

    let mut hand_bid_table = Vec::new();
    
    for line_result in reader.lines() {
        let line = line_result.expect("line");
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            [h, b] => hand_bid_table.push((hand(h, treat_j_as_joker), b.parse::<u64>().unwrap())), 
            _ => panic!("invalid input"),
        }
    }

    hand_bid_table.sort_by(|(x, _), (y, _)| x.cmp(y));

    let result: u64 = hand_bid_table.into_iter().enumerate().map(|(i, (_, v))| (i as u64 + 1)*v).sum();

    println!("{result}");
}
