use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

const INPUT: usize = 3310_0000;

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    const MAX_HOUSES: usize = 1_000_000;
    let mut houses = vec![0; MAX_HOUSES];

    for elf in 1..MAX_HOUSES {
        for house in (elf..MAX_HOUSES).step_by(elf) {
            houses[house] += elf * 10;
        }
    }

    let house_number = houses.iter().position(|&gifts| gifts >= INPUT).unwrap_or(0);

    format_result!(house_number)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    const MAX_HOUSES: usize = 1_000_000;
    let mut houses = vec![0; MAX_HOUSES];

    for elf in 1..MAX_HOUSES {
        let stop = MAX_HOUSES.min(elf * 50 + 1);
        for house in (elf..stop).step_by(elf) {
            houses[house] += elf * 11;
        }
    }

    let house_number = houses.iter().position(|&gifts| gifts >= INPUT).unwrap_or(0);

    format_result!(house_number)
}
