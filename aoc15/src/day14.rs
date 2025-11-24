use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, usize as nom_usize},
    combinator::{map, map_res},
    multi::many1,
};

const INPUT: &str = include_str!("../inputs/day14.txt");

struct Reindeer {
    speed: usize,
    flying_time: usize,
    rest_time: usize,
}

fn parse_line(i: &str) -> IResult<&str, Reindeer> {
    map(
        (
            alpha1,
            tag(" can fly "),
            nom_usize,
            tag(" km/s for "),
            nom_usize,
            tag(" seconds, but then must rest for "),
            nom_usize,
            tag(" seconds."),
        ),
        |(_, _, speed, _, flying_time, _, rest_time, _)| Reindeer {
            speed,
            flying_time,
            rest_time,
        },
    )
    .parse(i)
}

fn parse_many(i: &str) -> Vec<Reindeer> {
    many1(map_res((parse_line, line_ending), |(route, _)| {
        Ok::<_, nom::error::Error<&str>>(route)
    }))
    .parse(i)
    .unwrap()
    .1
}

fn calc_distance(deer: &Reindeer, seconds: usize) -> usize {
    let segment = deer.flying_time + deer.rest_time;
    let total_seg = seconds / segment;
    let rest_time = seconds % segment;
    total_seg * deer.flying_time * deer.speed
        + std::cmp::min(rest_time, deer.flying_time) * deer.speed
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let seconds = 2503;
    let ret = parse_many(INPUT)
        .iter()
        .map(|deer| calc_distance(deer, seconds))
        .max()
        .unwrap();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let seconds = 2503;
    let deers = parse_many(INPUT);
    let mut starts = vec![0; deers.len()];
    let mut distances = vec![0; deers.len()];

    for cur_sec in 1..=seconds {
        for (idx, deer) in deers.iter().enumerate() {
            distances[idx] = calc_distance(deer, cur_sec);
        }
        let lead = distances.iter().max().unwrap();
        for (idx, distance) in distances.iter().enumerate() {
            if distance == lead {
                starts[idx] += 1;
            }
        }
    }

    let ret = starts.iter().max().unwrap();

    format_result!(ret)
}
