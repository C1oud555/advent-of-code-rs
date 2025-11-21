use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use serde_json::Value;

const INPUT: &str = include_str!("../inputs/day12.txt");

fn get_num_sum(value: &Value) -> isize {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().expect("Wrong number") as isize,
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(get_num_sum).sum::<isize>(),
        Value::Object(map) => map.iter().map(|(_, v)| get_num_sum(v)).sum::<isize>(),
    }
}
fn get_num_sum1(value: &Value) -> isize {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().expect("Wrong number") as isize,
        Value::String(_) => 0,
        Value::Array(values) => values.iter().map(get_num_sum1).sum::<isize>(),
        Value::Object(map) => {
            if map.iter().any(|(_, v)| {
                if let Value::String(v) = v
                    && v == "red"
                {
                    true
                } else {
                    false
                }
            }) {
                0
            } else {
                map.iter().map(|(_, v)| get_num_sum1(v)).sum::<isize>()
            }
        }
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let values: Value = serde_json::from_str(INPUT).expect("Wrong json format");

    let ret = get_num_sum(&values);

    format_result!(ret);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let values: Value = serde_json::from_str(INPUT).expect("Wrong json format");

    let ret = get_num_sum1(&values);

    format_result!(ret);
}
