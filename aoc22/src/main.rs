mod day1;

use linkme::distributed_slice;

use rayon::prelude::*;

#[distributed_slice]
pub static PUZZLES: [fn() -> String];

#[macro_export]
macro_rules! format_result {
    ($x:expr) => {{
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
        format!("Result of {mod_name:>5}::{func_name:<10} is {res}")
    }};
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
