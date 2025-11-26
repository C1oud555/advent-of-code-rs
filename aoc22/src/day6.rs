use std::collections::HashSet;

use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day6.txt");

fn n_distinct_characters(n: usize) -> usize {
    INPUT
        .as_bytes()
        .windows(n)
        .position(|x| {
            let set: HashSet<&u8> = x.iter().collect();
            set.len() == n
        })
        .expect("Not found 4 unique sequence")
        + n
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = n_distinct_characters(4);
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = n_distinct_characters(14);
    format_result!(ret)
}
