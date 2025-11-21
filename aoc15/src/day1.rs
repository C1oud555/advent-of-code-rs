use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day1.txt");

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let final_floor = INPUT.chars().fold(0, |acc, ch| match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    format_result!(final_floor);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut res = 0;
    let mut floor = 0;
    for (index, ch) in INPUT.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => {
                if floor == 0 {
                    res = index + 1;
                    break;
                }
                floor -= 1;
            }
            _ => {}
        };
    }

    format_result!(res);
}
