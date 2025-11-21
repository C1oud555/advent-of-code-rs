use std::collections::HashMap;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day5.txt");

fn is_nice_part1(s: &str) -> bool {
    let bytes = s.as_bytes();
    // It contains at least three vowels (aeiou only).
    let has_three_vowels = s
        .chars()
        .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3;

    // It contains at least one letter that appears twice in a row.
    let has_double_letter = bytes.windows(2).any(|w| w[0] == w[1]);

    // It does not contain the strings ab, cd, pq, or xy.
    let has_naughty_strings = bytes
        .windows(2)
        .any(|w| matches!(w, b"ab" | b"cd" | b"pq" | b"xy"));

    has_three_vowels && has_double_letter && !has_naughty_strings
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let count = INPUT.lines().filter(|line| is_nice_part1(line)).count();
    format_result!(count);
}

fn has_non_overlapping_pair(s: &[u8]) -> bool {
    let mut seen_pairs = HashMap::<[u8; 2], usize>::with_capacity(s.len());
    for i in 0..s.len() - 1 {
        let pair: [u8; 2] = [s[i], s[i + 1]];
        if let Some(first_index) = seen_pairs.get(&pair) {
            if i > *first_index + 1 {
                return true;
            }
        } else {
            seen_pairs.insert(pair, i);
        }
    }
    false
}

fn is_nice_part2(s: &str) -> bool {
    let bytes = s.as_bytes();
    // It contains a pair of any two letters that appears at least twice in the string without overlapping.
    let has_pair = has_non_overlapping_pair(bytes);

    // It contains at least one letter which repeats with exactly one letter between them.
    let has_sandwich = bytes.windows(3).any(|w| w[0] == w[2]);

    has_pair && has_sandwich
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let count = INPUT.lines().filter(|line| is_nice_part2(line)).count();
    format_result!(count);
}
