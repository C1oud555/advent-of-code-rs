use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day8.txt");

/// Calculates the difference between the number of characters in the code representation
/// of a string literal and the number of characters in its in-memory representation.
///
/// This difference is the "overhead" caused by encoding characters for the literal.
/// For example:
/// - `""` is 2 characters in code, 0 in memory. Overhead is 2.
/// - `"abc"` is 5 characters in code, 3 in memory. Overhead is 2 (for the quotes).
/// - `"\""` is 3 characters in code, 1 in memory. Overhead is 2.
/// - `"\\"` is 4 characters in code, 1 in memory. Overhead is 3.
/// - `"\x27"` is 6 characters in code, 1 in memory. Overhead is 5.
fn decoded_overhead(literal: &str) -> usize {
    let mut overhead = 2; // Start with 2 for the outer quotes.
    let mut it = literal.chars();

    it.next(); // Skip initial quote, as we assume valid literals.

    loop {
        match it.next() {
            Some('\\') => {
                // An escape sequence always adds to the overhead.
                match it.next() {
                    Some('x') => {
                        // `\xHH` is 4 chars in code for 1 in memory.
                        // The `\` itself contributes 1, `xHH` contributes 2 more = 3 total.
                        it.next(); // H
                        it.next(); // H
                        overhead += 3;
                    }
                    Some(_) => {
                        // `\\` or `\"` is 2 chars in code for 1 in memory. Overhead += 1.
                        overhead += 1;
                    }
                    None => break, // Malformed string, end of literal.
                }
            }
            Some('"') => break, // End of literal.
            Some(_) => (),      // Regular character, no overhead.
            None => break,      // End of literal.
        }
    }
    overhead
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    // The puzzle asks for the sum of all literal lengths minus the sum of all memory lengths.
    // This is equivalent to summing the overhead for each line.
    let total_overhead: usize = INPUT.lines().map(decoded_overhead).sum();
    format_result!(total_overhead);
}

/// Calculates the number of extra characters needed to re-encode a string literal.
///
/// This is the difference between the new encoded length and the original literal length.
/// The re-encoding process involves wrapping the string in new quotes and escaping
/// existing backslashes and quotes.
fn encoded_overhead(literal: &str) -> usize {
    // The overhead comes from:
    // - 2 characters for the new outer quotes.
    // - 1 extra character for each existing `"` or `\` which needs to be escaped.
    2 + literal.chars().filter(|&c| c == '"' || c == '\\').count()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    // The puzzle asks for the total length of all newly encoded strings minus the
    // total length of all original string literals.
    // This is equivalent to summing the encoding overhead for each line.
    let total_overhead: usize = INPUT.lines().map(encoded_overhead).sum();
    format_result!(total_overhead);
}
