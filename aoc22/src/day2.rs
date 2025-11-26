use crate::{PUZZLES, format_result};
use linkme::distributed_slice;
use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/day2.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// The score for the shape itself.
    fn shape_score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    /// Determines the outcome of a match against an opponent.
    fn outcome(&self, opponent: &Move) -> Outcome {
        match (self, opponent) {
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    /// Determines which move to make to achieve the desired outcome.
    fn for_outcome(opponent: Move, outcome: Outcome) -> Self {
        match (opponent, outcome) {
            (m, Outcome::Draw) => m,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move"),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    /// The score for the outcome of the round.
    fn outcome_score(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Invalid outcome"),
        }
    }
}

fn parse_line<T: FromStr, U: FromStr>(line: &str) -> (T, U) {
    let mut parts = line.split_whitespace();
    let first = parts
        .next()
        .unwrap()
        .parse()
        .unwrap_or_else(|_| panic!("Invalid first part"));
    let second = parts
        .next()
        .unwrap()
        .parse()
        .unwrap_or_else(|_| panic!("Invalid second part"));
    (first, second)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let total_score: usize = INPUT
        .lines()
        .map(|line| {
            let (opponent_move, my_move): (Move, Move) = parse_line(line);
            let outcome = my_move.outcome(&opponent_move);
            my_move.shape_score() + outcome.outcome_score()
        })
        .sum();

    format_result!(total_score)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let total_score: usize = INPUT
        .lines()
        .map(|line| {
            let (opponent_move, desired_outcome): (Move, Outcome) = parse_line(line);
            let my_move = Move::for_outcome(opponent_move, desired_outcome);
            my_move.shape_score() + desired_outcome.outcome_score()
        })
        .sum();

    format_result!(total_score)
}
