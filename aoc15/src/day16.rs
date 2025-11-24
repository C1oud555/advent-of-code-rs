use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, usize as nom_usize},
    combinator::map,
    multi::separated_list1,
};

const INPUT: &str = include_str!("../inputs/day16.txt");

enum Compound {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

struct Record {
    tag: Compound,
    cnt: usize,
}

struct Aunt {
    which: usize,
    records: Vec<Record>,
}

fn str_to_compound(i: &str) -> Compound {
    match i {
        "children" => Compound::Children,
        "cats" => Compound::Cats,
        "samoyeds" => Compound::Samoyeds,
        "pomeranians" => Compound::Pomeranians,
        "akitas" => Compound::Akitas,
        "vizslas" => Compound::Vizslas,
        "goldfish" => Compound::Goldfish,
        "trees" => Compound::Trees,
        "cars" => Compound::Cars,
        "perfumes" => Compound::Perfumes,
        _ => panic!("not valid compound"),
    }
}

fn parse_line(i: &str) -> IResult<&str, Aunt> {
    map(
        (
            tag("Sue "),
            nom_usize,
            tag(": "),
            alpha1,
            tag(": "),
            nom_usize,
            tag(", "),
            alpha1,
            tag(": "),
            nom_usize,
            tag(", "),
            alpha1,
            tag(": "),
            nom_usize,
        ),
        |(_, which, _, cp0, _, cnt0, _, cp1, _, cnt1, _, cp2, _, cnt2)| {
            let records = vec![
                Record {
                    tag: str_to_compound(cp0),
                    cnt: cnt0,
                },
                Record {
                    tag: str_to_compound(cp1),
                    cnt: cnt1,
                },
                Record {
                    tag: str_to_compound(cp2),
                    cnt: cnt2,
                },
            ];

            Aunt { which, records }
        },
    )
    .parse(i)
}

fn parse_aunts(i: &str) -> Vec<Aunt> {
    separated_list1(line_ending, parse_line).parse(i).unwrap().1
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let aunts = parse_aunts(INPUT);
    let ret = aunts
        .iter()
        .find(|aunt| {
            for record in &aunt.records {
                match record.tag {
                    Compound::Children if record.cnt != 3 => return false,
                    Compound::Cats if record.cnt != 7 => return false,
                    Compound::Samoyeds if record.cnt != 2 => return false,
                    Compound::Pomeranians if record.cnt != 3 => return false,
                    Compound::Akitas if record.cnt != 0 => return false,
                    Compound::Vizslas if record.cnt != 0 => return false,
                    Compound::Goldfish if record.cnt != 5 => return false,
                    Compound::Trees if record.cnt != 3 => return false,
                    Compound::Cars if record.cnt != 2 => return false,
                    Compound::Perfumes if record.cnt != 1 => return false,
                    _ => {}
                }
            }
            true
        })
        .expect("Not found satified aunt")
        .which;
    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let aunts = parse_aunts(INPUT);
    let ret = aunts
        .iter()
        .find(|aunt| {
            for record in &aunt.records {
                match record.tag {
                    Compound::Children if record.cnt != 3 => return false,
                    Compound::Cats if record.cnt <= 7 => return false,
                    Compound::Samoyeds if record.cnt != 2 => return false,
                    Compound::Pomeranians if record.cnt >= 3 => return false,
                    Compound::Akitas if record.cnt != 0 => return false,
                    Compound::Vizslas if record.cnt != 0 => return false,
                    Compound::Goldfish if record.cnt >= 5 => return false,
                    Compound::Trees if record.cnt <= 3 => return false,
                    Compound::Cars if record.cnt != 2 => return false,
                    Compound::Perfumes if record.cnt != 1 => return false,
                    _ => {}
                }
            }
            true
        })
        .expect("Not found satified aunt")
        .which;
    format_result!(ret);
}
