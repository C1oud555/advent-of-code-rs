use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    combinator::map,
    sequence::preceded,
};

const INPUT: &str = include_str!("../inputs/day6.txt");
const BSIZE: usize = 1000;

// --- Parsing ---

enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Command {
    op: CommandType,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_coord(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        nom::sequence::separated_pair(complete::u32, tag(","), complete::u32),
        |(x, y)| (x as usize, y as usize),
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Command> {
    let (input, op_str) = alt((tag("turn on"), tag("turn off"), tag("toggle"))).parse(input)?;
    let (input, start) = preceded(space1, parse_coord).parse(input)?;
    let (input, end) = preceded((space1, tag("through"), space1), parse_coord).parse(input)?;

    let op = match op_str {
        "turn on" => CommandType::TurnOn,
        "turn off" => CommandType::TurnOff,
        "toggle" => CommandType::Toggle,
        _ => unreachable!(),
    };

    Ok((input, Command { op, start, end }))
}

// --- Part 1 ---

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let commands: Vec<Command> = INPUT.lines().map(|l| parse_line(l).unwrap().1).collect();
    let mut board = vec![false; BSIZE * BSIZE];

    for cmd in &commands {
        let (x1, y1) = cmd.start;
        let (x2, y2) = cmd.end;

        for y in y1..=y2 {
            let start = y * BSIZE + x1;
            let end = y * BSIZE + x2;
            match cmd.op {
                CommandType::TurnOn => board[start..=end].fill(true),
                CommandType::TurnOff => board[start..=end].fill(false),
                CommandType::Toggle => {
                    for val in &mut board[start..=end] {
                        *val = !*val;
                    }
                }
            }
        }
    }
    let lit_count = board.iter().filter(|&&v| v).count();
    format_result!(lit_count)
}

// --- Part 2 ---

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let commands: Vec<Command> = INPUT.lines().map(|l| parse_line(l).unwrap().1).collect();
    let mut board = vec![0u32; BSIZE * BSIZE];

    for cmd in &commands {
        let (x1, y1) = cmd.start;
        let (x2, y2) = cmd.end;

        for y in y1..=y2 {
            let start = y * BSIZE + x1;
            let end = y * BSIZE + x2;
            for light in &mut board[start..=end] {
                match cmd.op {
                    CommandType::TurnOn => *light += 1,
                    CommandType::TurnOff => *light = light.saturating_sub(1),
                    CommandType::Toggle => *light += 2,
                }
            }
        }
    }

    let total_brightness: u64 = board.iter().map(|&v| v as u64).sum();
    format_result!(total_brightness)
}
