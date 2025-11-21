use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day7.txt");

trait Circuit {
    fn is_valid(&self) -> bool;

    fn get_output(&mut self) -> u16;
}

enum Component {
    Wire {
        valid: bool,
        output: u16,
        from: Box<Component>,
    },
    Not {
        valid: bool,
        output: u16,
        from: Box<Component>,
    },
    And {
        valid: bool,
        output: u16,
        from0: Box<Component>,
        from1: Box<Component>,
    },
    Or {
        valid: bool,
        output: u16,
        from0: Box<Component>,
        from1: Box<Component>,
    },
    Rshift {
        valid: bool,
        output: u16,
        from0: Box<Component>,
        from1: Box<Component>,
    },
    Lshift {
        valid: bool,
        output: u16,
        from0: Box<Component>,
        from1: Box<Component>,
    },
    Consts(u16),
}

impl Circuit for Component {
    fn is_valid(&self) -> bool {
        match self {
            Component::Wire { valid, .. } => *valid,
            Component::Not { valid, .. } => *valid,
            Component::And { valid, .. } => *valid,
            Component::Or { valid, .. } => *valid,
            Component::Rshift { valid, .. } => *valid,
            Component::Lshift { valid, .. } => *valid,
            Component::Consts(_) => true,
        }
    }

    fn get_output(&mut self) -> u16 {
        match self {
            Component::Wire {
                valid,
                output,
                from,
            } => {
                if *valid {
                    *output
                } else {
                    *output = from.get_output();
                    *valid = true;
                    *output
                }
            }
            Component::Not {
                valid,
                output,
                from,
            } => {
                if *valid {
                    *output
                } else {
                    *output = !from.get_output();
                    *valid = true;
                    *output
                }
            }
            Component::And {
                valid,
                output,
                from0,
                from1,
            } => {
                if *valid {
                    *output
                } else {
                    let input0 = from0.get_output();
                    let input1 = from1.get_output();
                    *output = input0 & input1;
                    *valid = true;
                    *output
                }
            }
            Component::Or {
                valid,
                output,
                from0,
                from1,
            } => {
                if *valid {
                    *output
                } else {
                    let input0 = from0.get_output();
                    let input1 = from1.get_output();
                    *output = input0 | input1;
                    *valid = true;
                    *output
                }
            }
            Component::Rshift {
                valid,
                output,
                from0,
                from1,
            } => {
                if *valid {
                    *output
                } else {
                    let input0 = from0.get_output();
                    let input1 = from1.get_output();
                    *output = input0 >> input1;
                    *valid = true;
                    *output
                }
            }
            Component::Lshift {
                valid,
                output,
                from0,
                from1,
            } => {
                if *valid {
                    *output
                } else {
                    let input0 = from0.get_output();
                    let input1 = from1.get_output();
                    *output = input0 << input1;
                    *valid = true;
                    *output
                }
            }
            Component::Consts(val) => *val,
        }
    }
}

fn parse_line(line: &str, components: &mut HashMap<String, Component>) {
    let segs: Vec<&str> = line.split("->").collect();
    // input
    let inputs: Vec<&str> = segs[0].trim().split_whitespace().collect();
    let input_name = inputs[0];
    // output
    let output_name = segs[1].trim();
}

fn construct_whole_circuit() -> HashMap<String, Component> {
    todo!()
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let mut circuits = construct_whole_circuit();

    let output = circuits
        .get_mut("a")
        .expect("not found circuit a")
        .get_output();

    format_result!(output);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    format_result!("template 1");
}
