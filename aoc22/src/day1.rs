use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day1.txt");

fn parse_input() -> Vec<Vec<usize>> {
    let mut ret = vec![];
    let mut sub_ret = vec![];
    for line in INPUT.lines() {
        if line.is_empty() {
            ret.push(sub_ret.clone());
            sub_ret.clear();
            continue;
        }
        sub_ret.push(line.parse::<usize>().expect("Not valid calory number"));
    }

    ret
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let inputs = parse_input();
    let ret: usize = inputs.iter().map(|x| x.iter().sum()).max().unwrap();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let inputs = parse_input();
    let mut sums: Vec<usize> = inputs.iter().map(|x| x.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    let ret: usize = sums.iter().take(3).sum();
    format_result!(ret)
}
