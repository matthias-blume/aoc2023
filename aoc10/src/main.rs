use std::env;
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Ground,
    Start,
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

fn read_tile(c: char) -> Tile {
    match c {
        '.' => Ground,
        'S' => Start,
        '-' => EW,
        '|' => NS,
        'L' => NE,
        'J' => NW,
        '7' => SW,
        'F' => SE,
        _ => panic!("bad tile '{}'", c)
    }
}

type Row = Vec<Tile>;
type Board = Vec<Vec<Tile>>;

fn read_row(l: &str) -> Row {
    l.chars().map(read_tile).collect()
}

fn read_board(c: &str) -> Board {
    c.lines().map(read_row).collect()
}

fn start_col(line: &Row) -> Option<usize> {
    line.iter().position(|&x| x == Start)
}

fn start_row_col(board: &Board) -> Option<(usize, usize)> {
    let mut row = 0;
    for line in board.iter() {
        if let Some(col) = start_col(line) {
            return Some((row, col))
        }
        row += 1
    }
    None
}

fn blank_board(board: &Board) -> Board {
    board.iter().map(|r| r.iter().map(|_| Ground).collect()).collect()
}

fn opposite(d: Direction) -> Direction {
    match d {
        East => West,
        West => East,
        North => South,
        South => North,
    }
}

fn loop_length_and_board(coming_from: Direction, r0: usize, c0: usize, board: &Board) -> Option<(usize, Board)> {
    let rows = board.len();
    let cols = board[0].len();
    let mut prev = coming_from;
    let mut steps = 0;
    let mut r = r0;
    let mut c = c0;
    let mut loop_board = blank_board(&board);
    loop {
        if r >= rows || c >= cols {
            return None
        }
        let t = board[r][c];
        loop_board[r][c] = t;
        match (prev, t) {
            (_, Start) => {
                let start_tile = match (prev, opposite(coming_from)) {
                    (East, West) | (West, East) => EW,
                    (North, South) | (South, North) => NS,
                    (East, North) | (North, East) => NE,
                    (East, South) | (South, East) => SE,
                    (West, North) | (North, West) => NW,
                    (West, South) | (South, West) => SW,
                    _ => panic!("impossible start tile"),
                };
                loop_board[r][c] = start_tile;
                return Some((steps, loop_board))
            },
            (West, EW) => c += 1,
            (East, EW) if c > 0 => c -= 1,
            (North, NS) => r += 1,
            (South, NS) if r > 0 => r -= 1,
            (North, NE) => { c += 1; prev = West },
            (East, NE) if r > 0 => { r -= 1; prev = South },
            (South, SE) => { c += 1; prev = West },
            (East, SE) => { r += 1; prev = North },
            (North, NW) if c > 0 => { c -= 1; prev = East },
            (West, NW) if r > 0 => { r -= 1; prev = South },
            (South, SW) if c > 0 => { c -= 1; prev = East },
            (West, SW) => { r += 1; prev = North },
            _ => return None,
        }
        steps += 1;
    }
}

fn loop_diameter_and_board(board: &Board) -> Option<(usize, Board)> {
    let (srow, scol) = start_row_col(board)?;
    if let Some((steps, lb)) = loop_length_and_board(West, srow, scol+1, board) {
        return Some(((steps + 1) / 2, lb))
    }
    if let Some((steps, lb)) = loop_length_and_board(East, srow, scol-1, board) {
        return Some(((steps + 1) / 2, lb))
    }
    if let Some((steps, lb)) = loop_length_and_board(North, srow+1, scol, board) {
        return Some(((steps + 1) / 2, lb))
    }
    None
}

fn count_inside(lb: &Board) -> usize {
    let mut n = 0;
    for row in lb.iter() {
        let mut inside = false;
        let mut was_north = false;
        for tile in row.iter() {
            match tile {
                Ground => { if inside { n += 1 } },
                EW => (),
                NS => inside = !inside,
                NE => was_north = true,
                SE => was_north = false,
                NW => { if !was_north { inside = !inside } },
                SW => { if was_north { inside = !inside } },
                _ => panic!("unexpected tile"),
            }
        }
    }
    n           
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
    if let Some((diameter, loop_board)) = loop_diameter_and_board(&board) {
        let inside = count_inside(&loop_board);
        println!("{diameter} {inside}")
    } else {
        panic!("no solution")
    }
}
