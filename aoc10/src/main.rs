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
    board.iter().enumerate().find_map(|(r, row)| start_col(row).map(|c| (r, c)))
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

fn board_dimensions(board: &Board) -> (usize, usize) {
    (board.len(), board[0].len())
}

fn loop_length_and_board(coming_from: Direction, start_pos: (usize, usize), board: &Board) -> Option<(usize, Board)> {
    let (rows, cols) = board_dimensions(board);
    let mut prev = coming_from;
    let mut steps = 0;
    let (mut r, mut c) = start_pos;
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
                return Some(((steps + 1) / 2, loop_board))
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
    let (rows, cols) = board_dimensions(board);
    let (srow, scol) = start_row_col(board)?;
    let mut candidates = Vec::new();
    if scol < cols - 1 {
        candidates.push((West, (srow, scol+1)))
    }
    if srow < rows - 1 {
        candidates.push((North, (srow+1, scol)))
    }
    if scol > 0 {
        candidates.push((East, (srow, scol-1)))
    }
    candidates.iter().find_map(|&(d, p)| loop_length_and_board(d, p, board))
}

fn count_inside(loop_board: &Board) -> usize {
    loop_board.iter()
        .map(|row| row.iter()
             .fold((false, false, 0), |state @ (inside, was_north, n), tile|
                   match tile {
                       Ground => (inside, was_north, if inside { n+1 } else { n }),
                       EW => state,
                       NS => (!inside, was_north, n),
                       NE => (inside, true, n),
                       SE => (inside, false, n),
                       NW => (was_north == inside, was_north, n),
                       SW => (was_north != inside, was_north, n),
                       Start => panic!("unexpected Start tile on loop boad"),
                   })
             .2)
        .sum()
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
