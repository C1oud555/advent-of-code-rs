use crate::{PUZZLES, format_result};
use linkme::distributed_slice;

const INPUT: &str = include_str!("../inputs/day10.txt");

fn solve(input: &str, th: i32, display: bool) -> i32 {
    let mut cycle = 0;
    let mut regx = 1;
    let mut sum = 0;

    let mut screen = [['.'; 40]; 6];

    for line in input.lines() {
        let mut adden = 0;

        let (sx, sy): (i32, i32) = (cycle / 40, cycle % 40);
        if (regx - 1..=regx + 1).contains(&sy) {
            screen[sx as usize][sy as usize] = '#';
        }

        if line.len() == 4 {
            cycle += 1;
        } else {
            let (_, tmp) = line.split_once(' ').unwrap();
            let ladden = tmp.parse::<i32>().unwrap();
            if (cycle + 1 - 20) % 40 == 0 {
                sum += (cycle + 1) * regx;
            }
            let (sx, sy): (i32, i32) = ((cycle + 1) / 40, (cycle + 1) % 40);
            if (regx - 1..=regx + 1).contains(&sy) {
                screen[sx as usize][sy as usize] = '#';
            }

            adden = ladden;
            cycle += 2;
        }

        if (cycle - 20) % 40 == 0 {
            sum += cycle * regx;
        }

        regx += adden;
        if cycle > th {
            break;
        }
    }

    if display {
        for line in screen {
            for ch in line {
                print!("{}", ch);
            }
            println!();
        }
    }

    sum
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = solve(INPUT, 220, false);
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = solve(INPUT, 240, true);
    format_result!(ret)
}
