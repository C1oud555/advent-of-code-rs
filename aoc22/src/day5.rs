use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day5.txt");

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

/// The two crane models move crates differently.
enum CraneModel {
    CrateMover9000, // Moves one by one, reversing order
    CrateMover9001, // Moves as a single block, preserving order
}

/// Parses the entire input into an initial stack configuration and a list of moves.
fn parse_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let (drawing, moves_str) = INPUT.split_once("\n\n").unwrap();

    // Parse the initial stacks configuration
    let mut drawing_lines = drawing.lines().rev();
    let num_line = drawing_lines.next().unwrap();
    let num_stacks = num_line.split_whitespace().count();
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in drawing_lines {
        let chars: Vec<char> = line.chars().collect();
        for (i, stack) in stacks.iter_mut().enumerate() {
            let char_index = 1 + i * 4;
            if let Some(&c) = chars.get(char_index)
                && c.is_ascii_alphabetic()
            {
                stack.push(c);
            }
        }
    }

    // Parse the move instructions
    let moves = moves_str
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Move {
                count: parts[1].parse().unwrap(),
                from: parts[3].parse::<usize>().unwrap() - 1, // 1-indexed to 0-indexed
                to: parts[5].parse::<usize>().unwrap() - 1,   // 1-indexed to 0-indexed
            }
        })
        .collect();

    (stacks, moves)
}

fn solve(crane_model: CraneModel) -> String {
    let (mut stacks, moves) = parse_input();

    for m in &moves {
        let from_stack = &mut stacks[m.from];
        let drain_start = from_stack.len() - m.count;
        let mut drained_crates: Vec<char> = from_stack.drain(drain_start..).collect();

        if matches!(crane_model, CraneModel::CrateMover9000) {
            drained_crates.reverse();
        }

        stacks[m.to].extend(drained_crates);
    }

    stacks.iter().map(|s| s.last().unwrap_or(&' ')).collect()
}

#[distributed_slice(PUZZLES)]
pub fn part1() -> String {
    format_result!(solve(CraneModel::CrateMover9000))
}

#[distributed_slice(PUZZLES)]
pub fn part2() -> String {
    format_result!(solve(CraneModel::CrateMover9001))
}
