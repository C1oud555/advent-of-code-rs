use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: &[u8; 8] = b"yzbqklnj";
const T_CNT: usize = 8;
const THRESHOLD: usize = 1_0000_0000;

fn check_zeros(digest: &[u8], zeros: usize) -> bool {
    let bytes_to_check = zeros / 2;
    for &byte in digest.iter().take(bytes_to_check) {
        if byte != 0 {
            return false;
        }
    }

    if zeros % 2 == 1 && digest[bytes_to_check] & 0xF0 != 0 {
        return false;
    }

    true
}

fn find_hash(zeros: usize) -> usize {
    let result = AtomicUsize::new(usize::MAX);
    let index = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..T_CNT {
            s.spawn(|| {
                let mut res = index.fetch_add(1, Ordering::Relaxed);

                let mut input_buf = Vec::with_capacity(32);
                while res < THRESHOLD {
                    let current_min = result.load(Ordering::Relaxed);
                    if res >= current_min {
                        break;
                    }

                    input_buf.clear();
                    input_buf.extend_from_slice(INPUT);
                    write!(&mut input_buf, "{}", res).unwrap();

                    let digest = md5::compute(&input_buf);
                    if check_zeros(digest.as_ref(), zeros) {
                        result.fetch_min(res, Ordering::Relaxed);
                        break;
                    }
                    res += T_CNT;
                }
            });
        }
    });

    result.load(Ordering::Relaxed)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(find_hash(5));
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(find_hash(6));
}
