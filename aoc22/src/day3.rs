use crate::{PUZZLES, format_result};
use linkme::distributed_slice;
use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day3.txt");

/// Calculates the priority for a given item type (represented as a byte).
/// 'a' through 'z' have priorities 1 through 26.
/// 'A' through 'Z' have priorities 27 through 52.
fn priority(item: u8) -> usize {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as usize,
        b'A'..=b'Z' => (item - b'A' + 27) as usize,
        _ => 0, // Should not happen with valid input
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let total_priority: usize = INPUT
        .lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let comp1_items: HashSet<u8> = comp1.bytes().collect();

            let common_item = comp2
                .bytes()
                .find(|item| comp1_items.contains(item))
                .expect("Should be a common item in each rucksack");

            priority(common_item)
        })
        .sum();

    format_result!(total_priority)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let lines: Vec<&str> = INPUT.lines().collect();
    let total_priority: usize = lines
        .chunks(3)
        .map(|group| {
            let badge = group[0]
                .bytes()
                .find(|&item| {
                    group[1].bytes().any(|i| i == item) && group[2].bytes().any(|i| i == item)
                })
                .expect("Should be a common badge in each group");

            priority(badge)
        })
        .sum();

    format_result!(total_priority)
}
