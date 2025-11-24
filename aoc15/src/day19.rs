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
    let ret = solve_part1(&rules, &molecule);
    format_result!(ret)
}

fn solve_part1(rules: &[Rule], molecule: &str) -> usize {
    let mut distinct_molecules: HashSet<String> = HashSet::new();

    for rule in rules {
        // Use `match_indices` to find all non-overlapping occurrences of the substring.
        // This is more idiomatic and efficient than a manual while-let loop.
        for (start_index, _) in molecule.match_indices(&rule.from) {
            // Pre-allocate the new string with an appropriate capacity to reduce reallocations.
            let mut new_molecule =
                String::with_capacity(molecule.len() - rule.from.len() + rule.to.len());
            new_molecule.push_str(&molecule[..start_index]);
            new_molecule.push_str(&rule.to);
            new_molecule.push_str(&molecule[start_index + rule.from.len()..]);

            distinct_molecules.insert(new_molecule);
        }
    }

    distinct_molecules.len()
}

pub fn solve1() -> usize {
    let (mut rules, molecule) = parse_input();

    let mut count = 0;

    let mut current_molecule = molecule.clone();

    // Sort rules by the length of the "to" string in descending order.

    // This makes the greedy approach more effective by replacing longer sequences first.

    rules.sort_by(|a, b| b.to.len().cmp(&a.to.len()));

    while current_molecule != "e" {
        for rule in &rules {
            // Using rfind to work from the end of the molecule, which is a good

            // heuristic for reduction parsing.

            if let Some(index) = current_molecule.rfind(&rule.to) {
                current_molecule.replace_range(index..index + rule.to.len(), &rule.from);

                count += 1;

                // Restart the scan from the beginning of the rule list

                // to ensure we always apply the longest possible match first.

                break;
            }
        }
    }

    count
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = solve1();
    format_result!(ret)
}
