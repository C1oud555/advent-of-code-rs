use std::collections::HashSet;

use crate::PUZZLES;
use crate::format_result;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, usize as nom_usize},
    multi::many1,
    sequence::separated_pair,
};

use linkme::distributed_slice;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn parse_line(i: &str) -> IResult<&str, (&str, &str, usize)> {
    separated_pair(
        separated_pair(alpha1, tag(" to "), alpha1),
        tag(" = "),
        nom_usize,
    )
    .map_res(|((from, to), distance)| Ok::<_, &str>((from, to, distance)))
    .parse(i)
}

fn parse_input(i: &str) -> Vec<(&str, &str, usize)> {
    let (_rest, routes) = many1((parse_line, line_ending).map_res(|(g, _)| Ok::<_, &str>(g)))
        .parse(i)
        .unwrap();

    routes
}

const INPUT: &str = include_str!("../inputs/day9.txt");

fn get_distance(from: &str, to: &str, routes: &Vec<(&str, &str, usize)>) -> usize {
    routes
        .iter()
        .find_map(|x| {
            if x.0 == from && x.1 == to {
                Some(x.2)
            } else {
                None
            }
        })
        .expect("Not found this path")
}

fn find_min_path(
    from: &str,
    rest_places: HashSet<&str>,
    routes: &Vec<(&str, &str, usize)>,
) -> usize {
    if rest_places.is_empty() {
        return 0;
    }

    rest_places
        .iter()
        .map(|to| {
            let mut rest_places = rest_places.clone();
            rest_places.remove(from);
            get_distance(from, to, routes) + find_min_path(to, rest_places, routes)
        })
        .min()
        .unwrap()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    // TODO: opt with djskara algorithm
    let inputs = parse_input(INPUT);

    let mut places = HashSet::new();
    for route in &inputs {
        let (from, to, _) = route;
        places.insert(*from);
        places.insert(*to);
    }

    let ret = places
        .par_iter()
        .map(|house| {
            let mut rest_places = places.clone();
            rest_places.remove(house);
            find_min_path(house, rest_places, &inputs)
        })
        .min()
        .unwrap();

    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!("template 1");
}
