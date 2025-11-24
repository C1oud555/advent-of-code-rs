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

fn solve_optimized(nums: &mut [isize], target: isize) -> (usize, usize) {
    nums.sort(); // Sort in ascending order
    let mut count = 0;
    let mut min_len = usize::MAX;
    let mut min_len_count = 0;

    fn backtrack(
        target: isize,
        choices: &[isize],
        start: usize,
        depth: usize,
        count: &mut usize,
        min_len: &mut usize,
        min_len_count: &mut usize,
    ) {
        if target == 0 {
            *count += 1;
            if depth < *min_len {
                *min_len = depth;
                *min_len_count = 1;
            } else if depth == *min_len {
                *min_len_count += 1;
            }
            return;
        }

        for i in start..choices.len() {
            let choice = choices[i];
            if target - choice < 0 {
                // Since the choices are sorted, we can break early.
                break;
            }
            backtrack(
                target - choice,
                choices,
                i + 1,
                depth + 1,
                count,
                min_len,
                min_len_count,
            );
        }
    }

    backtrack(
        target,
        nums,
        0,
        0,
        &mut count,
        &mut min_len,
        &mut min_len_count,
    );
    (count, min_len_count)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut containers = parse_containers();
    let target = 150;
    let (ret, _) = solve_optimized(&mut containers, target);
    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut containers = parse_containers();
    let target = 150;
    let (_, ret) = solve_optimized(&mut containers, target);
    format_result!(ret);
}
