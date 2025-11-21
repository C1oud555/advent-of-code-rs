use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day6.txt");

const BSIZE: usize = 1000;

trait Board {
    fn turn_on(&mut self, range: &Range);

    fn turn_off(&mut self, range: &Range);

    fn toggle(&mut self, range: &Range);

    fn lit_cnt(&self) -> usize;
}

struct CntBoard {
    data: Vec<bool>,
}

impl CntBoard {
    fn new() -> Self {
        Self {
            data: vec![false; BSIZE * BSIZE],
        }
    }
}
impl Board for CntBoard {
    fn turn_on(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            self.data[start..=end].fill(true);
        }
    }

    fn turn_off(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            self.data[start..=end].fill(false);
        }
    }

    fn toggle(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            for val in &mut self.data[start..=end] {
                *val = !*val;
            }
        }
    }

    fn lit_cnt(&self) -> usize {
        self.data.iter().filter(|&&v| v).count()
    }
}

struct Range {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

enum Commands {
    Ton(Range),
    Toff(Range),
    Toggle(Range),
}

fn parse_xy(input: &str) -> (usize, usize) {
    let mut comps = input.split(',');
    let x = comps.next().unwrap().parse().unwrap();
    let y = comps.next().unwrap().parse().unwrap();
    (x, y)
}

fn parse_cmds() -> Vec<Commands> {
    INPUT
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cmd_part = if parts[0] == "turn" { 1 } else { 0 };

            let cmd = parts[cmd_part];
            let start_xy = parse_xy(parts[cmd_part + 1]);
            let end_xy = parse_xy(parts[cmd_part + 3]);

            let range = Range {
                start_x: start_xy.0,
                start_y: start_xy.1,
                end_x: end_xy.0,
                end_y: end_xy.1,
            };

            match cmd {
                "on" => Commands::Ton(range),
                "off" => Commands::Toff(range),
                "toggle" => Commands::Toggle(range),
                _ => panic!("invalid command"),
            }
        })
        .collect()
}

fn exe_cmds(board: &mut impl Board, cmds: &Vec<Commands>) {
    for cmd in cmds {
        match cmd {
            Commands::Ton(range) => board.turn_on(range),
            Commands::Toff(range) => board.turn_off(range),
            Commands::Toggle(range) => board.toggle(range),
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let commands = parse_cmds();
    let mut board = CntBoard::new();

    exe_cmds(&mut board, &commands);

    format_result!(board.lit_cnt());
}

struct LightnessBoard {
    data: Vec<usize>,
}

impl LightnessBoard {
    fn new() -> Self {
        Self {
            data: vec![0; BSIZE * BSIZE],
        }
    }
}

impl Board for LightnessBoard {
    fn turn_on(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            for val in &mut self.data[start..=end] {
                *val += 1;
            }
        }
    }

    fn turn_off(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            for val in &mut self.data[start..=end] {
                *val = val.saturating_sub(1);
            }
        }
    }

    fn toggle(&mut self, range: &Range) {
        for y in range.start_y..=range.end_y {
            let start = y * BSIZE + range.start_x;
            let end = y * BSIZE + range.end_x;
            for val in &mut self.data[start..=end] {
                *val += 2;
            }
        }
    }

    fn lit_cnt(&self) -> usize {
        self.data.iter().sum()
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let commands = parse_cmds();
    let mut board = LightnessBoard::new();

    exe_cmds(&mut board, &commands);

    format_result!(board.lit_cnt());
}
