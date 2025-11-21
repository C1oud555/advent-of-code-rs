use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = "3113322113";

fn encode(input: String) -> String {
    let mut input = input.as_bytes().iter().peekable();
    let mut cnt = 1;
    let mut ret = String::new();

    while let Some(ch) = input.next() {
        if let Some(&n_ch) = input.peek() {
            if ch == n_ch {
                cnt += 1;
            } else {
                ret.push_str(&format!("{}{}", cnt, *ch as char));
                cnt = 1;
            }
        } else {
            ret.push_str(&format!("{}{}", cnt, *ch as char));
        }
    }

    ret
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut input = INPUT.to_string();

    for _ in 0..40 {
        input = encode(input);
    }

    format_result!(input.len());
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut input = INPUT.to_string();

    for _ in 0..50 {
        input = encode(input);
    }

    format_result!(input.len());
}
