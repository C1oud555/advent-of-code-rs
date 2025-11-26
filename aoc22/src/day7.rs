use crate::{PUZZLES, format_result};
use linkme::distributed_slice;
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day7.txt");

const TOTAL_DISK_SPACE: u32 = 70_000_000;
const NEEDED_UPDATE_SPACE: u32 = 30_000_000;
const PART1_MAX_SIZE: u32 = 100_000;

struct Node {
    /// The index of the parent node in the arena.
    parent: Option<usize>,
    /// A map from directory name to the index of the child node.
    children: HashMap<String, usize>,
    /// The sum of sizes of files directly within this directory.
    direct_file_size: u32,
}

impl Node {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            children: HashMap::new(),
            direct_file_size: 0,
        }
    }
}

/// Parses the input and returns a vector containing the total sizes of every directory.
fn get_all_directory_sizes() -> Vec<u32> {
    let mut arena = vec![Node::new(None)]; // Root node at index 0
    let mut current_node_idx = 0;

    // --- Pass 1: Build the tree structure ---
    for line in INPUT.lines().skip(1) {
        // Skip the first "$ cd /"
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                current_node_idx = arena[current_node_idx].parent.unwrap();
            }
            ["$", "cd", dir_name] => {
                current_node_idx = arena[current_node_idx].children[dir_name];
            }
            ["$", "ls"] => { /* Do nothing, the following lines are the content */ }
            ["dir", dir_name] => {
                let new_node = Node::new(Some(current_node_idx));
                let new_node_idx = arena.len();
                arena.push(new_node);
                arena[current_node_idx]
                    .children
                    .insert(dir_name.to_string(), new_node_idx);
            }
            [size, _file_name] => {
                if let Ok(file_size) = size.parse::<u32>() {
                    arena[current_node_idx].direct_file_size += file_size;
                }
            }
            _ => { /* Should not happen with valid input */ }
        }
    }

    // --- Pass 2: Calculate total sizes ---
    // We iterate backwards from the last node to the root. This is a topological sort
    // that ensures children are always processed before their parents.
    let mut total_sizes = vec![0; arena.len()];
    for i in (0..arena.len()).rev() {
        let node = &arena[i];
        let children_total_size: u32 = node
            .children
            .values()
            .map(|&child_idx| total_sizes[child_idx])
            .sum();
        total_sizes[i] = node.direct_file_size + children_total_size;
    }

    total_sizes
}

#[distributed_slice(PUZZLES)]
pub fn part1() -> String {
    let all_sizes = get_all_directory_sizes();
    let sum: u32 = all_sizes.iter().filter(|&&s| s <= PART1_MAX_SIZE).sum();
    format_result!(sum)
}

#[distributed_slice(PUZZLES)]
pub fn part2() -> String {
    let all_sizes = get_all_directory_sizes();
    let used_space = all_sizes[0]; // Root directory size is the total used space
    let free_space = TOTAL_DISK_SPACE - used_space;
    let space_to_free = NEEDED_UPDATE_SPACE - free_space;

    let min_dir_to_delete = all_sizes
        .iter()
        .filter(|&&s| s >= space_to_free)
        .min()
        .unwrap();

    format_result!(min_dir_to_delete)
}
