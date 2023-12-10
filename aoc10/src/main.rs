use std::env;
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Ground,
    EW,
    NS,
    NE,
    SE,
    NW,
    SW,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

use crate::Tile::*;
use crate::Direction::*;

fn read_tile(c: char) -> Option<Tile> {
    match c {
        'S' => None,
        '.' => Some(Ground),
        '-' => Some(EW),
        '|' => Some(NS),
        'L' => Some(NE),
        'J' => Some(NW),
        '7' => Some(SW),
        'F' => Some(SE),
        _ => panic!("bad tile '{}'", c)
    }
}

// On the StartBoard one of the tiles is None, indicating the
// start tile ('S').  On the LoopBoard the only non-Ground tiles
// are those that are in the loop, and the start tile has been
// filled in with a proper Tile.
type Row<T> = Vec<T>;
type StartRow = Row<Option<Tile>>;
type LoopRow = Row<Tile>;
type Board<T> = Vec<Row<T>>;
type StartBoard = Board<Option<Tile>>;
type LoopBoard = Board<Tile>;
type Pos = (usize, usize);

fn read_row(l: &str) -> StartRow {
    l.chars().map(read_tile).collect()
}

fn read_board(c: &str) -> StartBoard {
    c.lines().map(read_row).collect()
}

// Find the column of the start tile on a StartRow.
fn start_col(line: &StartRow) -> Option<usize> {
    line.iter().position(|&x| x == None)
}

// Find the location of the start tile on a StartBoard.
fn start_pos(board: &StartBoard) -> Option<Pos> {
    board.iter().enumerate().find_map(|(r, row)| start_col(row).map(|c| (r, c)))
}

// Make a blank LoopBoard with the same dimensions as the given StartBoard.
fn blank_board(board: &StartBoard) -> LoopBoard {
    board.iter().map(|r| r.iter().map(|_| Ground).collect()).collect()
}

// Reverse the direction.
fn opposite(d: Direction) -> Direction {
    match d {
        East => West,
        West => East,
        North => South,
        South => North,
    }
}

// Given a tile and one of its directions, get the other direction.
fn out_direction(in_direction: Direction, t: Tile) -> Option<Direction> {
    match (in_direction, t) {
        (East, NE) | (West, NW) | (South, NS) => Some(North),
        (East, SE) | (West, SW) | (North, NS) => Some(South),
        (West, EW) | (North, NE) | (South, SE) => Some(East),
        (East, EW) | (North, NW) | (South, SW) => Some(West),
        _ => None,
    }
}

// Given two directions, return the tile that connects them.
fn connecting_pipe(d1: Direction, d2: Direction) -> Option<Tile> {
    match (d1, d2) {
        (East, West) | (West, East) => Some(EW),
        (North, South) | (South, North) => Some(NS),
        (North, East) | (East, North) => Some(NE),
        (North, West) | (West, North) => Some(NW),
        (South, East) | (East, South) => Some(SE),
        (South, West) | (West, South) => Some(SW),
        _ => None
    }
}

// Given a position and a direction, step one step in that direction if
// possible according to the given board dimensions.
fn single_step_pos((r, c): Pos, dim: Pos, direction: Direction) -> Option<Pos> {
    match direction {
        North if r > 0 => Some((r-1, c)),
        South if r < dim.0 => Some((r+1, c)),
        West if c > 0 => Some((r, c-1)),
        East if c < dim.1 => Some((r, c+1)),
        _ => None,
    }
}

type LoopDistanceAndBoard = (usize, LoopBoard);

// Given the position of the start tile and a starting direction, follow the loop
// if possible back to the start location, calculate the distance to the most
// distant point along the loop, and produce a LoopBoard containing the loop
// (with the start tile filled in).
fn try_start_direction(start_pos: Pos, start_dir: Direction, board: &StartBoard)
                       -> Option<LoopDistanceAndBoard> {
    let dim = (board.len(), board[0].len());
    let mut steps = 0;
    let mut pos = single_step_pos(start_pos, dim, start_dir)?;
    let mut prev = opposite(start_dir);
    let mut loop_board = blank_board(&board);
    loop {
        steps += 1;
        match board[pos.0][pos.1] {
            Some(t) => {
                loop_board[pos.0][pos.1] = t;
                let dir = out_direction(prev, t)?;
                pos = single_step_pos(pos, dim, dir)?;
                prev = opposite(dir);
            },
            None => {
                loop_board[pos.0][pos.1] = connecting_pipe(start_dir, prev)?;
                return Some((steps / 2, loop_board));
            },
        }
    }
}

// Find the loop on the given board and return distance to most distant point
// as well as the filled-in LoopBoard for it.
fn loop_info(board: &StartBoard) -> Option<LoopDistanceAndBoard> {
    let start_pos = start_pos(board)?;
    vec![North, East, South]
        .iter()
        .find_map(|&d| try_start_direction(start_pos, d, board))
}

// Calculate area inside loop on an individual LoopRow.
fn loop_row_area(row: &LoopRow) -> usize {
    row.iter()
        .fold((false, false, 0), |state @ (inside, was_north, n), tile|
              // NS always alternates between inside and outside.
              // Neither NE EW... NW nor SE EW... SW switch inside and outside.
              // Both NE EW... SW and SE EW... NW do switch (act like NS).
              // EW does nothing.
              // Ground is counted as enclosed area when currently inside.
              match tile {
                  Ground => (inside, was_north, if inside { n+1 } else { n }),
                  EW => state,
                  NS => (!inside, false, n),
                  NE => (inside, true, n),   // L---
                  SE => (inside, false, n),  // F---
                  NW => (was_north == inside, false, n),  // L---J  vs. F---J
                  SW => (was_north != inside, false, n),  // F---7  vs. L---7
              })
        .2
}

// Count area inside the loop on a LoopBoard.
fn loop_board_area(board: &LoopBoard) -> usize {
    board.iter().map(loop_row_area).sum()
}

fn main() {
    let mut args = env::args();
    let program = match args.next() {
        Some(arg) => arg,
        _ => panic!("no program name"),
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        _ => panic!("{}: no program name", program),
    };

    let contents = fs::read_to_string(file_path)
        .expect("Could not read file");

    let board = read_board(&contents);
    if let Some((distance, loop_board)) = loop_info(&board) {
        let inside = loop_board_area(&loop_board);
        println!("{distance} {inside}")
    } else {
        panic!("no loop found")
    }
}
