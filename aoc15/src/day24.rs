use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day24.txt");

fn parse_input() -> Vec<usize> {
    INPUT
        .lines()
        .map(|line| line.parse::<usize>().expect("Not valid number"))
        .collect()
}

struct CurrentState {
    sum: usize,
    qe: usize,
    len: usize,
}

struct ProblemConfig<'a> {
    packages: &'a [usize],
    target_sum: usize,
}

struct BestSolution {
    len: usize,
    qe: usize,
}

fn find_best_group(
    index: usize,
    current: CurrentState,
    config: &ProblemConfig,
    best: &mut BestSolution,
) {
    // Aggressive pruning: if current length already matches or exceeds the best found so far,
    // or if current sum exceeds target, this path cannot be better.
    if current.len >= best.len && current.sum != config.target_sum {
        return;
    }
    if current.sum > config.target_sum {
        return;
    }

    // Base case: if target sum is reached
    if current.sum == config.target_sum {
        if current.len < best.len {
            // New best length found
            best.len = current.len;
            best.qe = current.qe;
        } else if current.len == best.len && current.qe < best.qe {
            // Same best length, but better quantum entanglement
            best.qe = current.qe;
        }
        return; // Found a valid group, no need to add more packages to this group
    }

    // Base case: if all packages have been considered
    if index == config.packages.len() {
        return;
    }

    // Recursive step 1: Include the current package
    let package = config.packages[index];
    find_best_group(
        index + 1,
        CurrentState {
            sum: current.sum + package,
            qe: current.qe * package,
            len: current.len + 1,
        },
        config,
        best,
    );

    // Recursive step 2: Exclude the current package
    find_best_group(
        index + 1,
        current, // Pass the current state without modification
        config,
        best,
    );
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut packages = parse_input();
    packages.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending for better pruning
    let total_sum: usize = packages.iter().sum();
    let target_sum = total_sum / 3;

    let config = ProblemConfig {
        packages: &packages,
        target_sum,
    };
    let mut best = BestSolution {
        len: usize::MAX,
        qe: usize::MAX,
    };
    let initial_state = CurrentState {
        sum: 0,
        qe: 1,
        len: 0,
    };

    find_best_group(0, initial_state, &config, &mut best);

    format_result!(best.qe)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let mut packages = parse_input();
    packages.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending for better pruning
    let total_sum: usize = packages.iter().sum();
    let target_sum = total_sum / 4;

    let config = ProblemConfig {
        packages: &packages,
        target_sum,
    };
    let mut best = BestSolution {
        len: usize::MAX,
        qe: usize::MAX,
    };
    let initial_state = CurrentState {
        sum: 0,
        qe: 1,
        len: 0,
    };

    find_best_group(0, initial_state, &config, &mut best);

    format_result!(best.qe)
}
