use std::collections::HashMap;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, isize as nom_isize, line_ending},
    combinator::{map, map_res},
    multi::many1,
};

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

fn parse_many(i: &str) -> Vec<Ingredient> {
    many1(map_res((parse_line, line_ending), |(route, _)| {
        Ok::<_, nom::error::Error<&str>>(route)
    }))
    .parse(i)
    .unwrap()
    .1
}

fn scores(choosed: &HashMap<Ingredient, isize>) -> isize {
    let (c, d, f, t) = choosed.iter().fold((0, 0, 0, 0), |(c, d, f, t), (i, cnt)| {
        (
            c + cnt * i.capacity,
            d + cnt * i.durability,
            f + cnt * i.flavor,
            t + cnt * i.texture,
        )
    });
    println!(
        "{c} {d} {f} {t} : {}",
        c.max(0) * d.max(0) * f.max(0) * t.max(0)
    );
    c.max(0) * d.max(0) * f.max(0) * t.max(0)
}

fn fake_scores(choosed: &HashMap<Ingredient, isize>) -> isize {
    let (c, d, f, t) = choosed.iter().fold((0, 0, 0, 0), |(c, d, f, t), (i, cnt)| {
        (
            c + cnt * i.capacity,
            d + cnt * i.durability,
            f + cnt * i.flavor,
            t + cnt * i.texture,
        )
    });
    println!(
        "{c} {d} {f} {t} {}",
        c.max(1) * d.max(1) * f.max(1) * t.max(1)
    );
    c.max(1) * d.max(1) * f.max(1) * t.max(1)
}

fn find_max(
    iter_time: usize,
    avai: &[Ingredient],
    choosed: &mut HashMap<Ingredient, isize>,
) -> isize {
    for _ in 0..iter_time {
        let max_ingre = avai
            .iter()
            .map(|ingre| {
                let mut tmp = choosed.clone();
                tmp.entry(*ingre).and_modify(|v| *v += 1).or_insert(1);
                let ret = fake_scores(&tmp);
                (ingre, ret)
            })
            .max_by(|l, r| l.1.cmp(&r.1))
            .unwrap();
        choosed
            .entry(*max_ingre.0)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    scores(choosed)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let avai = parse_many(INPUT);
    let mut choosed = HashMap::new();

    let ret = find_max(100, &avai, &mut choosed);

    format_result!(ret);
}
fn find_max_withconstrait(avai: &[Ingredient], choosed: &mut HashMap<Ingredient, isize>) -> isize {
    let avai8: Vec<Ingredient> = avai.iter().filter(|x| x.calories == 8).copied().collect();
    let avai3: Vec<Ingredient> = avai.iter().filter(|x| x.calories == 3).copied().collect();
    let a80 = avai8[0];
    let a81 = avai8[1];
    let a30 = avai3[0];
    let a31 = avai3[1];
    choosed.insert(a80, 0);
    choosed.insert(a81, 0);
    choosed.insert(a30, 0);
    choosed.insert(a31, 0);
    let mut score = 0;
    for c80_cnt in 0..40 {
        let c81_cnt = 40 - c80_cnt;
        for c30_cnt in 0..60 {
            let c31_cnt = 60 - c30_cnt;
            choosed.entry(a80).and_modify(|x| *x = c80_cnt);
            choosed.entry(a81).and_modify(|x| *x = c81_cnt);
            choosed.entry(a30).and_modify(|x| *x = c30_cnt);
            choosed.entry(a31).and_modify(|x| *x = c31_cnt);

            if scores(choosed) > score {
                score = scores(choosed);
            }
        }
    }

    score
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let avai = parse_many(INPUT);
    let mut choosed = HashMap::new();
    let ret = find_max_withconstrait(&avai, &mut choosed);

    let total_calories: isize = choosed.iter().map(|(k, cnt)| k.calories * cnt).sum();

    println!("total calories: {}", total_calories);

    format_result!(ret);
}
