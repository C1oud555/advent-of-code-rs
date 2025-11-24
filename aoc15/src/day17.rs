use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day17.txt");

fn parse_containers() -> Vec<isize> {
    INPUT
        .lines()
        .map(|line| line.parse().expect("not valid number"))
        .collect()
}

fn backtrack(
    state: &mut Vec<isize>,
    target: isize,
    choice: &[isize],
    start: usize,
    res: &mut Vec<Vec<isize>>,
) {
    if target == 0 {
        res.push(state.clone());
        return;
    }

    for i in start..choice.len() {
        if target - choice[i] < 0 {
            break;
        }

        state.push(choice[i]);
        backtrack(state, target - choice[i], choice, i + 1, res);
        state.pop();
    }
}

fn solve(nums: &mut [isize], target: isize) -> Vec<Vec<isize>> {
    let mut state = Vec::new();
    nums.sort();
    let start = 0;
    let mut res = Vec::new();
    backtrack(&mut state, target, nums, start, &mut res);
    res
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut containers = parse_containers();

    let target = 150;

    let ret = solve(&mut containers, target).len();

    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut containers = parse_containers();

    let target = 150;

    let solutions = solve(&mut containers, target);
    let min_containers = solutions.iter().map(|x| x.len()).min().unwrap();

    let ret = solutions
        .iter()
        .filter(|x| x.len() == min_containers)
        .count();

    format_result!(ret);
}
