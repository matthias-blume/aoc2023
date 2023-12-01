use std::env;
use std::fs;

enum State {
    Start,
    O,
    ON,
    T,
    TW,
    TH,
    THR,
    THRE,
    F,
    FO,
    FOU,
    FI,
    FIV,
    S,
    SI,
    SE,
    SEV,
    SEVE,
    E,
    EI,
    EIG,
    EIGH,
    N,
    NI,
    NIN,
}

enum LineState {
    NoDigitYet,
    DigitSeen(u32),  // carries value of last digit seen
}

fn next_state(s: State, c: char) -> (State, Option<u32>) {
    // KMP-style transition table.
    // Notice that this will also recognize things like
    // "twone" as 2 followed by 1.
    // This is what the problem seems to require (although
    // it isn't explained carefully in the description).
    match (s, c) {
        (State::F, 'o') => (State::FO, None),
        (State::TW, 'o') => (State::O, Some(2)),
        (_, 'o') => (State::O, None),
        (State::O, 'n') => (State::ON, None),
        (State::ON, 'e') => (State::E, Some(1)),
        (State::ON, 'i') => (State::NI, None),
        (State::EIGH, 't') => (State::T, Some(8)),
        (_, 't') => (State::T, None),
        (State::T, 'w') => (State::TW, None),
        (State::T, 'h') => (State::TH, None),
        (State::TH, 'r') => (State::THR, None),
        (State::THR, 'e') => (State::THRE, None),
        (State::THRE, 'e') => (State::E, Some(3)),
        (State::THRE, 'i') => (State::EI, None),
        (_, 'f') => (State::F, None),
        (State::FO, 'u') => (State::FOU, None),
        (State::FO, 'n') => (State::ON, None),
        (State::FOU, 'r') => (State::Start, Some(4)),
        (State::F, 'i') => (State::FI, None),
        (State::FI, 'v') => (State::FIV, None),
        (State::FIV, 'e') => (State::E, Some(5)),
        (_, 's') => (State::S, None),
        (State::S, 'i') => (State::SI, None),
        (State::SI, 'x') => (State::Start, Some(6)),
        (State::S, 'e') => (State::SE, None),
        (State::SE, 'v') => (State::SEV, None),
        (State::SE, 'i') => (State::EI, None),
        (State::SEV, 'e') => (State::SEVE, None),
        (State::SEVE, 'n') => (State::N, Some(7)),
        (State::SEVE, 'i') => (State::EI, None),
        (State::NIN, 'e') => (State::E, Some(9)),
        (State::NIN, 'i') => (State::NI, None),
        (_, 'e') => (State::E, None),
        (State::E, 'i') => (State::EI, None),
        (State::EI, 'g') => (State::EIG, None),
        (State::EIG, 'h') => (State::EIGH, None),
        (State::NI, 'n') => (State::NIN, None),
        (_, 'n') => (State::N, None),
        (State::N, 'i') => (State::NI, None),
        _ => (State::Start,
              if c.is_digit(10) { Some(c as u32 - '0' as u32) }
              else { None })
    }
}

fn last_digit_value(s: LineState) -> u32 {
    match s {
        LineState::DigitSeen(last) => last,
        _ => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut tally: u32 = 0;
    let mut state = State::Start;
    let mut line_state =  LineState::NoDigitYet;

    for c in contents.chars() {
        if c == '\n' {
            tally += last_digit_value(line_state);
            state = State::Start;
            line_state = LineState::NoDigitYet;
        } else {
            let (new_state, opt_dig) = next_state(state, c);
            state = new_state;
            match opt_dig {
                None => (),
                Some(value) => {
                    match line_state {
                        LineState::NoDigitYet => tally += value * 10,
                        _ => (),
                    }
                    line_state = LineState::DigitSeen(value);
                }
            }
        }
    }
    // Deal with incomplete last line (missing newline):
    tally += last_digit_value(line_state);
    println!("tally is {tally}");
}
