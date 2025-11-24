use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use std::fmt::Write;

const INPUT: &str = "3113322113";

fn encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut result = String::with_capacity(s.len() * 2);
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next_c) = chars.peek() {
            if next_c == c {
                count += 1;
                chars.next(); // consume
            } else {
                break;
            }
        }
        // The write! macro on a String is infallible.
        write!(&mut result, "{}{}", count, c).unwrap();
    }
    result
}

fn solve(iterations: usize) -> usize {
    let mut input = INPUT.to_string();
    for _ in 0..iterations {
        input = encode(&input);
    }
    input.len()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(solve(40))
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(solve(50))
}
