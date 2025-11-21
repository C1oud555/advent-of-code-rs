use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day5.txt");

// 1. check vowels
// 2. check two character
// 3. check no `ab` `cd` `pq` `xy` inside
fn check_good(input: &str) -> bool {
    let has_three_vowels = input.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;
    let has_double_letter = input.as_bytes().windows(2).any(|w| w[0] == w[1]);
    let has_naughty_strings = input
        .as_bytes()
        .windows(2)
        .any(|w| matches!(w, b"ab" | b"cd" | b"pq" | b"xy"));

    has_three_vowels && has_double_letter && !has_naughty_strings
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut count = 0;
    for line in INPUT.lines() {
        if check_good(line) {
            count += 1;
        }
    }

    format_result!(count);
}

// 1. check aaxxxxxaa
// 2. check xyx
fn check_good1(input: &str) -> bool {
    let has_sandwich = input.as_bytes().windows(3).any(|w| w[0] == w[2]);

    let has_non_overlapping_pair =
        (0..input.len().saturating_sub(3)).any(|i| input[i + 2..].contains(&input[i..i + 2]));

    has_sandwich && has_non_overlapping_pair
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut count = 0;
    for line in INPUT.lines() {
        if check_good1(line) {
            count += 1;
        }
    }

    format_result!(count);
}
