mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod template;

use linkme::distributed_slice;

use rayon::prelude::*;

#[distributed_slice]
pub static PUZZLES: [fn() -> String];

#[macro_export]
macro_rules! format_result {
    ($x:expr) => {
        let mod_name = module_path!().split("::").last().unwrap_or("");
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let func_name = type_name_of(f)
            .strip_suffix("::f")
            .unwrap_or("")
            .split("::")
            .last()
            .unwrap_or("unknown");
        let res = $x;
        return format!("Result of {mod_name:>5}::{func_name:<10} is {res}");
    };
}

fn main() {
    let mut results = PUZZLES
        .par_iter()
        .map(|puzzle| puzzle())
        .collect::<Vec<_>>();

    results.sort();
    for res in results {
        println!("{}", res);
    }
}
