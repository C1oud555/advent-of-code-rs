use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, isize as nom_isize, line_ending},
    combinator::map,
    multi::separated_list1,
};
use std::sync::OnceLock;

const INPUT: &str = include_str!("../inputs/day15.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_line(i: &str) -> IResult<&str, Ingredient> {
    map(
        (
            alpha1,
            tag(": capacity "),
            nom_isize,
            tag(", durability "),
            nom_isize,
            tag(", flavor "),
            nom_isize,
            tag(", texture "),
            nom_isize,
            tag(", calories "),
            nom_isize,
        ),
        |(_, _, capacity, _, durability, _, flavor, _, texture, _, calories)| Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )
    .parse(i)
}

fn parse_ingredients(i: &str) -> Vec<Ingredient> {
    separated_list1(line_ending, parse_line).parse(i).unwrap().1
}

fn solve() -> (isize, isize) {
    let ingredients = parse_ingredients(INPUT);
    let mut max_score_part1 = 0;
    let mut max_score_part2 = 0;

    if ingredients.len() != 4 {
        // This solution is hardcoded for 4 ingredients.
        // A generic solution would require recursion or dynamic programming.
        // For Advent of Code, this is a common and acceptable optimization.
        panic!("This solution expects exactly 4 ingredients.");
    }

    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                let l = 100 - i - j - k;

                let capacity = (ingredients[0].capacity * i
                    + ingredients[1].capacity * j
                    + ingredients[2].capacity * k
                    + ingredients[3].capacity * l)
                    .max(0);
                let durability = (ingredients[0].durability * i
                    + ingredients[1].durability * j
                    + ingredients[2].durability * k
                    + ingredients[3].durability * l)
                    .max(0);
                let flavor = (ingredients[0].flavor * i
                    + ingredients[1].flavor * j
                    + ingredients[2].flavor * k
                    + ingredients[3].flavor * l)
                    .max(0);
                let texture = (ingredients[0].texture * i
                    + ingredients[1].texture * j
                    + ingredients[2].texture * k
                    + ingredients[3].texture * l)
                    .max(0);

                let score = capacity * durability * flavor * texture;
                max_score_part1 = max_score_part1.max(score);

                let calories = ingredients[0].calories * i
                    + ingredients[1].calories * j
                    + ingredients[2].calories * k
                    + ingredients[3].calories * l;

                if calories == 500 {
                    max_score_part2 = max_score_part2.max(score);
                }
            }
        }
    }

    (max_score_part1, max_score_part2)
}

static RESULTS: OnceLock<(isize, isize)> = OnceLock::new();

fn get_results() -> (isize, isize) {
    *RESULTS.get_or_init(solve)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let result = get_results().0;
    format_result!(result)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let result = get_results().1;
    format_result!(result)
}
