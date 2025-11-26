use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: (usize, usize) = (2978, 3083);

const INIT: usize = 20151125;

fn apply_rule(old: usize) -> usize {
    (old * 252533) % 33554393
}

fn diagonal_gen(target: (usize, usize)) -> usize {
    let mut code = INIT;
    let mut coord: (usize, usize) = (1, 1);
    while coord != target {
        code = apply_rule(code);
        let (row, col) = coord;
        if row == 1 {
            coord = (col + 1, row);
        } else {
            coord = (row - 1, col + 1);
        }
    }

    code
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = diagonal_gen(INPUT);
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!("50 stars ~")
}
