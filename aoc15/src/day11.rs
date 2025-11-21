use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &[u8; 8] = b"cqjxjnds";

fn inc_password(password: &mut [u8; 8]) {
    let mut inc = true;

    for ch in password.iter_mut().rev() {
        if inc {
            if *ch == b'z' {
                *ch = b'a';
                inc = true;
            } else {
                *ch += 1;
                inc = false;
            }
        } else {
            break;
        }
    }
}
fn check_passwd(password: &[u8; 8]) -> bool {
    // 1
    let has_inc = password
        .windows(3)
        .any(|w| w[0] == w[1] - 1 && w[0] == w[2] - 2);

    // 2
    let has_special = password.iter().any(|ch| matches!(ch, b'i' | b'o' | b'l'));

    // 3
    let has_2_non_overlapping_pair = password
        .iter()
        .fold((0, b'0'), |(cnt, lst_ch), &ch| {
            if ch == lst_ch {
                (cnt + 1, b'0')
            } else {
                (cnt, ch)
            }
        })
        .0
        >= 2;

    has_inc && !has_special && has_2_non_overlapping_pair
}

fn next_password(cur: &[u8; 8]) -> [u8; 8] {
    let mut password = *cur;
    while !check_passwd(&password) {
        inc_password(&mut password);
    }

    password
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let cur = INPUT;
    let tmp = next_password(cur);
    let ret = String::from_utf8_lossy(&tmp);
    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let cur = INPUT;
    let mut tmp0 = next_password(cur);
    inc_password(&mut tmp0);
    let tmp1 = next_password(&tmp0);
    let ret = String::from_utf8_lossy(&tmp1);
    format_result!(ret);
}
