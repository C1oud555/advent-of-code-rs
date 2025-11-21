use std::collections::HashSet;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day3.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn move_by(&mut self, instruction: u8) {
        match instruction {
            b'^' => self.y += 1,
            b'v' => self.y -= 1,
            b'>' => self.x += 1,
            b'<' => self.x -= 1,
            _ => {}
        }
    }
}

fn record_present_num0() -> usize {
    std::iter::once(Coord::default())
        .chain(INPUT.bytes().scan(Coord::default(), |pos, instruction| {
            pos.move_by(instruction);
            Some(*pos)
        }))
        .collect::<HashSet<_>>()
        .len()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(record_present_num0());
}

fn record_present_num1() -> usize {
    let mut visited_houses = HashSet::new();
    visited_houses.insert(Coord::default()); // Starting house

    // Santa's moves
    let santa_path = INPUT
        .bytes()
        .step_by(2)
        .scan(Coord::default(), |pos, instruction| {
            pos.move_by(instruction);
            Some(*pos)
        });
    visited_houses.extend(santa_path);

    // Robo-Santa's moves
    let robot_path = INPUT
        .bytes()
        .skip(1)
        .step_by(2)
        .scan(Coord::default(), |pos, instruction| {
            pos.move_by(instruction);
            Some(*pos)
        });
    visited_houses.extend(robot_path);

    visited_houses.len()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(record_present_num1());
}
