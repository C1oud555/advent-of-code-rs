use std::collections::HashSet;

use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day9.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    num: i32,
}

fn parse_cmds(i: &str) -> Vec<Move> {
    i.lines()
        .map(|line| {
            let mut ll = line.split_whitespace();
            let dir = ll.next().unwrap();
            let direction = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("Unknown direction: {}", dir),
            };

            let num = ll.next().unwrap().parse::<i32>().unwrap();
            Move { direction, num }
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn move_by(&mut self, mv: &Direction) {
        match mv {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    fn follow(&mut self, head: &Coord) {
        if (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1 {
            return;
        }
        if self.y < head.y {
            self.y += 1;
        } else if self.y > head.y {
            self.y -= 1;
        }
        if self.x < head.x {
            self.x += 1;
        } else if self.x > head.x {
            self.x -= 1;
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let moves = parse_cmds(INPUT);
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };
    let mut tail_records = HashSet::new();
    for cmd in moves {
        for _ in 0..cmd.num {
            head.move_by(&cmd.direction);
            tail.follow(&head);
            println!("tail: {} {}", tail.x, tail.y);
            tail_records.insert((tail.x, tail.y));
        }
    }
    let ret = tail_records.len();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let moves = parse_cmds(INPUT);
    let mut rope = [Coord { x: 0, y: 0 }; 10];
    let mut tail_records = HashSet::new();
    for cmd in moves {
        for _ in 0..cmd.num {
            rope[0].move_by(&cmd.direction);
            for i in 1..rope.len() {
                let tmp = rope[i - 1];
                rope[i].follow(&tmp);
            }
            tail_records.insert((rope[9].x, rope[9].y));
        }
    }
    let ret = tail_records.len();
    format_result!(ret)
}
