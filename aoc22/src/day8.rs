use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day8.txt");

fn parse_input() -> Vec<Vec<u8>> {
    let board_len = INPUT.lines().count();
    let mut ret = vec![vec![0; board_len]; board_len];
    for (i, line) in INPUT.lines().enumerate() {
        for (j, byte) in line.bytes().enumerate() {
            ret[i][j] = byte;
        }
    }

    ret
}

fn check_visible(board: &[Vec<u8>], row: usize, col: usize) -> bool {
    let board_len = board.len();
    let value = board[row][col];
    let mut ret = [true; 4];
    (0..row).for_each(|ridx| {
        ret[0] = (board[ridx][col] < value) && ret[0];
    });
    (row + 1..board_len).for_each(|ridx| {
        ret[1] = (board[ridx][col] < value) && ret[1];
    });
    (0..col).for_each(|cidx| {
        ret[2] = (board[row][cidx] < value) && ret[2];
    });
    (col + 1..board_len).for_each(|cidx| {
        ret[3] = (board[row][cidx] < value) && ret[3];
    });
    ret.iter().any(|x| *x)
}

fn scenic_score(board: &[Vec<u8>], row: usize, col: usize) -> u32 {
    let board_len = board.len();

    if row == 0 || row == board_len - 1 || col == 0 || col == board_len - 1 {
        return 0;
    }

    let value = board[row][col];
    let mut ret = [0; 4];
    let mut ridx = row - 1;
    while ridx < board_len {
        if board[ridx][col] < value {
            ret[0] += 1;
            ridx -= 1;
        } else if board[ridx][col] == value {
            ret[0] += 1;
            break;
        } else {
            break;
        }
    }
    let mut ridx = row + 1;
    while ridx < board_len {
        if board[ridx][col] < value {
            ret[1] += 1;
            ridx += 1;
        } else {
            ret[1] += 1;
            break;
        }
    }

    let mut cidx = col - 1;
    while cidx < board_len {
        if board[row][cidx] < value {
            ret[2] += 1;
            cidx -= 1;
        } else {
            ret[2] += 1;
            break;
        }
    }
    let mut cidx = col + 1;
    while cidx < board_len {
        if board[row][cidx] < value {
            ret[3] += 1;
            cidx += 1;
        } else {
            ret[3] += 1;
            break;
        }
    }
    println!("iter: {:?}", ret);
    ret.iter().product()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let board = parse_input();
    let mut ret = 0;
    for (ridx, row) in board.iter().enumerate() {
        for (cidx, _col) in row.iter().enumerate() {
            if check_visible(&board, ridx, cidx) {
                ret += 1;
            }
        }
    }
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let board = parse_input();
    let mut ret = 0;
    for (ridx, row) in board.iter().enumerate() {
        for (cidx, _col) in row.iter().enumerate() {
            let score = scenic_score(&board, ridx, cidx);
            ret = ret.max(score);
        }
    }
    format_result!(ret)
}
