use crate::{PUZZLES, format_result};
use linkme::distributed_slice;
use once_cell::sync::Lazy;

const INPUT: &str = include_str!("../inputs/day1.txt");

static RESULTS: Lazy<(usize, usize)> = Lazy::new(|| {
    let (top_three, max_calories) = INPUT
        .split("\n\n")
        .map(|elf_block| {
            elf_block
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<usize>().expect("Not a valid calory number"))
                .sum::<usize>()
        })
        .fold(
            ([0, 0, 0], 0),
            |(mut top_three, max_calories), current_calories| {
                // Update max_calories for Part 1
                let new_max = max_calories.max(current_calories);

                // Update top_three for Part 2
                if current_calories > top_three[0] {
                    top_three[0] = current_calories;
                    top_three.sort_unstable();
                }
                (top_three, new_max)
            },
        );

    let part2_result = top_three.iter().sum();
    (max_calories, part2_result)
});

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(RESULTS.0)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(RESULTS.1)
}
