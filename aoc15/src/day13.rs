use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, u32 as nom_u32},
};
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("../inputs/day13.txt");

struct SeatingArrangement {
    distances: Vec<Vec<isize>>,
    people_count: usize,
}

impl SeatingArrangement {
    fn new(routes_data: &[(String, String, isize)], add_self: bool) -> Self {
        let mut people_to_id = FxHashMap::default();
        let mut get_id = |person: &str| {
            let next_id = people_to_id.len();
            *people_to_id.entry(person.to_string()).or_insert(next_id)
        };

        for (from, to, _) in routes_data {
            get_id(from);
            get_id(to);
        }

        if add_self {
            get_id("me");
        }

        let people_count = people_to_id.len();
        let mut distances = vec![vec![0; people_count]; people_count];

        for (from, to, dist) in routes_data {
            if let (Some(&from_id), Some(&to_id)) = (people_to_id.get(from), people_to_id.get(to)) {
                distances[from_id][to_id] = *dist;
            }
        }

        (0..people_count).for_each(|i| {
            ((i + 1)..people_count).for_each(|j| {
                distances[i][j] += distances[j][i];
                distances[j][i] = distances[i][j];
            });
        });

        Self {
            distances,
            people_count,
        }
    }

    fn solve_tsp(&self) -> isize {
        if self.people_count < 2 {
            return 0;
        }
        self.find_max_path_recursive(0, 1 << 0)
    }

    fn find_max_path_recursive(&self, current_id: usize, visited_mask: usize) -> isize {
        if visited_mask == (1 << self.people_count) - 1 {
            return self.distances[current_id][0];
        }

        let mut max_dist = isize::MIN;

        for next_id in 0..self.people_count {
            if (visited_mask >> next_id) & 1 == 0 {
                let dist = self.distances[current_id][next_id]
                    + self.find_max_path_recursive(next_id, visited_mask | (1 << next_id));
                if dist > max_dist {
                    max_dist = dist;
                }
            }
        }
        max_dist
    }
}
fn parse_line(i: &str) -> IResult<&str, (String, String, isize)> {
    let (i, p1) = alpha1(i)?;
    let (i, _) = tag(" would ")(i)?;
    let (i, action) = alpha1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, amount) = nom_u32(i)?;
    let (i, _) = tag(" happiness units by sitting next to ")(i)?;
    let (i, p2) = alpha1(i)?;
    let (i, _) = tag(".")(i)?;

    let sign = if action == "gain" { 1 } else { -1 };
    Ok((i, (p1.to_string(), p2.to_string(), amount as isize * sign)))
}

fn parse_input(i: &str) -> Vec<(String, String, isize)> {
    i.trim()
        .split('\n')
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn solve(add_self: bool) -> isize {
    let routes = parse_input(INPUT);
    let arrangement = SeatingArrangement::new(&routes, add_self);
    arrangement.solve_tsp()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let result = solve(false);
    format_result!(result)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let result = solve(true);
    format_result!(result)
}
