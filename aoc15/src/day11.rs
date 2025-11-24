use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;

const INPUT: &[u8; 8] = b"cqjxjnds";

// Increments password, like "xx" -> "xy", or "xz" -> "ya"
fn inc_password(password: &mut [u8; 8]) {
    for ch in password.iter_mut().rev() {
        if *ch == b'z' {
            *ch = b'a';
        } else {
            *ch += 1;
            return;
        }
    }
}

// Passwords must include one increasing straight of at least three letters, like `abc`.
fn has_straight(password: &[u8; 8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

// Passwords must contain at least two different, non-overlapping pairs of letters, like `aa`, `bb`, or `zz`.
fn has_two_pairs(password: &[u8; 8]) -> bool {
    let mut pairs = 0;
    let mut i = 0;
    while i < 7 {
        if password[i] == password[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    pairs >= 2
}

fn find_next_password(start: &[u8; 8]) -> [u8; 8] {
    let mut password = *start;
    loop {
        inc_password(&mut password);

        // Rule 2: No 'i', 'o', or 'l'.
        // If found, we can skip many passwords by incrementing the forbidden letter
        // and resetting the rest.
        if let Some(pos) = password
            .iter()
            .position(|&c| matches!(c, b'i' | b'o' | b'l'))
        {
            password[pos] += 1;
            ((pos + 1)..8).for_each(|j| {
                password[j] = b'a';
            });
            continue; // Re-validate the new jumped-to password
        }

        // Rules 1 and 3
        if has_straight(&password) && has_two_pairs(&password) {
            return password;
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let next = find_next_password(INPUT);
    let ret = String::from_utf8_lossy(&next);
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let first = find_next_password(INPUT);
    let second = find_next_password(&first);
    let ret = String::from_utf8_lossy(&second);
    format_result!(ret)
}
