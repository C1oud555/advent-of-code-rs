use std::collections::VecDeque;

use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day5.txt");

struct Move {
    cnt: u32,
    from: u32,
    to: u32,
}

fn parse_input() -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut init_state: Vec<VecDeque<char>> = Vec::new();
    let mut moves = Vec::new();

    for line in INPUT.lines() {
        if line.starts_with("[") {
            for (index, bbs) in line.as_bytes().chunks(4).enumerate() {
                if bbs[1] != b' ' {
                    while init_state.get(index).is_none() {
                        init_state.push(VecDeque::new());
                    }
                    init_state[index].push_front(bbs[1] as char);
                }
            }
        } else if line.starts_with("move") {
            let mut comps = line.split_whitespace();
            let _ = comps.next();
            let cnt = comps.next().unwrap().parse::<u32>().unwrap();
            let _ = comps.next();
            let from = comps.next().unwrap().parse::<u32>().unwrap() - 1;
            let _ = comps.next();
            let to = comps.next().unwrap().parse::<u32>().unwrap() - 1;
            moves.push(Move { cnt, from, to })
        }
    }

    (init_state, moves)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let (mut stacks, moves) = parse_input();
    for stack in &stacks {
        println!("{:?}", stack);
    }
    for mmove in &moves {
        for _ in 0..mmove.cnt {
            let tmp = stacks[mmove.from as usize].pop_back().unwrap();
            stacks[mmove.to as usize].push_back(tmp);
        }
    }
    let ret: String = stacks.iter_mut().map(|x| x.pop_back().unwrap()).collect();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let (mut stacks, moves) = parse_input();
    for stack in &stacks {
        println!("{:?}", stack);
    }
    for mmove in &moves {
        let mut tmp_vec = Vec::new();
        for _ in 0..mmove.cnt {
            let tmp = stacks[mmove.from as usize].pop_back().unwrap();
            tmp_vec.push(tmp);
        }
        tmp_vec.reverse();
        for cc in tmp_vec {
            stacks[mmove.to as usize].push_back(cc);
        }
    }
    let ret: String = stacks.iter_mut().map(|x| x.pop_back().unwrap()).collect();
    format_result!(ret)
}
