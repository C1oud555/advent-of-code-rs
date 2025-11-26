use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day2.txt");

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn fight0(&self, other: &Play) -> usize {
        match (&self, other) {
            (Play::Rock, Play::Rock) => 3 + 1,
            (Play::Rock, Play::Paper) => 1,
            (Play::Rock, Play::Scissors) => 6 + 1,
            (Play::Paper, Play::Rock) => 6 + 2,
            (Play::Paper, Play::Paper) => 3 + 2,
            (Play::Paper, Play::Scissors) => 2,
            (Play::Scissors, Play::Rock) => 3,
            (Play::Scissors, Play::Paper) => 6 + 3,
            (Play::Scissors, Play::Scissors) => 3 + 3,
        }
    }
    fn fight1(&self, other: &Play) -> usize {
        match (&self, other) {
            // lose
            (Play::Rock, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 1,
            (Play::Rock, Play::Scissors) => 2,
            // draw
            (Play::Paper, Play::Rock) => 3 + 1,
            (Play::Paper, Play::Paper) => 3 + 2,
            (Play::Paper, Play::Scissors) => 3 + 3,
            // win
            (Play::Scissors, Play::Rock) => 6 + 2,
            (Play::Scissors, Play::Paper) => 6 + 3,
            (Play::Scissors, Play::Scissors) => 6 + 1,
        }
    }
}

fn parse_input() -> Vec<(Play, Play)> {
    INPUT
        .lines()
        .map(|line| {
            let elf = if line.starts_with("A") {
                Play::Rock
            } else if line.starts_with("B") {
                Play::Paper
            } else if line.starts_with("C") {
                Play::Scissors
            } else {
                panic!("aa");
            };
            let me = if line.ends_with("X") {
                Play::Rock
            } else if line.ends_with("Y") {
                Play::Paper
            } else if line.ends_with("Z") {
                Play::Scissors
            } else {
                panic!("aa");
            };
            (elf, me)
        })
        .collect()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret: usize = parse_input().iter().map(|(elf, me)| me.fight0(elf)).sum();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret: usize = parse_input().iter().map(|(elf, me)| me.fight1(elf)).sum();
    format_result!(ret)
}
