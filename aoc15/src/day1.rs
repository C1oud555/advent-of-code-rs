use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day1.txt");

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let final_floor = INPUT.bytes().fold(0, |acc, ch| match ch {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => acc,
    });

    format_result!(final_floor)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let basement_pos = INPUT
        .bytes()
        .scan(0, |floor, b| {
            *floor += match b {
                b'(' => 1,
                b')' => -1,
                _ => 0,
            };
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .map(|p| p + 1)
        .unwrap_or(0);

    format_result!(basement_pos)
}
