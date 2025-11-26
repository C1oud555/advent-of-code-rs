use crate::{PUZZLES, format_result};
use linkme::distributed_slice;
use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/day4.txt");

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// Checks if this range fully contains another range.
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Checks if this range overlaps with another range.
    /// Two ranges overlap if the end of one is after the start of the other,
    /// and vice-versa.
    fn overlaps(&self, other: &Self) -> bool {
        self.end >= other.start && other.end >= self.start
    }
}

/// Parses a string like "2-4" into a Range.
impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("Invalid start")?;
        let end = parts
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("Invalid end")?;
        Ok(Range { start, end })
    }
}

/// Parses a line like "2-4,6-8" into a pair of Ranges.
fn parse_line(line: &str) -> (Range, Range) {
    let mut parts = line.split(',');
    let range1 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Invalid first range");
    let range2 = parts
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Invalid second range");
    (range1, range2)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let count = INPUT
        .lines()
        .filter(|line| {
            let (r1, r2) = parse_line(line);
            r1.contains(&r2) || r2.contains(&r1)
        })
        .count();

    format_result!(count)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let count = INPUT
        .lines()
        .filter(|line| {
            let (r1, r2) = parse_line(line);
            r1.overlaps(&r2)
        })
        .count();

    format_result!(count)
}
