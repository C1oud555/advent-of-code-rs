use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../inputs/day16.txt");

static TICKER_TAPE: Lazy<HashMap<Compound, usize>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(Compound::Children, 3);
    m.insert(Compound::Cats, 7);
    m.insert(Compound::Samoyeds, 2);
    m.insert(Compound::Pomeranians, 3);
    m.insert(Compound::Akitas, 0);
    m.insert(Compound::Vizslas, 0);
    m.insert(Compound::Goldfish, 5);
    m.insert(Compound::Trees, 3);
    m.insert(Compound::Cars, 2);
    m.insert(Compound::Perfumes, 1);
    m
});

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl FromStr for Compound {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(Compound::Children),
            "cats" => Ok(Compound::Cats),
            "samoyeds" => Ok(Compound::Samoyeds),
            "pomeranians" => Ok(Compound::Pomeranians),
            "akitas" => Ok(Compound::Akitas),
            "vizslas" => Ok(Compound::Vizslas),
            "goldfish" => Ok(Compound::Goldfish),
            "trees" => Ok(Compound::Trees),
            "cars" => Ok(Compound::Cars),
            "perfumes" => Ok(Compound::Perfumes),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Aunt {
    id: usize,
    properties: HashMap<Compound, usize>,
}

fn parse_aunts(input: &str) -> Vec<Aunt> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            let id = parts[0].replace("Sue ", "").parse().ok()?;
            let mut properties = HashMap::new();

            let props_str = parts[1];
            let props_parts: Vec<&str> = props_str.split(", ").collect();

            for part in props_parts {
                let prop_val: Vec<&str> = part.split(": ").collect();
                if let (Ok(compound), Ok(value)) = (prop_val[0].parse(), prop_val[1].parse()) {
                    properties.insert(compound, value);
                }
            }
            Some(Aunt { id, properties })
        })
        .collect()
}

fn solve<F>(aunts: &[Aunt], predicate: F) -> usize
where
    F: Fn(&Compound, &usize, &usize) -> bool,
{
    aunts
        .iter()
        .find(|aunt| {
            TICKER_TAPE.iter().all(|(compound, &ticker_value)| {
                aunt.properties
                    .get(compound)
                    .is_none_or(|&aunt_value| predicate(compound, &aunt_value, &ticker_value))
            })
        })
        .unwrap()
        .id
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let aunts = parse_aunts(INPUT);
    let result = solve(&aunts, |_compound, &aunt_value, &ticker_value| {
        aunt_value == ticker_value
    });
    format_result!(result)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let aunts = parse_aunts(INPUT);
    let result = solve(
        &aunts,
        |compound, &aunt_value, &ticker_value| match compound {
            Compound::Cats | Compound::Trees => aunt_value > ticker_value,
            Compound::Pomeranians | Compound::Goldfish => aunt_value < ticker_value,
            _ => aunt_value == ticker_value,
        },
    );
    format_result!(result)
}
