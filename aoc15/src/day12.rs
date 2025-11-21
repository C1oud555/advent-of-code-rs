use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use serde_json::Value;

const INPUT: &str = include_str!("../inputs/day12.txt");

fn sum_numbers(value: &Value, ignore_red: bool) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, ignore_red)).sum(),
        Value::Object(map) => {
            if ignore_red && map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(|v| sum_numbers(v, ignore_red)).sum()
            }
        }
        _ => 0, // Null, Bool, String
    }
}

fn solve(ignore_red: bool) -> i64 {
    let values: Value = serde_json::from_str(INPUT).expect("Wrong json format");
    sum_numbers(&values, ignore_red)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    format_result!(solve(false));
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!(solve(true));
}
