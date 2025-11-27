use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day8.txt");

/// Parses the input string into a grid of numbers (u8).
/// This is more idiomatic and efficient than the original implementation.
fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

/// --- Part 1: Visible Trees ---
/// Calculates the number of trees visible from outside the grid.
///
/// # Algorithm
/// This implementation is much more efficient than the original O(N^3) approach.
/// It uses an O(N^2) algorithm by creating a boolean `visible` grid.
///
/// 1. Initialize a `visible` grid of the same dimensions as the tree grid, with all values set to `false`.
/// 2. Perform four sweeps across the grid:
///    - From left to right (for each row)
///    - From right to left (for each row)
///    - From top to bottom (for each column)
///    - From bottom to top (for each column)
/// 3. In each sweep, keep track of the maximum height seen so far. Any tree taller than the max height
///    is visible from that direction, so we mark its position as `true` in the `visible` grid.
/// 4. Finally, count the number of `true` values in the `visible` grid.
fn part1(grid: &[Vec<u8>]) -> usize {
    let (height, width) = (grid.len(), grid[0].len());
    let mut visible = vec![vec![false; width]; height];

    // Left-to-right sweep
    for r in 0..height {
        let mut max_h = -1;
        for c in 0..width {
            let tree_h = grid[r][c] as i8;
            if tree_h > max_h {
                visible[r][c] = true;
                max_h = tree_h;
            }
        }
    }

    // Right-to-left sweep
    for r in 0..height {
        let mut max_h = -1;
        for c in (0..width).rev() {
            let tree_h = grid[r][c] as i8;
            if tree_h > max_h {
                visible[r][c] = true;
                max_h = tree_h;
            }
        }
    }

    // Top-to-bottom sweep
    for c in 0..width {
        let mut max_h = -1;
        for r in 0..height {
            let tree_h = grid[r][c] as i8;
            if tree_h > max_h {
                visible[r][c] = true;
                max_h = tree_h;
            }
        }
    }

    // Bottom-to-top sweep
    for c in 0..width {
        let mut max_h = -1;
        for r in (0..height).rev() {
            let tree_h = grid[r][c] as i8;
            if tree_h > max_h {
                visible[r][c] = true;
                max_h = tree_h;
            }
        }
    }

    visible.iter().flatten().filter(|&&v| v).count()
}

/// --- Part 2: Scenic Score ---
/// Calculates the scenic score for a single tree.
/// This version is safer and more idiomatic than the original, using iterators.
fn scenic_score(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    let our_height = grid[r][c];
    let (height, width) = (grid.len(), grid[0].len());

    // Look up
    let score_up = (0..r)
        .rev()
        .position(|row| grid[row][c] >= our_height)
        .map_or(r, |p| p + 1);

    // Look down
    let score_down = (r + 1..height)
        .position(|row| grid[row][c] >= our_height)
        .map_or(height - r - 1, |p| p + 1);

    // Look left
    let score_left = (0..c)
        .rev()
        .position(|col| grid[r][col] >= our_height)
        .map_or(c, |p| p + 1);

    // Look right
    let score_right = (c + 1..width)
        .position(|col| grid[r][col] >= our_height)
        .map_or(width - c - 1, |p| p + 1);

    score_up * score_down * score_left * score_right
}

/// Calculates the highest scenic score in the grid.
fn part2(grid: &[Vec<u8>]) -> usize {
    let (height, width) = (grid.len(), grid[0].len());
    let mut max_score = 0;

    for r in 0..height {
        for c in 0..width {
            max_score = max_score.max(scenic_score(grid, r, c));
        }
    }
    max_score
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let grid = parse_grid(INPUT);
    format_result!(part1(&grid))
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let grid = parse_grid(INPUT);
    format_result!(part2(&grid))
}
