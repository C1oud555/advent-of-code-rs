use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day2.txt");

fn parse_input() -> impl Iterator<Item = (u32, u32, u32)> {
    INPUT.lines().map(|line| {
        let nums: Vec<u32> = line
            .split("x")
            .map(|num_str| num_str.parse::<u32>().expect("not valid number: {x}"))
            .collect();
        (nums[0], nums[1], nums[2])
    })
}

pub fn calc_area(shape: &(u32, u32, u32)) -> u32 {
    let (l, h, w) = shape;
    let areas = [l * h, l * w, h * w];
    let min = areas.iter().min().expect("no min");
    areas.iter().sum::<u32>() * 2 + min
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let all_area = parse_input().map(|shape| calc_area(&shape)).sum::<u32>();
    format_result!(all_area);
}

pub fn calc_ribbon(shape: &(u32, u32, u32)) -> u32 {
    let (l, h, w) = shape;
    let bow = l * h * w;
    let max = l.max(h).max(w);
    let wrap = 2 * (l + w + h - max);
    bow + wrap
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let all_ribbon = parse_input().map(|shape| calc_ribbon(&shape)).sum::<u32>();
    format_result!(all_ribbon);
}
