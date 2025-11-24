use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day18.txt");

const BOARD_SIZE: usize = 100;

struct Board {
    status: [bool; BOARD_SIZE * BOARD_SIZE],
    four_corner_always_on: bool,
}

impl Board {
    fn set_4c_a_on(&mut self) {
        self.four_corner_always_on = true
    }
    fn turn_on(&mut self, ridx: usize, cidx: usize) {
        self.status[ridx * BOARD_SIZE + cidx] = true;
    }
    fn turn_off(&mut self, ridx: usize, cidx: usize) {
        if self.four_corner_always_on {
            self.status[ridx * BOARD_SIZE + cidx] =
                (ridx == 0 || ridx == BOARD_SIZE - 1) && (cidx == 0 || cidx == BOARD_SIZE - 1);
        } else {
            self.status[ridx * BOARD_SIZE + cidx] = false;
        }
    }
    fn get(&self, ridx: usize, cidx: usize) -> bool {
        self.status[ridx * BOARD_SIZE + cidx]
    }

    fn get_alive_nightbor_cnt(&self, ridx: usize, cidx: usize) -> usize {
        let mut ret = 0;
        for i in 0..=2 {
            for j in 0..=2 {
                let row_index = ridx as isize - 1 + i;
                let col_index = cidx as isize - 1 + j;
                if !(0..BOARD_SIZE as isize).contains(&row_index)
                    || !(0..BOARD_SIZE as isize).contains(&col_index)
                    || (i == 1 && j == 1)
                {
                    continue;
                }
                ret += if self.get(row_index as usize, col_index as usize) {
                    1
                } else {
                    0
                }
            }
        }
        ret
    }

    fn alive_cnt(&self) -> usize {
        self.status.iter().filter(|x| **x).count()
    }
}

fn init_state() -> Board {
    let mut board = Board {
        status: [false; BOARD_SIZE * BOARD_SIZE],
        four_corner_always_on: false,
    };

    for (ridx, line) in INPUT.lines().enumerate() {
        for (cidx, bc) in line.bytes().enumerate() {
            if bc == b'#' {
                board.turn_on(ridx, cidx);
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
                    board1.turn_on(row, col);
                } else {
                    board1.turn_off(row, col);
                }
            } else if alive_nighbor_cnt == 3 {
                board1.turn_on(row, col);
            } else {
                board1.turn_off(row, col);
            }
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut board0 = init_state();
    let mut board1 = Board {
        status: [false; BOARD_SIZE * BOARD_SIZE],
        four_corner_always_on: false,
    };

    let iter_times = 100;

    for i in 0..iter_times {
        if i % 2 == 0 {
            evolve(&board0, &mut board1);
        } else {
            evolve(&board1, &mut board0);
        }
    }
    let ret = board0.alive_cnt();
    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut board0 = init_state();
    let mut board1 = Board {
        status: [false; BOARD_SIZE * BOARD_SIZE],
        four_corner_always_on: false,
    };

    board0.set_4c_a_on();
    board1.set_4c_a_on();

    let iter_times = 100;

    for i in 0..iter_times {
        if i % 2 == 0 {
            evolve(&board0, &mut board1);
        } else {
            evolve(&board1, &mut board0);
        }
    }
    let ret = board0.alive_cnt();
    format_result!(ret);
}
