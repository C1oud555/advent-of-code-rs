use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day3.txt");

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret: usize = INPUT
        .lines()
        .map(|x| {
            let half = x.len() / 2;
            let slice = x.as_bytes();
            for h in 0..half {
                for l in half..x.len() {
                    if slice[h] == slice[l] {
                        return slice[h];
                    }
                }
            }
            0
        })
        .map(|x| {
            if x > 97 {
                (x - b'a' + 1) as usize
            } else {
                (x - b'A' + 1 + 26) as usize
            }
        })
        .sum();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let tmp: Vec<&str> = INPUT.lines().collect();
    let ret: usize = tmp
        .chunks(3)
        .map(|x| {
            let slice0 = x[0].as_bytes();
            let slice1 = x[1].as_bytes();
            let slice2 = x[2].as_bytes();
            for b0 in slice0 {
                for b1 in slice1 {
                    for b2 in slice2 {
                        if b0 == b1 && b0 == b2 {
                            return *b0;
                        }
                    }
                }
            }
            0
        })
        .map(|x| {
            if x > 97 {
                (x - b'a' + 1) as usize
            } else {
                (x - b'A' + 1 + 26) as usize
            }
        })
        .sum();
    format_result!(ret)
}
