use std::env;
use std::fs;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

enum TileConfig {
    Empty,
    DiagDown,
    DiagUp,
    Hor,
    Vert,
}
use TileConfig::*;

#[derive(PartialEq, Eq, Clone, Copy)]
struct TileState {
    up_in: bool,
    up_out: bool,
    down_in: bool,
    down_out: bool,
    left_in: bool,
    left_out: bool,
    right_in: bool,
    right_out: bool,
}

type Config = Vec<Vec<TileConfig>>;
type State = Vec<Vec<TileState>>;

fn fresh_tile_state() -> TileState {
    TileState { up_in: false, up_out: false, down_in: false, down_out: false, left_in: false, left_out: false, right_in: false, right_out: false }
}

fn tile_config(c: char) -> TileConfig {
    match c {
        '.' => Empty,
        '\\' => DiagDown,
        '/' => DiagUp,
        '-' => Hor,
        '|' => Vert,
        _ => panic!("bad tile"),
    }
}

fn read_line(line: &str) -> Vec<TileConfig> {
    line.chars().map(tile_config).collect()
}

fn propagate_light(from: Direction, (i, j): (i64, i64), config: &Config, state: &mut State) {
    if i < 0 || j < 0 || i as usize >= config.len() || j as usize >= config[0].len() {
        return
    }
    let old_state = state[i as usize][j as usize];
    let cfg = &config[i as usize][j as usize];
    let mut new_state = old_state.clone();
    let mut go_left = false;
    let mut go_right = false;
    let mut go_up = false;
    let mut go_down = false;
    match from {
        Up => {
            new_state = TileState{ up_in: true, ..new_state };
            match cfg {
                DiagDown => {
                    new_state = TileState { right_out: true, ..new_state };
                    go_right = true;
                },
                DiagUp => {
                    new_state = TileState { left_out: true, ..new_state };
                    go_left = true;
                },
                Hor => {
                    new_state = TileState { left_out: true, right_out: true, ..new_state };
                    go_right = true;
                    go_left = true;
                },
                Vert | Empty => {
                    new_state = TileState { down_out: true, ..new_state };
                    go_down = true;
                },
            }
        },
        Down => {
            new_state = TileState{ down_in: true, ..new_state };
            match cfg {
                DiagUp => {
                    new_state = TileState { right_out: true, ..new_state };
                    go_right = true;
                },
                DiagDown => {
                    new_state = TileState { left_out: true, ..new_state };
                    go_left = true;
                },
                Hor => {
                    new_state = TileState { left_out: true, right_out: true, ..new_state };
                    go_right = true;
                    go_left = true;
                },
                Vert | Empty => {
                    new_state = TileState { up_out: true, ..new_state };
                    go_up = true;
                },
            }
        },
        Left => {
            new_state = TileState{ left_in: true, ..new_state };
            match cfg {
                DiagUp => {
                    new_state = TileState { up_out: true, ..new_state };
                    go_up = true;
                },
                DiagDown => {
                    new_state = TileState { down_out: true, ..new_state };
                    go_down = true;
                },
                Vert => {
                    new_state = TileState { up_out: true, down_out: true, ..new_state };
                    go_up = true;
                    go_down = true;
                },
                Hor | Empty => {
                    new_state = TileState { right_out: true, ..new_state };
                    go_right = true;
                },
            }
        },
        Right => {
            new_state = TileState{ left_in: true, ..new_state };
            match cfg {
                DiagDown => {
                    new_state = TileState { up_out: true, ..new_state };
                    go_up = true;
                },
                DiagUp => {
                    new_state = TileState { down_out: true, ..new_state };
                    go_down = true;
                },
                Vert => {
                    new_state = TileState { up_out: true, down_out: true, ..new_state };
                    go_up = true;
                    go_down = true;
                },
                Hor | Empty => {
                    new_state = TileState { left_out: true, ..new_state };
                    go_left = true;
                },
            }
        },
    }
    if old_state != new_state {
        state[i as usize][j as usize] = new_state;
        if go_right { propagate_light(Left, (i, j+1), config, state) };
        if go_left { propagate_light(Right, (i, j-1), config, state) };
        if go_down { propagate_light(Up, (i+1, j), config, state) };
        if go_up { propagate_light(Down, (i-1, j), config, state) };
    }
}

fn is_active(s: &TileState) -> bool {
    s.up_out || s.down_out || s.right_out || s.left_out
}

fn calc_num_active(state: &State) -> usize {
    let mut num_active = 0;
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            if is_active(&state[i][j]) {
                num_active += 1;
            }
        }
    }
    num_active
}

fn final_state(from: Direction, i: usize, j: usize, config: &Config) -> State {
    let mut state: State =
        config.iter().map(|row| row.iter().map(|_| fresh_tile_state()).collect()).collect();
    propagate_light(from, (i as i64, j as i64), config, &mut state);
    state
}

fn main() {
    let mut args = env::args();
    let program = match args.next() {
        Some(arg) => arg,
        _ => panic!("no program name"),
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        _ => panic!("{}: no input file name", program),
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let mut config: Config = Vec::new();
   
    for line in contents.lines() {
        config.push(read_line(&line));
    }

    let mut max_active = 0;
    let maxi = config.len();
    let maxj = config[0].len();
    for i in 0..maxi {
        let state = final_state(Left, i, 0, &config);
        let active = calc_num_active(&state);
        if active > max_active {
            max_active = active;
        }
        let state = final_state(Right, i, maxj-1, &config);
        let active = calc_num_active(&state);
        if active > max_active {
            max_active = active;
        }
    }
    for j in 0..maxj {
        let state = final_state(Up, 0, j, &config);
        let active = calc_num_active(&state);
        if active > max_active {
            max_active = active;
        }
        let state = final_state(Down, maxi-1, j, &config);
        let active = calc_num_active(&state);
        if active > max_active {
            max_active = active;
        }
    }
    
    let state = final_state(Left, 0, 0, &config);

    let part1 = calc_num_active(&state);
    
    println!("{part1} {max_active}");
}
