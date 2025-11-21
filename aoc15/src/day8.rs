use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day8.txt");

fn parse_line(line: &str) -> usize {
    let mut count = 0;
    let mut iter = line.as_bytes().iter();
    while let Some(ch) = iter.next() {
        match ch {
            b'\\' => match iter.next().unwrap() {
                b'\\' => count += 1,
                b'"' => count += 1,
                b'x' => {
                    iter.next();
                    iter.next();
                    count += 1;
                }
                _ => panic!("invalid escape"),
            },
            b'"' => {}
            _ => count += 1,
        }
    }

    count
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let total_chars: usize = INPUT.lines().map(|line| line.len()).sum();
    let parsed_chars: usize = INPUT.lines().map(parse_line).sum();
    format_result!(total_chars - parsed_chars);
}

fn encode_line(line: &str) -> String {
    let mut res = String::new();
    res.push('"');

    for ch in line.as_bytes().iter() {
        match ch {
            b'\\' => res.push_str("\\\\"),
            b'"' => res.push_str("\\\""),
            _ => res.push(*ch as char),
        }
    }
    res.push('"');

    res
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let total_chars: usize = INPUT.lines().map(|line| line.len()).sum();
    let encoded_chars: usize = INPUT.lines().map(|line| encode_line(line).len()).sum();
    format_result!(encoded_chars - total_chars);
}
