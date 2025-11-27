use std::collections::HashSet;

use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day9.txt");

fn solve(input: &str, rope_length: usize) -> String {
    let mut knots = vec![(0, 0); rope_length];
    let mut visited = HashSet::from([(0, 0)]);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount: i32 = amount.parse().unwrap();

        for _ in 0..amount {
            // Move the head knot
            match dir {
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                _ => unreachable!(),
            }

            // Move the subsequent knots
            for i in 1..rope_length {
                let head = knots[i - 1];
                let tail = &mut knots[i];

                let dx: i32 = head.0 - tail.0;
                let dy: i32 = head.1 - tail.1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    tail.0 += dx.signum();
                    tail.1 += dy.signum();
                }
            }
            // Record the position of the last knot
            visited.insert(*knots.last().unwrap());
        }
    }
    format_result!(visited.len())
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    solve(INPUT, 2)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    solve(INPUT, 10)
}
