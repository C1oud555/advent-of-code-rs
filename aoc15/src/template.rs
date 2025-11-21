use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day1.txt");

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let _ = INPUT;
    format_result!("template 0");
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!("template 1");
}
