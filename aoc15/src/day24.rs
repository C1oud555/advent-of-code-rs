use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day24.txt");

fn parse_input() -> Vec<usize> {
    INPUT
        .lines()
        .map(|line| line.parse::<usize>().expect("Not valid number"))
        .collect()
}

fn solve0(
    start: usize,
    choices: &[usize],
    target: usize,
    selected: &[usize],
    solutions: &mut Vec<Vec<usize>>,
) {
    for i in start..choices.len() {
        let choice = choices[i];
        let min_len = solutions.iter().map(|x| x.len()).min().unwrap_or(100);
        if selected.len() > min_len {
            return;
        }

        if choice > target {
            continue;
        }
        let mut tmp = selected.to_owned();
        tmp.push(choice);
        if choice == target {
            solutions.push(tmp);
        } else {
            solve0(i + 1, choices, target - choice, &tmp, solutions);
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut input = parse_input();
    input.sort();
    input.reverse();
    let i_sum: usize = input.iter().sum();
    let target = i_sum / 3;

    let mut solutions = vec![];
    solve0(0, &input, target, &[], &mut solutions);
    let min_len = solutions.iter().map(|x| x.len()).min().unwrap();
    solutions.retain(|x| x.len() == min_len);

    let ret = solutions
        .iter()
        .map(|x| x.iter().product::<usize>())
        .min()
        .unwrap();

    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut input = parse_input();
    input.sort();
    input.reverse();
    let i_sum: usize = input.iter().sum();
    let target = i_sum / 4;

    let mut solutions = vec![];
    solve0(0, &input, target, &[], &mut solutions);
    let min_len = solutions.iter().map(|x| x.len()).min().unwrap();
    solutions.retain(|x| x.len() == min_len);

    let ret = solutions
        .iter()
        .map(|x| x.iter().product::<usize>())
        .min()
        .unwrap();

    format_result!(ret)
}
