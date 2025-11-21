use std::collections::HashSet;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day3.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

fn record_present_num0() -> HashSet<Coord> {
    let mut visited_houses = HashSet::new();
    let mut current_pos = Coord { x: 0, y: 0 };

    visited_houses.insert(current_pos);

    for ch in INPUT.chars() {
        match ch {
            '^' => current_pos.y += 1,
            'v' => current_pos.y -= 1,
            '>' => current_pos.x += 1,
            '<' => current_pos.x -= 1,
            _ => {}
        }
        visited_houses.insert(current_pos);
    }

    visited_houses
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let num = record_present_num0().len();
    format_result!(num);
}

fn record_present_num1() -> HashSet<Coord> {
    let mut visited_houses = HashSet::new();
    let mut santa_pos = Coord { x: 0, y: 0 };
    let mut robot_pos = Coord { x: 0, y: 0 };

    visited_houses.insert(santa_pos);

    for (i, ch) in INPUT.chars().enumerate() {
        let current_pos = if i % 2 == 0 {
            // Santa's turn
            &mut santa_pos
        } else {
            // Robo-Santa's turn
            &mut robot_pos
        };

        match ch {
            '^' => current_pos.y += 1,
            'v' => current_pos.y -= 1,
            '>' => current_pos.x += 1,
            '<' => current_pos.x -= 1,
            _ => {}
        }
        visited_houses.insert(*current_pos);
    }

    visited_houses
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let num = record_present_num1().len();
    format_result!(num);
}
