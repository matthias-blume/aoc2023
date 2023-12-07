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
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand(HandType, CardType, CardType, CardType, CardType, CardType);

fn card_type(c: char) -> CardType {
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
        'J' => CardType::CJ,
        'Q' => CardType::CQ,
        'K' => CardType::CK,
        'A' => CardType::CA,
        _ => panic!("bad card"),
    }
}

const TYPES: &'static [CardType] =
    &[CardType::C2, CardType::C3, CardType::C4, CardType::C5, CardType::C6,
      CardType::C7, CardType::C8, CardType::C9, CardType::CT, CardType::CJ,
      CardType::CQ, CardType::CK, CardType::CA];

fn run_with_augmented(j: i32, v: &mut Vec<CardType>, f: &mut impl FnMut(&Vec<CardType>) -> ()) {
    if j == 0 { f(&v) }
    else {
        for t in TYPES {
            v.push(*t);
            run_with_augmented(j-1, v, f);
            v.pop();
        }
    }
}

fn plain_hand_type(orig: &Vec<CardType>) -> HandType {
    let mut types = orig.clone();
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
    match counts.as_slice() {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("bad card counts {:?} {:?}", counts, types),
    }
}

fn hand_type(orig: Vec<CardType>) -> HandType {
    let mut best_ct: Vec<CardType> = orig.clone();
    let mut best_ht = plain_hand_type(&best_ct);
    let mut types: Vec<CardType> = orig.into_iter().filter(|&t| t != CardType::CJ).collect();
    let num_j = 5 - types.len() as i32;
    run_with_augmented(num_j, &mut types, &mut |t| { let ht = plain_hand_type(t); if ht > best_ht { best_ct = t.clone(); best_ht = ht } });
    best_ht
}

fn hand(s: &str) -> Hand {
    if s.len() != 5 { panic!("hand of wrong size") }
    let c1 = card_type(s.chars().nth(0).unwrap());
    let c2 = card_type(s.chars().nth(1).unwrap());
    let c3 = card_type(s.chars().nth(2).unwrap());
    let c4 = card_type(s.chars().nth(3).unwrap());
    let c5 = card_type(s.chars().nth(4).unwrap());

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

    let path = Path::new(&file_path);
    let file = File::open(&path).expect("open file");
    let reader = BufReader::new(file);

    let mut hand_bid_table = Vec::new();
    
    for line_result in reader.lines() {
        let line = line_result.expect("line");
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            [h, b] => hand_bid_table.push((hand(h), b.parse::<u64>().unwrap())), 
            _ => panic!("invalid input"),
        }
    }

    hand_bid_table.sort_by(|(x, _), (y, _)| x.cmp(y));

    let result: u64 = hand_bid_table.into_iter().enumerate().map(|(i, (_, v))| (i as u64 + 1)*v).sum();

    println!("{result}");
}
