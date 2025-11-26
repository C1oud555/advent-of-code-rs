use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day6.txt");

/// Checks if all bytes in a slice are unique.
/// This is faster than creating a HashSet for small slices, as it avoids heap allocation.
fn all_unique(window: &[u8]) -> bool {
    // An array to track which bytes we've seen. ASCII has 128 chars, but we use 256 to be safe.
    let mut seen = [false; 256];
    for &byte in window {
        if seen[byte as usize] {
            // If we've seen this byte before, it's not a unique sequence.
            return false;
        }
        seen[byte as usize] = true;
    }
    // If we get through the whole loop, all characters were unique.
    true
}

/// Finds the index of the first sequence of `n` distinct characters.
fn find_marker(n: usize) -> usize {
    INPUT
        .as_bytes()
        .windows(n)
        .position(all_unique)
        .map(|pos| pos + n) // The problem asks for the index of the *last* character of the marker.
        .unwrap_or_else(|| panic!("No sequence of {} unique characters found", n))
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(find_marker(4))
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(find_marker(14))
}
