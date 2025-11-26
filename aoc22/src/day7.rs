use std::collections::HashMap;

use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day7.txt");

#[derive(Debug)]
struct Dir {
    files: Vec<u32>,
    subdirs: Vec<u32>,
    parent: u32,
}

fn parse_input() -> HashMap<u32, Dir> {
    let mut dirs: HashMap<u32, Dir> = HashMap::new();
    let mut name_to_id_table: HashMap<String, u32> = HashMap::new();
    let mut ids = 0;

    dirs.insert(
        ids,
        Dir {
            files: vec![],
            subdirs: vec![],
            parent: 0,
        },
    );
    name_to_id_table.insert("/".to_string(), ids);

    let mut cur_dir_id = 0u32;

    for line in INPUT.lines().skip(1) {
        if line.starts_with("$ cd") {
            let dir_name = String::from_utf8_lossy(&line.as_bytes()[5..]).to_string();
            if dir_name == ".." {
                cur_dir_id = dirs.get(&cur_dir_id).unwrap().parent;
            } else {
                cur_dir_id = *name_to_id_table
                    .get(&(cur_dir_id.to_string() + &dir_name))
                    .unwrap();
            }
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
            let dir_name = String::from_utf8_lossy(&line.as_bytes()[4..]).to_string();
            ids += 1;
            name_to_id_table.insert(cur_dir_id.to_string() + &dir_name, ids);
            dirs.get_mut(&cur_dir_id).unwrap().subdirs.push(ids);
            dirs.insert(
                ids,
                Dir {
                    files: vec![],
                    subdirs: vec![],
                    parent: cur_dir_id,
                },
            );
        } else {
            // file
            let mut comps = line.split_whitespace();
            let size = comps.next().unwrap().parse::<u32>().unwrap();
            dirs.get_mut(&cur_dir_id).unwrap().files.push(size);
        }
    }

    dirs
}

fn get_size(id: u32, dirs: &HashMap<u32, Dir>, sizes: &mut HashMap<u32, u32>) -> u32 {
    if let Some(s) = sizes.get(&id) {
        return *s;
    }

    let cur_node = dirs.get(&id).unwrap();

    let mut self_size = cur_node.files.iter().sum::<u32>();

    for subdir in &cur_node.subdirs {
        self_size += get_size(*subdir, dirs, sizes);
    }
    sizes.insert(id, self_size);
    self_size
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let inputs = parse_input();
    let mut sizes = HashMap::new();
    get_size(0, &inputs, &mut sizes);
    let ret = sizes
        .iter()
        .filter(|(_, v)| **v <= 10_0000)
        .map(|x| x.1)
        .sum::<u32>();
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let inputs = parse_input();
    let mut sizes = HashMap::new();
    get_size(0, &inputs, &mut sizes);
    let total = 7000_0000;
    let needed = 3000_0000;
    let current_size = sizes.get(&0).unwrap();
    let ret = sizes
        .iter()
        .filter(|(_, v)| (current_size - *v + needed) < total)
        .map(|x| x.1)
        .min()
        .unwrap();
    format_result!(ret)
}
