use crate::PUZZLES;
use crate::format_result;
use std::collections::HashSet;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day19.txt");

struct Rule {
    from: String,
    to: String,
}

fn parse_input() -> (Vec<Rule>, String) {
    let mut rules = Vec::new();
    let mut iter = INPUT.lines();
    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut tmp = line.split(" => ");

        rules.push(Rule {
            from: tmp.next().unwrap().to_string(),
            to: tmp.next().unwrap().to_string(),
        });
    }

    let molecule = iter.next().unwrap();

    (rules, molecule.to_string())
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let (rules, molecule) = parse_input();

    let ret = solve(rules, molecule);

    format_result!(ret);
}

fn solve(rules: Vec<Rule>, molecube: String) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    for rule in rules {
        apply_rule(rule, molecube.to_owned(), &mut set);
    }

    set.len()
}

fn apply_rule(rule: Rule, molecube: String, set: &mut HashSet<String>) {
    let mut g_index = 0;

    while let Some(index) = molecube[g_index..].find(&rule.from) {
        let t_index = g_index + index;
        let tlen = rule.from.len();
        let mut key = String::new();
        key.push_str(&molecube[0..t_index]);
        key.push_str(&rule.to);
        key.push_str(&molecube[t_index + tlen..]);
        set.insert(key);
        g_index = t_index + tlen;
    }
}

const MAX_DEPTH: usize = 10000;

fn backtrace(
    target: &String,
    depth: usize,
    start: &String,
    rules: &Vec<Rule>,
    min_depth: &mut usize,
    max_depth: &mut usize,
) {
    if depth > MAX_DEPTH {
        return;
    }

    if start == target {
        if *min_depth > depth {
            *min_depth = depth;
        }
        if *max_depth < depth {
            *max_depth = depth;
        }
        return;
    }

    for rule in rules {
        let mut start = start.clone();

        while let Some(new_start) = apply_rule1(rule, start) {
            println!("to  : {}", &new_start);
            backtrace(target, depth + 1, &new_start, rules, min_depth, max_depth);
            start = new_start;
        }
    }
}

fn apply_rule1(rule: &Rule, start: String) -> Option<String> {
    if let Some(index) = start.find(&rule.to) {
        let mut new_start = String::new();
        let tlen = rule.to.len();
        new_start.push_str(&start[0..index]);
        new_start.push_str(&rule.from);
        new_start.push_str(&start[index + tlen..]);
        Some(new_start)
    } else {
        None
    }
}

pub fn solve1() -> usize {
    let (mut rules, molecule) = parse_input();
    rules.sort_by(|l, r| l.to.len().cmp(&r.to.len()));
    let mut min_depth = 0;
    let mut max_depth = 0;
    backtrace(
        &"e".to_string(),
        0,
        &molecule,
        &rules,
        &mut min_depth,
        &mut max_depth,
    );
    min_depth
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = solve1();
    format_result!(ret);
}
