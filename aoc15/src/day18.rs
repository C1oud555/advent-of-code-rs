use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day18.txt");

const BOARD_SIZE: usize = 100;

struct Board {
    status: [bool; BOARD_SIZE * BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Board {
            status: [false; BOARD_SIZE * BOARD_SIZE],
        }
    }

    fn set(&mut self, ridx: usize, cidx: usize, value: bool) {
        self.status[ridx * BOARD_SIZE + cidx] = value;
    }

    fn get(&self, ridx: usize, cidx: usize) -> bool {
        self.status[ridx * BOARD_SIZE + cidx]
    }

    fn get_alive_nightbor_cnt(&self, ridx: usize, cidx: usize) -> usize {
        let mut count = 0;
        let r = ridx as isize;
        let c = cidx as isize;

        let neighbors = [
            (r - 1, c - 1),
            (r - 1, c),
            (r - 1, c + 1),
            (r, c - 1),
            (r, c + 1),
            (r + 1, c - 1),
            (r + 1, c),
            (r + 1, c + 1),
        ];

        for &(nr, nc) in &neighbors {
            if nr >= 0
                && nr < BOARD_SIZE as isize
                && nc >= 0
                && nc < BOARD_SIZE as isize
                && self.get(nr as usize, nc as usize)
            {
                count += 1;
            }
        }
        count
    }

    fn alive_cnt(&self) -> usize {
        self.status.iter().filter(|x| **x).count()
    }
}

fn init_state() -> Board {
    let mut board = Board::new();

    for (ridx, line) in INPUT.lines().enumerate() {
        for (cidx, bc) in line.bytes().enumerate() {
            if bc == b'#' {
                board.set(ridx, cidx, true);
            }
        }
    }

    board
}

fn evolve(board0: &Board, board1: &mut Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let alive_nighbor_cnt = board0.get_alive_nightbor_cnt(row, col);
            if board0.get(row, col) {
                if alive_nighbor_cnt == 2 || alive_nighbor_cnt == 3 {
                    board1.set(row, col, true);
                } else {
                    board1.set(row, col, false);
                }
            } else if alive_nighbor_cnt == 3 {
                board1.set(row, col, true);
            } else {
                board1.set(row, col, false);
            }
        }
    }
}

fn run_simulation(steps: usize, corners_on: bool) -> usize {
    let mut board0 = init_state();
    let mut board1 = Board::new();

    if corners_on {
        board0.set(0, 0, true);
        board0.set(0, BOARD_SIZE - 1, true);
        board0.set(BOARD_SIZE - 1, 0, true);
        board0.set(BOARD_SIZE - 1, BOARD_SIZE - 1, true);
    }

    let mut current = &mut board0;
    let mut next = &mut board1;

    for _ in 0..steps {
        evolve(current, next);
        if corners_on {
            next.set(0, 0, true);
            next.set(0, BOARD_SIZE - 1, true);
            next.set(BOARD_SIZE - 1, 0, true);
            next.set(BOARD_SIZE - 1, BOARD_SIZE - 1, true);
        }
        std::mem::swap(&mut current, &mut next);
    }

    current.alive_cnt()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = run_simulation(100, false);
    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = run_simulation(100, true);
    format_result!(ret);
}
