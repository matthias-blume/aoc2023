use std::env;
use std::fs;
use std::ops::{Index,IndexMut};
use std::cmp::max;

type Pos = (i64, i64);

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    // Increments position according to the direction.
    fn incr(self, (i, j): Pos) -> Pos {
        match self {
            Left => (i, j-1),
            Right => (i, j+1),
            Up => (i-1, j),
            Down => (i+1, j),
        }
    }
}

#[derive(Copy, Clone)]
enum TileConfig {
    Empty,
    DiagDown,
    DiagUp,
    Hor,
    Vert,
}
use TileConfig::*;

impl TileConfig {
    fn from(c: char) -> Self {
        match c {
            '.' => Empty,
            '\\' => DiagDown,
            '/' => DiagUp,
            '-' => Hor,
            '|' => Vert,
            _ => panic!("bad tile"),
        }
    }
}

struct TileState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl TileState {
    fn new() -> TileState {
        TileState { up: false, down: false, left: false, right: false }
    }
}

impl Index<Direction> for TileState {
    type Output = bool;
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Up => &self.up,
            Down => &self.down,
            Left => &self.left,
            Right => &self.right,
        }
    }
}

impl IndexMut<Direction> for TileState {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Up => &mut self.up,
            Down => &mut self.down,
            Left => &mut self.left,
            Right => &mut self.right,
        }
    }
}

type State = Vec<Vec<TileState>>;

struct Config {
    height: usize,
    width: usize,
    board: Vec<Vec<TileConfig>>,
}

impl Config {
    fn new(b: Vec<Vec<TileConfig>>) -> Self {
        Config{
            height: b.len(),
            width: if b.len() > 0 { b[0].len() } else { 0 },
            board: b,
        }
    }

    // Fresh State with same dimensions as the Config.
    fn new_state(&self) -> State {
        (0..self.height).map(
            |_| (0..self.width).map(|_| TileState::new()).collect())
            .collect()
    }

    // Get individual tile configuration unless position is out of bounds.
    fn get(&self, p: Pos) -> Option<TileConfig> {
        if p.0 < 0 || p.0 as usize >= self.height ||
            p.1 < 0 || p.1 as usize >= self.width { None }
        else { Some(self.board[p.0 as usize][p.1 as usize]) }
    }
}

fn read_line(line: &str) -> Vec<TileConfig> {
    line.chars().map(TileConfig::from).collect()
}

// Update state to indicate that light is going out in the given
// direction.  If this update changes the state, then propagate
// light in that direction.
fn maybe_propagate(d: Direction, p: Pos, config: &Config, state: &mut State) {
    let st = &mut state[p.0 as usize][p.1 as usize][d];
    let was_set = *st;
    *st = true;
    if !was_set { propagate(d, d.incr(p), config, state) }
}

// Propagate light into a position, going in the given direction.
fn propagate(d: Direction, p: Pos, config: &Config, state: &mut State) {
    if let Some(cfg) = config.get(p) {
        let (d1, maybe_d2) = match (d, cfg) {
            (Down, DiagDown) | (Up, DiagUp) | (Right, Hor | Empty) =>
                (Right, None),
            (Down, DiagUp) | (Up, DiagDown) | (Left, Hor | Empty) =>
                (Left, None),
            (Right, DiagUp) | (Left, DiagDown) | (Up, Vert | Empty) =>
                (Up, None),
            (Left, DiagUp) | (Right, DiagDown) | (Down, Vert | Empty) =>
                (Down, None),
            (Up | Down, Hor) =>
                (Left, Some(Right)),
            (Left | Right, Vert) =>
                (Up, Some(Down)),
        };
        maybe_propagate(d1, p, config, state);
        if let Some(d2) = maybe_d2 { maybe_propagate(d2, p, config, state) };
    }
}

// Tile is active if light is flowing out into any direction.
fn is_active(s: &&TileState) -> bool {
    [Up, Down, Left, Right].iter().any(|&d| s[d])
}

// Count active tiles after propagating light from the given position,
// starting from a fresh state.
fn num_active(d: Direction, p: Pos, config: &Config) -> usize {
    let mut state: State = config.new_state();
    propagate(d, p, config, &mut state);
    state.iter().map(|row| row.iter().filter(is_active).count()).sum()
}

// Maximizes active tiles, starting from a side, horizontally.
fn max_hor_active(config: &Config) -> usize {
    // From left edge at every row:
    let m = (0..config.height)
        .map(|i| num_active(Right, (i as i64, 0), &config))
        .fold(0, max);
    // From right edge at every row:
    (0..config.height)
        .map(|i| num_active(Left, (i as i64, config.width as i64 - 1), &config))
        .fold(m, max)
}

// Maximizes active tiles, starting from top or bottom, vertically.
fn max_vert_active(config: &Config) -> usize {
    // From top at every column:
    let m = (0..config.width)
        .map(|j| num_active(Down, (0, j as i64), &config))
        .fold(0, max);
    // From bottom at every column:
    (0..config.width)
        .map(|j| num_active(Up, (config.height as i64 - 1, j as i64), &config))
        .fold(m, max)
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

    let config: Config = Config::new(contents.lines().map(read_line).collect());

    let max_active = max_hor_active(&config).max(max_vert_active(&config));
    
    let part1 = num_active(Right, (0, 0), &config);
    println!("{part1} {max_active}");
}
